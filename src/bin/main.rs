extern crate  trait_test;
use trait_test::*;

fn main() {
    let mat = Box::new(Lambert::new(200,200,200));
    let sphere = Sphere::new(2.0, mat);
    sphere.render();
}
