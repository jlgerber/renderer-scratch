
use Material;
use Shape;
pub struct Sphere<'a> {
    pub radius: f32,
    pub material: &'a Box<Material>
}

impl<'a> Sphere<'a> {
    pub fn new(radius: f32, material: &'a Box<Material> ) -> Sphere {
        Sphere { radius, material }
    }
}

impl<'a> Shape for Sphere<'a> {
    fn render(&self) -> String {
       format!("sphere radius: {} material: {}",self.radius, self.material.render())
    }
}