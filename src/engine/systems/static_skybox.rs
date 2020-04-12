use specs::ReadStorage;
use crate::engine::components::StaticSkyboxComponent;

pub struct StaticSkyboxSystem {
    skybox: Option<String>,
    last_multi_skybox_warning: Option<std::time::Instant>,
}

impl StaticSkyboxSystem {
    pub fn new() -> StaticSkyboxSystem {
        StaticSkyboxSystem {
            skybox: None,
            last_multi_skybox_warning: None,
        }
    }

    pub fn get_and_flush(&mut self) -> Option<String> {
        std::mem::replace(&mut self.skybox, None)
    }
}

impl<'a> specs::System<'a> for StaticSkyboxSystem {
    type SystemData = ReadStorage<'a, StaticSkyboxComponent>;

    fn run(&mut self, skyboxes: Self::SystemData) {
        use specs::Join;

        for skybox in skyboxes.join() {
            if !skybox.visible {
                continue;
            }

            if self.skybox.is_some() {
                if self.last_multi_skybox_warning.is_none() ||
                self.last_multi_skybox_warning.unwrap().elapsed().as_secs_f32() >
                crate::consts::MULTI_SKYBOX_WARNING_INTERVAL_SECONDS {
                    crate::log::warning("Multiple skyboxes are visible. This hurts performance.");
                    self.last_multi_skybox_warning = Some(std::time::Instant::now());
                }
                break;
            }

            self.skybox = Some(skybox.skybox.clone());
        }
    }
}