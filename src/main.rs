use glam::Vec3;
use material::Material;
use renderer::{Renderer, RenderSettings};
use scene::Scene;
use sphere::Sphere;

mod renderer;
mod buffer;
mod camera;
mod ray;
mod hitable;
mod sphere;
mod scene;
mod material;

fn main() {
    let mut renderer = Renderer::new(1000,1000);
    renderer.render_settings(RenderSettings{
        samples: 10,
        bounces: 30,
        rays_per_pixel: 100,
    });
    let scene = room_scene();

    use std::time::Instant;
    let now = Instant::now();

    renderer.render(&scene);

    println!("Rendered scene in {:.2?}", now.elapsed());
    renderer.save_render("output/test2.ppm");
}

fn depth_test_scene() -> Scene{
    let mut scene = Scene::default();
    scene.init_camera(Vec3::ZERO,Vec3::new(0.0,0.0,1.0));
    scene.push(Sphere::new(Vec3::new(1.66,-0.25,7.83),1.0,Material{
        color:Vec3::new(1.0,0.0,0.0),
        emission_color: Vec3::ZERO,
        emission_strength: 0.0,
    }));
    scene.push(Sphere::new(Vec3::new(0.33, -0.5,9.47),1.5,Material{
        color:Vec3::new(0.0,1.0,0.0),
        emission_color: Vec3::ZERO,
        emission_strength: 0.0,
    }));
    scene.push(Sphere::new(Vec3::new(-1.72,-0.72,11.45),1.0,Material{
        color:Vec3::new(0.0,0.0,1.0),
        emission_color: Vec3::ZERO,
        emission_strength: 0.0,
    }));

    scene
}

fn lighting_test_scene() -> Scene{
    let mut scene = Scene::default();
    scene.init_camera(Vec3::new(3.089,1.53,-3.0),Vec3::new(-2.0,-1.0,2.0));

/*  Main objects*/
    scene.push(Sphere::new(Vec3::new(-2.54,-0.72,0.5),0.6, Material {
        color: Vec3::new(1.0,0.0,0.0),
        emission_color: Vec3::new(0.0,0.0,0.0),
        emission_strength: 0.0,
    }));
    scene.push(Sphere::new(Vec3::new(-1.27,-0.72,1.0),0.5, Material {
        color: Vec3::new(0.0,1.0,0.0),
        emission_color: Vec3::new(0.0,0.0,0.0),
        emission_strength: 0.0,
    }));

    scene.push(Sphere::new(Vec3::new(-0.5,-0.9,1.55),0.35, Material {
        color: Vec3::new(0.0,0.0,1.0),
        emission_color: Vec3::new(0.0,0.0,0.0),
        emission_strength: 0.0,
    }));

/*  floor*/
    scene.push(Sphere::new(Vec3::new(-3.46,-15.88,2.76),15.0, Material {
        color: Vec3::new(0.5,0.0,0.8),
        emission_color: Vec3::new(1.0,1.0,1.0),
        emission_strength: 0.0,
    }));

/*  Light Object       */
    scene.push(Sphere::new(Vec3::new(-7.44,-0.72,20.0),15.0, Material {
        color: Vec3::new(1.0,1.0,1.0),
        emission_color: Vec3::new(1.0,1.0,1.0),
        emission_strength: 1.0,
    }));
    scene
}

fn room_scene() -> Scene{
    let mut scene = Scene::default();
    scene.push(Sphere::new(Vec3::new(5.54,-0.3,0.685),0.5,Material {
        color: Vec3::new(1.0,0.0,0.0),
        emission_color: Vec3::new(0.0,0.0,0.0),
        emission_strength: 0.0, 
    }));

    scene.push(Sphere::new(Vec3::new(5.11,-8.0,0.0),7.5,Material {
        color: Vec3::new(1.0,1.0,1.0),
        emission_color: Vec3::new(0.0,0.0,0.0),
        emission_strength: 0.0, 
    }));
    scene.push(Sphere::new(Vec3::new(15.09,0.65,0.0),7.5,Material {
        color: Vec3::new(1.0,1.0,1.0),
        emission_color: Vec3::new(0.0,0.0,0.0),
        emission_strength: 0.0, 
    }));
    scene.push(Sphere::new(Vec3::new(5.11,-0.73,-9.19),7.5,Material {
        color: Vec3::new(0.0,0.0,1.0),
        emission_color: Vec3::new(0.0,0.0,0.0),
        emission_strength: 0.0, 
    }));
    scene.push(Sphere::new(Vec3::new(5.11,-0.73,9.19),7.5,Material {
        color: Vec3::new(0.0,1.0,0.0),
        emission_color: Vec3::new(0.0,0.0,0.0),
        emission_strength: 0.0, 
    }));
    scene.push(Sphere::new(Vec3::new(5.11,8.19,0.0),7.5,Material {
        color: Vec3::new(1.0,1.0,0.0),
        emission_color: Vec3::new(0.0,0.0,0.0),
        emission_strength: 0.0, 
    }));

    scene.push(Sphere::new(Vec3::new(5.232,1.029,0.19),0.5,Material {
        color: Vec3::new(0.0,0.0,0.0),
        emission_color: Vec3::new(1.0,1.0,1.0),
        emission_strength: 2.0, 
    }));
    scene
}