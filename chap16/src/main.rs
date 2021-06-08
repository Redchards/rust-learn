use std::thread;
use std::time::Duration;

fn main() {
    // Now onto threads
    // Here's how you spawn threads :
    let t1 = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi, number {} from first spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..10 {
        println!("Hi, number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Here we will wait for the spawned threads to have stopped before continuing
    t1.join().unwrap();

    println!("Thread 1 finished!");

    for i in 1..10 {
        println!("Hi, number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }


}
