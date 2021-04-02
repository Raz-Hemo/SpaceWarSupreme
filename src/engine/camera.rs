use nalgebra::{Matrix4, Point3, Vector3, geometry::UnitQuaternion};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    quat: UnitQuaternion<f32>,
    pos: Point3<f32>,
}

impl Camera {
    pub fn new(pos: Point3<f32>, lookat: Point3<f32>, up: Vector3<f32>) -> Camera {
        Camera {
            quat: UnitQuaternion::face_towards(&((lookat - pos).normalize()), &up.normalize()),
            pos,
        }
    }
}

impl crate::engine::scripting::interpolate::Interpolate for Camera {
    fn get(&self, other: &Camera, alpha: f32) -> Camera {
        Camera {
            quat: self.quat.try_slerp(&other.quat, alpha, 0.0001).unwrap_or(
                other.quat
            ),
            pos: self.pos + (other.pos - self.pos) * alpha,
        }
    }
}

impl crate::engine::graphics::Camera for Camera {
    fn get_view(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(
            &self.pos,
            &(self.pos + self.quat.transform_vector(&Vector3::z_axis())),
            &self.quat.transform_vector(&Vector3::y_axis()),
        )
    }

    fn get_view_at_origin(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(
            &Point3::new(0.0, 0.0, 0.0),
            &(Point3::new(0.0, 0.0, 0.0) + self.quat.transform_vector(&Vector3::z_axis())),
            &self.quat.transform_vector(&Vector3::y_axis()),
        )
    }

    fn get_world_position(&self) -> nalgebra::Point3<f32> {
        self.pos
    }
}
