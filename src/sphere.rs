
use Material;
use Shape;
use HitRecord;

pub struct Sphere<'a> {
    pub radius: f32,
    pub material: &'a Box<Material + 'a>
}

impl<'a> Sphere<'a> {
    pub fn new(radius: f32, material: &'a Box<Material + 'a> ) -> Sphere<'a> {
        Sphere { radius, material }
    }
}

impl<'a> Shape for Sphere<'a> {
    fn render(&self) -> String {
       format!("sphere radius: {} material: {}",self.radius, self.material.render())
    }

    fn hit(&self, hit_record: &mut HitRecord) {
        hit_record.material = Some(self.material);
    }

}