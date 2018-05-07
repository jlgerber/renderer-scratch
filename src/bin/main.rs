extern crate  trait_test;
use trait_test::*;
use std::{
    sync::Arc,
};

fn main() {
    let mat = Box::new(Lambert::new(200,200,200));
    let sphere = Arc::new(Sphere::new(2.0, mat));
    //sphere.render();

    let pool = ThreadPool::new(4);
    for c in 0..4 {
        let sphere = Arc::clone(&sphere);
        pool.execute(move ||{
            sphere.render();
            println!("");
        });
    }
}
