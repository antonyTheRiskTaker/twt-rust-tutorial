use std::thread;
// use std::time::Duration;
// use std::sync::mpsc;
use std::sync::Mutex;
use std::rc::Rc;

fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {i} from the spawned thread!");
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    
    // handle.join().unwrap();

    // for i in 1..5 {
    //     println!("hi number {i} from the main thread!");
    //     thread::sleep(Duration::from_millis(1));
    // }

    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(move || {
    //     println!("Here's a vector: {:?}", v);
    // });

    // drop(v);

    // handle.join().unwrap();

    // let (tx, rx) = mpsc::channel();

    // let tx1 = tx.clone();
    // thread::spawn(move || {
        // let val = String::from("hi");
        // tx.send(val).unwrap();
        // println!("val is {}", val); // Error!
        // let vals = vec![
        //     String::from("hi"),
        //     String::from("from"),
        //     String::from("the"),
        //     String::from("thread"),
        // ];

        // for val in vals {
        //     tx1.send(val).unwrap();
        //     thread::sleep(Duration::from_secs(1));
        // }
    // });

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("more"),
    //         String::from("messages"),
    //         String::from("for"),
    //         String::from("you"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);
    // for received in rx {
    //     println!("Got: {received}");
    // }

    // let m = Mutex::new(5);

    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }

    // println!("m = {:?}", m);
    
    // TODO: continue from `Multiple Ownership with Multiple Threads`

    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}