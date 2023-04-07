use glam::Vec3;
use std::fs;
pub struct Buffer{
    pub size: [usize;2],
    pub image: Vec<Vec3>
}

impl Buffer{
    pub fn new(width: usize, height: usize) -> Self{
        let size = [width, height];
        let mut image = Vec::new();
        for _ in 0..(width*height){
            image.push(Vec3::new(0.0,0.0,0.0));
        }
        Self{
            size,
            image
        }
    }
    pub fn width(&self) -> usize{
        self.size[0]
    }
    pub fn height(&self) -> usize{
        self.size[1]
    }
    pub fn clear(&mut self){
        for x in self.image.iter_mut(){
            *x = Vec3::new(0.0,0.0,0.0);
        }
    }
    pub fn to_file(&self, path: &str){
        let mut file = String::new();
        file.push_str(&format!("P3\n{} {}\n255\n",self.width(),self.height()));
        for y in (0..self.height()).rev(){
            for x in (0..self.width()).rev(){
                let i = x + y * self.height();
                file.push_str(&format!("{} {} {}\n",
                (self.image[i].x * 255.99) as i32,
                (self.image[i].y * 255.99) as i32,
                (self.image[i].z * 255.99) as i32));
            }
        }
        fs::write(path, file).expect("Failed to write to file");
        println!("Saved buffer to file: {}", path);
    }
    
}