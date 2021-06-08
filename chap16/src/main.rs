use std::thread;
use std::time::Duration;
use std::sync::mpsc;

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

    // Capturing variables is a little bit more tricky though. Consider the following example:
    // let v = vec![1, 2, 3];
    //
    // thread::spawn(|| {
    //    println!("Here's a vector {:?}", v);
    // });
   
    // This code would produce "closure may outlive the current function, but it borrows v, which is owned by the current function"
    // Then Rust hints at us that we could use the "move" keyword to move the captured objects. Here's an example

    let v = vec![1, 2, 3];
    
    thread::spawn(move|| {
       println!("Here's a vector {:?}", v);
    });

    // The following line is now invalid :
    // println!("Here's a vector {:?}", v);
    // Also, consider what woul happen if we were to call "drop" after spawning the thread in both of these examples :)
    // In the first example, we get the reason why we can't borrow : the resource may get freed in the main thread 
    // and as such, the reference may become dangling at any time
    // In the second case, the value has been moved, we don't own it anymore, we can't drop it

    // Let's look at channels now.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi from spawned thread");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Received : {}", received);

    // It's important to realize that the send() method moves the value : we can't use it afterwards. For instance, this
    // snippet wouldn't work
    // ...
    // 
    //    thread::spawn(move || {
    //        let val = String::from("Hi from spawned thread");
    //        tx.send(val).unwrap();
    //        println!("val = {}", val); // Can't print that!
    //    });
    // ...

    // It makes sense, as the other thread could change the value and produce inaccurate results or even drop the value
}
