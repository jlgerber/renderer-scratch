extern crate  trait_test;
extern crate scoped_threadpool;
use scoped_threadpool::Pool;

use std::{
    sync::Arc,
};
use std::collections::HashMap;

use trait_test::*;
use trait_test::traits::*;

type MaterialMgr = HashMap<String,Box<Material>>;

fn main() {
    let key = String::from("material");
    let key_ref = key.clone();
    let mut mgr = MaterialMgr::new();
    let texture = Box::new(Constant::new(200,100,50));
    let mat = Box::new(Lambert::new(texture));
    mgr.insert(key, mat);

    let keyref = mgr.get(key_ref.as_str()).unwrap();
    let sphere = Arc::new(Sphere::new(2.0, keyref));
    let threads = 4;

    let mut pool = Pool::new(threads);
    pool.scoped(|scope|
        for _c in 0..16 {
            let mut hr = HitRecord::new();
            let sphere = Arc::clone(&sphere);
            scope.execute(move ||{
                sphere.hit(&mut hr);
                println!("{}",sphere.render());
            });
        }
    );
}
