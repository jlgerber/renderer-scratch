
use Material;

pub struct HitRecord<'b> {
    pub material: Option<&'b Box<Material + 'b>>
}

impl<'b> HitRecord<'b> {
    pub fn new() -> HitRecord<'b> {
        HitRecord { material: None }
    }
}