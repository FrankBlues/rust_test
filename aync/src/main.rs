// extern crate futures;
use futures::executor::block_on;
extern crate threadpool;

use threadpool::ThreadPool;
use std::sync::mpsc::channel; 


fn main() {
    // let future = hello_world(); // Nothing is printed
    // block_on(future); // `future` is run and "hello, world!" is printed

    let n_workers = 4;
    let n_jobs = 8;
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = channel();
    for _ in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move|| {
            tx.send([1, 2]).unwrap();
        });
    }

    // println!("{}", rx.iter().take(n_jobs).fold(0, |[a, b]| a + b));

    // assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), 8);
}

fn add (a: i32, b: i32) -> i32 {
    a + b
}

// async fn hello_world() {
//     println!("hello, world!");
// }


