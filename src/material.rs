use glam::Vec3;

pub struct Material{
    pub color: Vec3,
    pub emission_color: Vec3,
    pub emission_strength: f32,
}

impl Material{
    pub fn color(&self) -> Vec3{
        Vec3::new(1.0,0.0,0.0)
    }
}
