use std::sync::mpsc::channel;

enum SoundEvent {
    Play(ArcSoundBank, SoundID),
    AcquireDevice,
    DestroyDevice,
}

type SoundID = String;
type SoundEventQueueRx = std::sync::mpsc::Receiver<SoundEvent>;
type SoundEventQueueTx = std::sync::mpsc::Sender<SoundEvent>;
type SoundBank = std::collections::HashMap<SoundID, Sound>;
type ArcSoundBank = std::sync::Arc<SoundBank>;

pub struct AudioManager {
    sender: SoundEventQueueTx,
    assets: ArcSoundBank,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl AudioManager {
    pub fn new() -> AudioManager {
        let (sender, receiver) = channel();
        let thread = std::thread::Builder::new().name(String::from("spacewar_audio"))
        .spawn(|| {
            audio_worker_thread(receiver);
        });
        if thread.is_err() {
            crate::log::error(&format!("Failed to create audio thread: {:?}", thread));
        }

        AudioManager {
            sender,
            thread: thread.ok(),
            assets: load_sounds(),
        }
    }

    pub fn play_sound(&self, id: &str) {
        if self.assets.contains_key(id) {
            if let Err(e) = self.sender.send(
                SoundEvent::Play(self.assets.clone(), String::from(id))
            ) {
                crate::log::warning(&format!("Failed to send sound to worker thread: {}", e));
            }
        } else {
            crate::log::error(&format!("No such sound {}", id));
        }
    }

    pub fn acquire_audio_device(&self) {
        if let Err(e) = self.sender.send(SoundEvent::AcquireDevice) {
            crate::log::error(&format!("Send acquire message to audio thread failed: {}", e))
        }
    }

    pub fn destroy_audio_device(&self) {
        if let Err(e) = self.sender.send(SoundEvent::DestroyDevice) {
            crate::log::error(&format!("Send destroy message to audio thread failed: {}", e))
        }
    }
}

pub fn audio_worker_thread(queue: SoundEventQueueRx) {
    // For convert_samples()
    use rodio::source::Source;
    let mut device = try_acquire_audio_device();

    loop {
        match queue.recv() {
            Ok(SoundEvent::Play(assets, id)) => {
                if assets.contains_key(&id) && device.is_some() { 
                    rodio::play_raw(device.as_ref().unwrap(), assets[&id].decoder().convert_samples()) 
                }
            },
            Ok(SoundEvent::AcquireDevice) => {
                device = try_acquire_audio_device();
            },
            Ok(SoundEvent::DestroyDevice) => {
                device = None
            },
            Err(_) => break,
        }
    }
}

use std::io;

pub struct Sound {
    samples: std::sync::Arc<Vec<u8>>,
}

impl std::convert::AsRef<[u8]> for Sound {
    fn as_ref(&self) -> &[u8] {
        &self.samples
    }
}

impl Sound {
    pub fn new(filename: &str) -> crate::utils::SWSResult<Sound> {
        use std::io::Read;
        use std::fs::File;

        let file = File::open(filename);
        if file.is_err() {
            return Err(format!("Failed to open sound file {}", filename));
        }
        let mut file = file.unwrap();

        let mut buf = Vec::new();
        if file.read_to_end(&mut buf).is_err() {
            return Err(format!("Failed to read sound file {}", filename));
        }

        Ok(Sound { samples: std::sync::Arc::new(buf) })
    }
    pub fn decoder(self: &Self) -> rodio::Decoder<io::Cursor<Sound>> {
        rodio::Decoder::new(io::Cursor::new(
            Sound { samples: self.samples.clone() }
        )).unwrap()
    }
}

fn load_sounds() -> ArcSoundBank {
    let mut result = std::collections::HashMap::new();

    for f in crate::utils::get_files_with_extension_from(
            crate::consts::SOUND_FOLDER_PATH, Vec::from(crate::consts::SUPPORTED_SOUND_EXTENSIONS)) {
        if let Some(name) = f.file_stem() {
            let filename = &String::from(f.to_string_lossy());
            match Sound::new(filename) {
                Err(e) => crate::log::error(&e),
                Ok(s) => { result.insert(
                    String::from(name.to_string_lossy()), 
                    s);
                },
            }
        }
    }

    std::sync::Arc::new(result)
}

fn try_acquire_audio_device() -> Option<rodio::Device>{
    if let Some(device) = rodio::default_output_device() {
        Some(device)
    } else {
        crate::log::error("Could not find a sound device for output!");
        None
    }
}
