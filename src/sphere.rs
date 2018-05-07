
use Material;
use Shape;
pub struct Sphere {
    pub radius: f32,
    pub material: Box<Material>
}

impl Sphere {
    pub fn new(radius: f32, material: Box<Material> ) -> Sphere {
        Sphere { radius, material }
    }
}

impl Shape for Sphere {
    fn render(&self) -> String {
       format!("sphere radius: {} material: {}",self.radius, self.material.render())
    }
}