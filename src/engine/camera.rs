use cgmath::{Matrix4, Point3, Vector3};

#[derive(Clone, Copy)]
pub struct Camera {
    pub pos: Point3<f32>,
    pub look_at: Point3<f32>,
    pub up: Vector3<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            pos: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, 1.0),
            up: Vector3::new(0.0, 1.0, 0.0)
        }
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at(self.pos, self.look_at, self.up)
    }

    // This is glitched by a slight offset. TBD when will actually need to use it
    //pub fn get_raycast(&self, xalpha: f32, yalpha: f32) -> Vector3<f32> {
    //    let forward = (self.look_at - self.pos).normalize();
    //    let up = Vector3::new(0.0, -1.0, 0.0);
    //    let right = forward.cross(up);
    //    let up = right.cross(forward);

    //    let yaw = Deg((xalpha - 0.5) * self.aspect * self.fovy_deg);
    //    let pitch = Deg((yalpha - 0.5) * self.fovy_deg);

    //    cgmath::Quaternion::from_axis_angle(right, pitch).rotate_vector(
    //        cgmath::Quaternion::from_axis_angle(up, yaw).rotate_vector(
    //            forward
    //        )
    //    )
    //}
}
