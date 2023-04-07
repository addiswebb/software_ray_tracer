use glam::{Vec3, Mat4, Quat};

pub struct Camera{
    pub origin: Vec3,
    pub lower_left_corner:Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub near: f32,
    pub far: f32,
    u: Vec3,
    v: Vec3,
}
impl Camera{
    pub fn new(origin: Vec3, look_at: Vec3, view_up: Vec3, fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let theta = fov * std::f32::consts::PI / 180.0;
        let half_height = near * f32::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let w = (origin- look_at).normalize();
        let u = view_up.cross(w).normalize();
        let v = w.cross(u);
        Camera {
            origin,
            lower_left_corner: origin - half_width * u - half_height * v - near * w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            near, far,
            u, v,
        }
    }
}