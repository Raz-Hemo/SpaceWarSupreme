pub type SoundID = u32;
pub type SoundEventQueue = std::sync::mpsc::Receiver<SoundID>;

pub fn audio_worker_thread(queue: SoundEventQueue) {
    if let Some(device) = rodio::default_output_device() {
        loop {
            match queue.recv() {
                Ok(id) => rodio::play_raw(&device, some_way_to_get_source_from_id(id)),
                Err(e) => break,
            }
        }
    } else {
        crate::log::error("Could not find a sound device for output!");
    }
}