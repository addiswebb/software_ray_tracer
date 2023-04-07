use glam::Vec3;

use crate::{hitable::{Hitable, HitRecord}, ray::Ray};

#[derive(Default)]
pub struct Scene{
    entities: Vec<Box<dyn Hitable>>,
    camera_pos: Vec3,
    camera_target: Vec3,
}

impl Scene{
    pub fn push(&mut self, hitable: impl Hitable + 'static){
        self.entities.push(Box::new(hitable))
    }
    pub fn init_camera(&mut self, pos:Vec3, target:Vec3){
        self.camera_pos=pos;
        self.camera_target=target;
    }
}

impl Hitable for Scene{
    fn hit(&self, ray: &Ray, near: f32, far: f32)->Option<HitRecord> {
        let mut closest_so_far = far;
        let mut closest_hit: Option<HitRecord> = None;
        for e in self.entities.iter(){
            if let Some(h) = e.hit(ray,near,closest_so_far){
                if h.dst < closest_so_far{
                    closest_so_far = h.dst;
                    closest_hit = Some(h);
                }
            }
        }
        closest_hit 
    }
}