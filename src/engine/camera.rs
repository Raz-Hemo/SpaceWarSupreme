use cgmath::{Matrix4, Point3, Vector3, perspective, Deg, InnerSpace, Rotation3, Rotation};

pub struct Camera {
    pub pos: Point3<f32>,
    pub look_at: Point3<f32>,
    fovy_deg: f32,
    aspect: f32,
    projection: Matrix4<f32>
}

impl Camera {
    pub fn new(resolution_x: u32, resolution_y: u32, fovy_deg: f32) -> Camera {
        let aspect = resolution_x as f32 / resolution_y as f32;
        Camera {
            pos: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, 1.0),
            fovy_deg,
            aspect,
            projection: perspective(Deg(fovy_deg), aspect, 0.01, 10000.0),
        }
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at(self.pos, self.look_at, Vector3::new(0.0, -1.0, 0.0))
    }

    pub fn get_projection_matrix(&self) -> Matrix4<f32> {
        self.projection
    }

    /// This is glitched by a slight offset. TBD when will actually need to use it
    pub fn get_raycast(&self, xalpha: f32, yalpha: f32) -> Vector3<f32> {
        let forward = (self.look_at - self.pos).normalize();
        let up = Vector3::new(0.0, -1.0, 0.0);
        let right = forward.cross(up);
        let up = right.cross(forward);

        let yaw = Deg((xalpha - 0.5) * self.aspect * self.fovy_deg);
        let pitch = Deg((yalpha - 0.5) * self.fovy_deg);

        cgmath::Quaternion::from_axis_angle(right, pitch).rotate_vector(
            cgmath::Quaternion::from_axis_angle(up, yaw).rotate_vector(
                forward
            )
        )
    }
}
