use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::Mutex;
use std::sync::Arc;

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

    // NOTE : the durations have been reduced to keep on writing examples without having to wait, but it can be increased
    // to witness the concurrency.
    // It makes sense, as the other thread could change the value and produce inaccurate results or even drop the value

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("the"),
            String::from("spawned"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for r in rx {
        println!("{}", r);
    }

    // We can also clone transmitters. Below is an example with transmitters :
    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("the"),
            String::from("spawned"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("More"),
            String::from("messages"),
            String::from("from"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for r in rx {
        println!("{}", r);
    }

    // In won't repeat here the little blurb about mutexes that is present in the Rust book, but let's see how the type system
    // helps us with that

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("num = {:?}", m);

    // Let's try to share the mutex between multiple threads. The following wouldn't work :
    // let counter = Mutex::new(0);
    // let mut handles = vec![];
    //
    // for _ in 1..10 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });

    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // This is because we're moving the mutex in the first closure. Fortunately, we have Rc<T>, right? Let's see :
    // let counter = Rc::new(Mutex::new(0));
    // let mut handles = vec![];

    // for _ in 1..10 {       
    //     let counter = Rc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num +=1;
    //     });

    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // Here we get : `Rc<Mutex<i32>>` cannot be sent between threads safely. Pretty explicit
    // Rc<T> is indeed not safe to send between threads, it doesn't implement the trait "send". Fortunately, with a tiny
    // change we can get a thread safe Rc<T>, using Arc<T>
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num +=1;
        });

        handles.push(handle);
    }


    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final value : {}", counter.lock().unwrap());

    // Now let's quickly talk about relevant traits, Sync and Send
    // Send indicates that the ownership of the value can be transferred between traits. Most types are Send, except for Rc<T>
    // and a few others.
    // Almost all primitives types, excluding pointers, are Send, and objects composed of primitive types or other objects that
    // are Send are automatically Send in turn.
    // A type T is sync if its reference can be sent to another thread, meaning that &T is Send.
    
}
