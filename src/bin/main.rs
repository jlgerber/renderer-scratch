extern crate  trait_test;
extern crate scoped_threadpool;
use scoped_threadpool::Pool;

use trait_test::*;
use std::{
    sync::mpsc,
    sync::Arc,
};
use std::collections::HashMap;
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

    //let pool = ThreadPool::new(threads);
    let mut pool = Pool::new(threads);
    pool.scoped(|scope|
    for _c in 0..16 {
        let sphere = Arc::clone(&sphere);
        scope.execute(move ||{
            println!("{}",sphere.render());
        });
    }
    );

    // add a sentinel message which indicates we are complete
    // we will just send
    //signal(&pool, threads)();

    // let (snd,rec) = mpsc::channel();
    // signal_done(&pool, snd, threads);
    // wait_for_done(rec, threads);

}

// fn signal(pool: &ThreadPool, threads: usize) -> Box<Fn() -> ()> {
//     let (snd,rec) = mpsc::channel();
//     for _c in 0 .. threads {
//         let snd_c = snd.clone();
//             pool.execute(move || {
//                 snd_c.send(1).unwrap();
//         });
//     }
//     Box::new(move || {
//         wait_for_done(rec, threads);
//     })
// }

fn signal_done(pool: &ThreadPool, snd: mpsc::Sender<usize>, threads:usize) {
    for _c in 0 .. threads {
        let snd_c = snd.clone();
            pool.execute(move || {
                snd_c.send(1).unwrap();
        });
    }
}

fn wait_for_done(rx: mpsc::Receiver<usize>, threads: usize) {
    let mut cnt = 0;
    loop {
        cnt += rx.recv().unwrap();
        if cnt >= threads {
            println!("Done");
            break;
        }
    }
}