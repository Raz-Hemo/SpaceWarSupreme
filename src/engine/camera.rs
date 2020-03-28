use cgmath::{Matrix4, Point3, Vector3, perspective, Deg};

pub struct Camera {
    pub pos: Point3<f32>,
    pub look_at: Point3<f32>,
    projection: Matrix4<f32>
}

impl Camera {
    pub fn new(resolution_x: u32, resolution_y: u32, fovy_deg: f32) -> Camera {
        let aspect = resolution_x as f32 / resolution_y as f32;
        Camera {
            pos: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, 1.0),
            projection: perspective(Deg(fovy_deg), aspect, 0.01, 10000.0),
        }
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at(self.pos, self.look_at, Vector3::new(0.0, -1.0, 0.0))
    }

    pub fn get_projection_matrix(&self) -> Matrix4<f32> {
        self.projection
    }
}
