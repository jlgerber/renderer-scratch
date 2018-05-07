use Texture;

pub struct Constant {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Constant {
    pub fn new(r: u8, g: u8, b:u8) -> Constant {
        Constant {r, g, b }
    }
}

impl Texture for Constant {
    fn render(&self) -> String {
        format!("Constant r:{} g:{} b:{}", self.r, self.g, self.b)
    }
}