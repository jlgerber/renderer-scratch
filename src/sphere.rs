use std::{
    sync::Arc,
};
use Material;
use Shape;
pub struct Sphere {
    pub radius: f32,
    pub material: Arc<Material+Send>,
}

impl Sphere {
    pub fn new(radius: f32, material: Arc<Material+Send> ) -> Sphere {
        Sphere { radius, material }
    }
}

impl Shape for Sphere {
    fn render(&self) {
        print!("sphere radius: {}",self.radius);
        self.material.render();
        println!("");
    }
}