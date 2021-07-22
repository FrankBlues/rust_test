use std::thread;
use std::time::Duration;

use std::sync::mpsc;

fn main() {
    // new thread  spawn join
    // A JoinHandle is an owned value that, when we call the join method on it, will wait for its thread to finish
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    // handle.join().unwrap();
    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // handle.join().unwrap();

    //move

    // let v = vec![1, 2, 3];

    // // force the closure to take ownership of the values it’s using rather than allowing Rust to infer that it should borrow the values.
    // let handle = thread::spawn(move || {
    //     println!("Here's a vector: {:?}", v);
    // });

    // handle.join().unwrap();
    // channel
    // mpsc stands for multiple producer, single consumer
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    // send one string so the spawned thread is communicating with the main thread
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // tx.send(val).unwrap();
        // error value borrowed here after move
        // println!("val is {}", val);
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // block the main thread’s execution and wait until a value is sent down the channel
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    // treating rx as an iterator
    for received in rx {
        println!("Got: {}", received);
    }

}
