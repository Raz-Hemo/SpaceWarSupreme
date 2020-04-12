pub struct StaticSkyboxComponent {
    pub skybox: String,
    pub visible: bool,
}

impl StaticSkyboxComponent {
    pub fn new(skybox: &str) -> StaticSkyboxComponent {
        StaticSkyboxComponent {
            skybox: String::from(skybox),
            visible: true,
        }
    }
}

impl specs::Component for StaticSkyboxComponent {
    type Storage = specs::HashMapStorage<Self>;
}
