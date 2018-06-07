use HitRecord;

pub trait Shape:Sync+Send {
    fn render(&self) -> String;
    fn hit(&self, hit_record: &mut HitRecord);
}

pub trait Material:Sync+Send {
     fn render(&self) -> String;
}

pub trait Texture:Sync+Send {
     fn render(&self) -> String;
}


pub trait FnBox {
    fn call_box(self: Box<Self>);
}


impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}