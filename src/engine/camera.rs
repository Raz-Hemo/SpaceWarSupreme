use nalgebra::{Matrix4, Point3, Vector3, geometry::UnitQuaternion};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    quat: UnitQuaternion<f32>,
    pos: Point3<f32>,
}

impl Camera {
    pub fn new(pos: Point3<f32>, lookat: Point3<f32>, up: Vector3<f32>) -> Camera {
        Camera {
            quat: UnitQuaternion::face_towards(&(lookat - pos), &up),
            pos,
        }
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(
            &self.pos,
            &(self.pos + self.quat.transform_vector(&Vector3::z_axis())),
            &self.quat.transform_vector(&Vector3::y_axis()),
        )
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

impl crate::scripting::interpolate::Interpolate for Camera {
    fn get(&self, other: &Camera, alpha: f32) -> Camera {
        let result = Camera {
            quat: self.quat.try_slerp(&other.quat, alpha, 0.0001).unwrap_or(
                other.quat
            ),
            pos: self.pos + (other.pos - self.pos) * alpha,
        };
        //println!("{:?}-<>-{:?}=={:?}", self, other, result);
        result
    }
}