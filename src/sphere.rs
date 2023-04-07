use glam::Vec3;

use crate::{hitable::{Hitable, HitRecord}, ray::Ray, material::Material};

pub struct Sphere{
    pub pos: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere{
    pub fn new(pos: Vec3, radius: f32, material: Material) -> Self{
        Self { pos, radius, material}
    }
}

impl Hitable for Sphere{
    fn hit(&self, ray: &Ray, _near: f32, _far:f32)->Option<HitRecord> {
        let oc = ray.origin - self.pos;
        let a = ray.dir.dot(ray.dir);
        let b = 2.0 * oc.dot(ray.dir);
        let c = oc.dot(oc) - self.radius.powi(2);
        let discriminant = b * b - 4.0 * a * c;
        if discriminant >= 0.0{
            let dst = (-b - discriminant.sqrt())/(2.0*a);
            if dst >= 0.0{
                let hit_point = ray.origin + ray.dir * dst;
                return Some(HitRecord {
                    dst,
                    hit_point,
                    normal: (hit_point - self.pos).normalize(),
                    material: &self.material
                })
            }
        } 
        return None
    }
}