pub struct Camera {
    pos: cgmath::Vector3<f32>,
    pitch: f32,
    yaw: f32,
    roll: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            pos: cgmath::Vector3::new(0.0, 0.0, 0.0),
            pitch: 0.0,
            yaw: 0.0,
            roll: 0.0
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new()
    }
}
