pub enum SoundEvent {
    Play(ArcSoundBank, SoundID),
    AcquireDevice,
    DestroyDevice,
}

pub type SoundID = String;
pub type SoundEventQueueRx = std::sync::mpsc::Receiver<SoundEvent>;
pub type SoundEventQueueTx = std::sync::mpsc::Sender<SoundEvent>;
pub type SoundBank = std::collections::HashMap<SoundID, Sound>;
pub type ArcSoundBank = std::sync::Arc<SoundBank>;

pub fn new_soundbank() -> SoundBank {
    std::collections::HashMap::new()
}

fn try_acquire_audio_device() -> Option<rodio::Device>{
    if let Some(device) = rodio::default_output_device() {
        Some(device)
    } else {
        crate::log::error("Could not find a sound device for output!");
        None
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