extern crate  trait_test;
use trait_test::*;
use std::{
    sync::Arc,
};

fn main() {
    let texture = Box::new(Constant::new(200,100,50));
    let mat = Box::new(Lambert::new(texture));
    let sphere = Arc::new(Sphere::new(2.0, mat));

    let pool = ThreadPool::new(4);
    for _c in 0..4 {
        let sphere = Arc::clone(&sphere);
        pool.execute(move ||{
            println!("{}",sphere.render());
        });
    }
}
