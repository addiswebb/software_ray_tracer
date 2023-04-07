use glam::Vec3;

pub struct Ray{
    pub dir: Vec3,
    pub origin: Vec3,
}

impl Ray{
    pub fn new(origin: Vec3, dir: Vec3) -> Self{
        Self{
            dir,
            origin 
        }
    }
}