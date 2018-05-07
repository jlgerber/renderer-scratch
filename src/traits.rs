pub trait Shape {
    fn render(&self);
}

pub trait Material:Sync {
     fn render(&self);
}

pub trait Texture {
     fn render(&self);
}


pub trait FnBox {
    fn call_box(self: Box<Self>);
}


impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}