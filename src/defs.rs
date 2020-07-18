use cgmath::*;

pub type Vec3f = Vector3<f32>;
pub type Vec4f = Vector4<f32>;
pub type Vec2f = Vector2<f32>;
pub type Vec2i = Vector2<i32>;
pub type Vec3i = Vector3<i32>;
pub type Mat4 = Matrix4<f32>;

pub fn wrap_angle(theta: f32) -> f32 {
    let zero_to_2pi = theta % (2.0 * std::f32::consts::PI);
    if zero_to_2pi > std::f32::consts::PI {
        return zero_to_2pi - (2.0 * std::f32::consts::PI);
    }
    zero_to_2pi
}
