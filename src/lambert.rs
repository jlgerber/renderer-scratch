use Material;

pub struct Lambert {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Lambert {
    pub fn new(r: u8, g: u8, b:u8) -> Lambert {
        Lambert {r, g, b }
    }
}

impl Material for Lambert {
    fn render(&self) {
        print!("material: Lambert r:{} g:{} b:{}", self.r, self.g, self.b)
    }
}