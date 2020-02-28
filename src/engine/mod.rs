use std::sync::mpsc::channel;

pub struct Engine<'a> {
    pub input: crate::input::InputInfo<'a>,
    pub cfg: crate::config::Config,
    audio_sender: crate::audio::SoundEventQueueTx,
    audio_assets: crate::audio::ArcSoundBank,
    audio_thread: Option<std::thread::JoinHandle<()>>,
}

impl<'a> Engine<'a> {
    pub fn new() -> Engine<'a> {
        let (audio_sender, audio_receiver) = channel();
        let audio_assets = Engine::load_sounds();
        let audio_thread = std::thread::Builder::new().name(String::from("spacewar_audio"))
        .spawn(|| {
            crate::audio::audio_worker_thread(audio_receiver);
        });

        if audio_thread.is_err() {
            crate::log::error(&format!("Failed to create audio thread: {:?}", audio_thread));
        }

        Engine {
            input: crate::input::InputInfo::new(),
            cfg: crate::config::Config::load(),
            audio_sender: audio_sender,
            audio_assets: audio_assets,
            audio_thread: audio_thread.ok(),
        }
    }

    fn load_sounds() -> crate::audio::ArcSoundBank {
        let mut result = crate::audio::new_soundbank();

        for f in crate::utils::get_files_with_extension_from(
                crate::consts::SOUND_FOLDER_PATH, Vec::from(crate::consts::SUPPORTED_SOUND_EXTENSIONS)) {
            if let Some(name) = f.file_stem() {
                let filename = &String::from(f.to_string_lossy());
                match crate::audio::Sound::new(filename) {
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

    pub fn play_sound(&self, id: &str) {
        if self.audio_assets.contains_key(id) {
            if let Err(e) = self.audio_sender.send(
                crate::audio::SoundEvent::Play(self.audio_assets.clone(), String::from(id))
            ) {
                crate::log::warning(&format!("Failed to send sound to worker thread: {}", e));
            }
        } else {
            crate::log::error(&format!("No such sound {}", id));
        }
    }

    pub fn acquire_audio_device(&self) {
        if let Err(e) = self.audio_sender.send(crate::audio::SoundEvent::AcquireDevice) {
            crate::log::error(&format!("Send acquire message to audio thread failed: {}", e))
        }
    }

    pub fn destroy_audio_device(&self) {
        if let Err(e) = self.audio_sender.send(crate::audio::SoundEvent::DestroyDevice) {
            crate::log::error(&format!("Send destroy message to audio thread failed: {}", e))
        }
    }
}
