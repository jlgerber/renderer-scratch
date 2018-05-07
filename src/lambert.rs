use Material;
use Texture;

pub struct Lambert {
   pub texture: Box<Texture>
}

impl Lambert {
    pub fn new(texture: Box<Texture> ) -> Lambert {
        Lambert {texture }
    }
}

impl Material for Lambert {
    fn render(&self) -> String {
        format!("Lambert {}", self.texture.render())
    }
}