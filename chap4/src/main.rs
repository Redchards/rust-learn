fn main() {
    // this variable will be available for the rest of the main function
    let always_available = "Hello world";
    println!("I'm not going out of scope for the remaining of this function!! {}", always_available);

    // This one won't
    {
        let available_in_this_scope = "Bye world";
        println!("I'm going out of scope rn!! {}", available_in_this_scope)
    }

    // Strings can be mutated unlike string literals, but they also must be allocated on the heap
    // Tbh I'm also writing a lot of comments just to practice writing with a qwerty keyboard
    // Filthy frenchman is having trouble doing that rnm which is not very good
    // Because I'm stupidly retarded and have to use an azerty to be able to type efficiently...
    let mut s1 = String::from("Hello string!");
    println!("When not mutated I say '{}'", s1);
    s1.push_str(" And hello you!");
    println!("When mutated I say '{}'", s1);

    // Now we copy the content of s1 to s2... or, do we?
    // Actually, no, we don't, we move s1 to s2, and hence can't use s1 anymore, if we try to, Rust won't let us do so
    let s2 = s1;
    println!("Now our initial string has been moved and isn't valid anymore, we can't use it and Rust won't let use anyway, why would it?");
    println!("But our s2 says : {}", s2);

    // Can't do that anymore, because the String class does not implement the copy trait, so it can't be borrowed here
    // println!("{}", s1);

    // If we clone the string, then it's different, the original isn't moved, but a deep copy is created!
    // Now both s2 and s3 can be used
    let s3 = s2.clone();
    println!("Our s2 says : {}", s2);
    println!("Our s3 says : {}", s3);

    // Primitive types are allocated on the stack and hence will always see a deep copy performed on them
    let x1 = 6;
    let x2 = x1;

    // x1 hasn't been moved and can stil be used here
    println!("x1={} and x2={}", x1, x2);

    // Now let's see how ownership works with functions
    // By default a function will take ownership for a variable

    let will_be_stolen = String::from("Hello");
    takes_ownership(will_be_stolen);

    // Oh no!! The function stole the ownership, now we can't use our string anymore... Much sad
    // The following line wouldn't work
    // println!("{}", will_be_stolen);
    // And neither would this one
    // let new_string = will_be_stolen

    // With an integer, the ownership won't be taken though
    let x3 = 5;
    copies_stuff_because_primitive(x3);

    println!("{}", x3);

    // But we can also grab the stolen variable back if the function is ever so kind to give it back to us
    let will_be_given_back = String::from("Hello");
    let will_be_given_back = takes_ownership_and_give_it_back(will_be_given_back);

    println!("The function gave me back ownership of a string containing '{}'", will_be_given_back);

    // But it's very cumbersome, we what if we want to return another value as well? We can use a tuple for instance, but it wouldn't be very nice
    // especially as it's something what we want to be able to do quite often, every time we pass a value that is not dee copyable to a function a matter
    // of fact
    // Let's have a gander at a semi-practical example

    let s = String::from("hello");
    let (s, length) = compute_length(s);
    println!("The string {} has length {}", s, length);

    // But clearly, we don't want to be doing that as it's very cumbersome, we would like for functions to have a way to take ownership temporarily...
    // To "borrow" our variable so to speak... Oh wait, the creator of Rust was far from a complete idiot, so of course he tought ahead and implemented 
    // exaclty that in the language : borrowing
    // Hurray!

    let s = String::from("hello");
    println!("Our string has length {}", borrow_string(&s));
    
    // Of course by default, a reference is not mutable, so the function can't change its value
    // So doing a push_str in the function wouldn't work for example
    // For that, we need to use a mutable reference

    let mut s = String::from("hello");
    borrow_and_modify_string(&mut s);
    println!("Oh! The string was modified!! It now says {}", s);

    // That being said, we're in Rust, so we're not gonna be allowed to do whatever
    // For that reason, we're limited to one mutable reference to a particular variable per block of code
    // Hence the following wouldn't work, telling us that we can't borrow `s` as a mutable more than once
    // let mut s = String::from("hello");
    // let sr1 = &mut s; 
    // let sr2 = &mut s; 
    // println!("{}, {}", sr1, sr2);

    // It allows us to avoid data races (at most one pointer will modify the reference at a time)

    // We can do that if we are using a new scope though
    let mut s = String::from("hello");

    { 
        let sr1 = &mut s; 
        println!("{}", sr1);
    }

    let sr2 = &mut s; 

    println!("{}", sr2);

    // Rust merely requires both the mutable references to not be in the same blocks of code, also said to be simulateneous 
    // Note that we can have as many immutable references as we want though, but if we have a immutable reference, we can't have 
    // a mutable one anymore
    // let mut s = String::from("hello");
    // let sr1 = &s; 
    // let sr2 = &s; 
    // let sr3 = &mut s; 
    // println!("{}, {}, {}", sr1, sr2, sr3);
    
    // As always there's a pretty good reason for that, it's mainly because the users of immutable references don't expect the value to
    // suddenly change!

    // We also can't have a dangling reference using this mechanism, as if the object doesn't exist in the scope we're in anymore, then
    // we will refuse the borrowing
    // It means that the following won't work 
    // let dangle_ref = {
    //     let s = String::from("hello");
    //     &s
    // };

    // To sum up, the rules are as follows
    // At any given time, we can have either one mutable reference or any number of mutable references
    // References must always be valid

    // Slices are useful as they allow us to get a view on an object without a copy but also prevent use from doing stupid things
    // Example 
    let mut s = String::from("hello world");
    let words = (find_nth_word(&s, 0), find_nth_word(&s, 1));
    // This line doesn't work
    // s.clear();
    // because of the immutable borrow
    println!("{}, {}", words.0, words.1);
    // Now we can clear it though
    s.clear();

    // Note that string literals are slices
}

fn takes_ownership(s: String) {
    println!("Ahah! I took ownership of a string that contains the message '{}' and I'm not giving it back!!!", s);
}

fn copies_stuff_because_primitive(n: i32) {
    println!("I just copied this number man... {}", n);
}

fn takes_ownership_and_give_it_back(s: String) -> String {
    println!("This is mine for a while, but then I will give it back, or throw it away if nobody wants it anymore :3 {}", s);
    s
}

fn compute_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn borrow_string(s: &String) -> usize {
    s.len()
}

fn borrow_and_modify_string(s: &mut String) {
    s.push_str(" you and hello everybody!!");
}

fn find_nth_word(s: &String, n: u32) -> &str {
    let bytes = s.as_bytes();
    let mut first = 0;
    let mut count = 0;

    for (idx, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            if count == n
            {
                return &s[first..idx];
            }
            count += 1;
            first = idx + 1;
        }
    }

    &s[0..]
}