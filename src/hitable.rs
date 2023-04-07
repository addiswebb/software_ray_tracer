use glam::Vec3;

use crate::{ray::Ray, material::Material};

pub struct HitRecord<'a>{
    pub dst: f32,
    pub hit_point: Vec3,
    pub normal: Vec3, 
    pub material: &'a Material,
}

pub trait Hitable{
    fn hit(&self, ray: &Ray, near: f32, far:f32)->Option<HitRecord>;
}