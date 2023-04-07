use core::num;

use glam::{Vec3, Vec2};
use rand::Rng;

use crate::{buffer::Buffer, camera::Camera, ray::Ray, hitable::{HitRecord, Hitable}, sphere::Sphere, scene::Scene};
pub struct RenderSettings{
    pub samples: usize,
    pub bounces: usize,
    pub rays_per_pixel: usize,
}
pub struct Renderer{
    camera: Camera,
    image_buffer: Buffer,
    settings: RenderSettings,
}
pub struct FragInput<'a>{
    pos: Vec2,
    i: usize,
    scene: &'a Scene,
}

impl Renderer{
    pub fn new(width: usize, height: usize) -> Self{

/*depth         let origin = Vec3::new(0.0,0.0,0.0);
        let look_at = Vec3::new(0.0,0.0,1.0); */
/*light        let origin = Vec3::new(3.089,1.53,-3.0);
        let look_at =Vec3::new(-2.0,-1.0,2.0); */
        let origin = Vec3::new(0.0,0.0,0.0);
        let look_at = Vec3::new(1.0,0.0,0.0);
        let up = Vec3::new(0.0,1.0,0.0);
        let fov = 45.0;
        let aspect = width as f32/height as f32; 
        let near_clipping_plane = 0.1;
        let far_clipping_plane = 100.0;

        let camera = Camera::new(origin,look_at,up,fov,aspect,near_clipping_plane, far_clipping_plane);
        let mut image_buffer = Buffer::new(width, height);

        image_buffer.clear();

        let settings = RenderSettings{
            samples: 1,
            bounces: 1,
            rays_per_pixel: 1,
        };

        Self{
            camera,
            image_buffer,
            settings,
        }
    }
    pub fn render_settings(&mut self, settings: RenderSettings) {
        self.settings = settings;
    }
    pub fn render(&mut self, scene: &Scene){
        let mut rng = rand::thread_rng();
        for y in (0..self.image_buffer.height()).rev(){
            Renderer::print_progress(y, self.image_buffer.height());
            for x in 0..self.image_buffer.width(){
                let mut color = Vec3::new(0.0,0.0,0.0);
                let i = x + y * self.image_buffer.height();
                for _ in 0..self.settings.samples{
                    let offset = if self.settings.samples > 1 {Vec2::new(rng.gen::<f32>(), rng.gen::<f32>())} else {Vec2::new(0.0,0.0)};
                    let pos = Vec2::new(
                        (x as f32 + offset.x)/self.image_buffer.width() as f32,
                        (y as f32 + offset.y)/self.image_buffer.height() as f32
                    );
                    let frag_input = FragInput{
                        pos,
                        i,
                        scene,
                    };
                    color += self.frag(&frag_input); 
                }
                color /= self.settings.samples as f32;
                self.image_buffer.image[i] = color;
            }
        }
    }
   /*  pub fn frag1(&self, i: &FragInput) -> Vec3 {
        let origin = self.camera.origin;
        let dir = self.camera.lower_left_corner + i.pos.x * self.camera.horizontal + i.pos.y *self.camera.vertical - origin;
        let ray = Ray::new(origin,dir);
        if let Some(hit) = i.scene.hit(&ray, self.camera.near, self.camera.far){
            hit.color
        }
        else{
            let t = 0.5 * (ray.dir.normalize().y + 1.0);
            (1.0-t) * Vec3::new(1.0,1.0,1.0)+t*Vec3::new(0.5,0.7,1.0)
        }
    } */
    pub fn frag(&self, i: &FragInput) -> Vec3{
        let origin = self.camera.origin;
        let dir = (self.camera.lower_left_corner + i.pos.x * self.camera.horizontal + i.pos.y *self.camera.vertical - origin).normalize();
        let mut ray = Ray::new(origin,dir);
        let mut total_incoming_light = Vec3::ZERO;
        for _ in 0..self.settings.rays_per_pixel{
            total_incoming_light += self.trace(&mut ray,i) ;
        }
        return total_incoming_light;
    }
    pub fn trace(&self, ray: &mut Ray, i: &FragInput)-> Vec3{
        let mut color = Vec3::ONE;
        let mut incoming_light = Vec3::ZERO;
        for _ in 0..self.settings.bounces+1{
            if let Some(hit) = i.scene.hit(&ray, self.camera.near, self.camera.far){
                ray.origin = hit.hit_point;
                ray.dir = hit.normal + Renderer::rand_unit_sphere();
                let emitted_light = hit.material.emission_color * hit.material.emission_strength;
                incoming_light += emitted_light * color;
                color *= hit.material.color;
            }else{
                break;
            }
        }
        return incoming_light;
    }
    pub fn rand_unit_sphere()->Vec3{
        let mut rng = rand::thread_rng();
        let unit = Vec3::new(1.0, 1.0, 1.0);
        for _ in 0..100{
            let p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - unit;
            if p.length().sqrt() < 1.0{
                return p;
            }
        }
        eprint!("Error: Failed to generate random direction in sphere");
        Vec3::new(0.0,0.0,0.0)
    }
    pub fn rand_hemisphere_direction(normal: Vec3)->Vec3{
        let dir = Renderer::random_direction();
        dir * normal.dot(dir).signum()
    }
    pub fn rand_distribution(x: f32)-> f32{
        return x;
        let theta = 2.0 * 3.1415926 * x;
        let rho = (-2.0*x.log10()).sqrt();
        rho * theta.cos()
    }
    pub fn random_direction() -> Vec3{
        let mut rng = rand::thread_rng();
        for _ in 0..100{
            let x = rng.gen::<f32>();
            let y = rng.gen::<f32>();
            let z = rng.gen::<f32>();
            let point_in_cube= Vec3::new(Renderer::rand_distribution(x),Renderer::rand_distribution(y),Renderer::rand_distribution(z));
            let sqr_dst_from_centre = point_in_cube.dot(point_in_cube);
            if sqr_dst_from_centre <= 1.0{
                return point_in_cube / sqr_dst_from_centre.sqrt();
            }
        }
        eprint!("Error: Failed to generate random direction in hemisphere");
        Vec3::new(0.0,0.0,0.0)
    }
    
    pub fn save_render(&self, path: &str){
        self.image_buffer.to_file(path);
    }
    pub fn print_progress(at: usize, out_of: usize){
        print!("\x1B[2J\x1B[1;1H");
        println!("Progress: {:.2}%",100.0-100.0*(at as f32/ out_of as f32));
    }
}