use std::ops::Deref;
use std::rc::Rc;

fn main() {
    // Here come the smart pointers
    // A simple example of a boxed value for i32. The variable is allocated on the stack and we just retrieve a pointer to it
    {
        let b = Box::new(5);
        println!("Boxed = {}", b);
    } // Here b is deallocated

    // Boxed types also allow for recursive types. Indeed, Rust must know the size of the types at compile time, hence
    // it we had something like the following, it wouldn't work :
    // enum List {
    //   Cons(u32, List),
    //   Nil,
    // }
    //
    // Then it wouldn't work because we would infinitely recurse trying to find the type size. By definition, the size of the type
    // can't be known without knowing its own size...
    // But pointer have a very well defined size (sizeof(usize)), and as such using a boxed type would solve our issue
    
    // Implementing a "Cons list" would go as follows
    use lispy::List;

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("Hello cons list : {}", list);

    // Smart pointers must implement the Deref and Drop traits, the former of which allows them to be treated like typical references
    // A reference in Rust is functionally a pointer, even the syntax is quite similar to that of C (it differs from the references in
    // the C++ language though, that are pretty much automatically dereferences and not reassignable pointers)
    let mut a = 5;
    println!("a = {}", a);

    let a_ref = &mut a;
    println!("a_ref = {}", a_ref); // We will also print 5 because the reference will be dereferenced in the println! macro
    println!("*a_ref = {}", *a_ref);

    // We can't write the following :
    // a_ref = 42;

    // However, we can write this :
    *a_ref = 42;
    println!("*a_ref = {}", *a_ref); // Will correctly print out 42

    // In that regard, the Box<T> type is almost indistinguishable
    let a = 5;
    let mut a_ref = Box::new(a);

    println!("a = {}", a);
    println!("a_ref = {}", a_ref); // We will also print 5 because the reference will be dereferenced in the println! macro
    println!("*a_ref = {}", *a_ref);

    // We still can't write the following :
    // a_ref = 42;

    // Similarly, we can write this :
    *a_ref = 42;
    println!("*a_ref = {}", *a_ref); // Will correctly print out 42

    // As we can see, the semantic for smart pointers and references is the same
    // Let's try to make our own now (though it won't allocate anything on the heap)
    let a = 5;
    #[allow(unused_variables)]
    let a_ref = BadBox::new(a);
    println!("a = {}", a);

    // The two following lines won't work though
    // println!("a_ref = {}", a_ref);
    // println!("*a_ref = {}", *a_ref);
    // The first one is because we are not implementing the trait Display
    // The second one is more interesting though : type BadBox<{integer}> cannot be dereferenced

    let a = 5;
    let a_ref = GoodBox::new(a);
    println!("a = {}", a);
    println!("*a_ref = {}", *a_ref); // Prints 5, YAY!
    // The following line still won't work because we haven't implemented the Display trait
    // println!("a_ref = {}", a_ref);

    let m = GoodBox::new(String::from("world"));
    hello(&m);

    // Deref coercion works in the following way :
    // From &T to &U when T: Deref<Target=U>
    // From &mut T to &mut U when T: DerefMut<Target=U>
    // From &mut T to &U when T: Deref<Target=U>
    
    // Note that because if we have a mutable reference to a piece of data it can be the onnly reference to
    // the said data, the third rule works but the opposite couldn't

    let c = CustomSmartPointer {
        data: String::from("I will be destroyed!"),
    };

    let _d = CustomSmartPointer {
        data: String::from("I will be destroyed too!"),
    };

    println!("CustomSmartPointers created");
    // And they will be dropped at the end of the main function
    // We can also force Rust to drop things early
    // The following line won't work though :
    // c.drop();
    // We can't do that as we would borrow 'c', drop it and then the drop method would be called again at the end of main
    // creating a double free situation
    // Instead we must do the following
    drop(c);
    println!("Dropped early!");

    // Now let's have a gander at Rc, the reference-counter smart pointer by combining lists
    // The following wouldn't work :
    // let a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
    // let b = List::Cons(3, Box::new(a));
    // let c = List::Cons(4, Box::new(a));

    // a is moved in the first Box::new(a) to construct b, we can't construct c
    // With a new type that takes Rc<T> instead of Box<T>, we can write it though :
    let a = Rc::new(List2::Cons(5, Rc::new(List2::Cons(10, Rc::new(List2::Nil)))));
    println!("Count after creating a : {}", Rc::strong_count(&a));
    let _b = List2::Cons(3, Rc::clone(&a));
    println!("Count after creating _b : {}", Rc::strong_count(&a));
    {
        let _c = List2::Cons(4, Rc::clone(&a));
        println!("Count after creating _c : {}", Rc::strong_count(&a));
    }
    println!("Count after _c goes out : {}", Rc::strong_count(&a));
}

mod lispy {

use std::fmt::Display;
use std::fmt;

pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T: ToString> List<T> {
    fn to_string(&self) -> String {
        let mut s = String::new();

        match self {
            List::Cons(val, next) => {
                s.push_str(&val.to_string()[..]);
                s.push_str(", ");
                s.push_str(&next.to_string()[..]);
            },
            List::Nil             => ()
        };
        
        s
    }
}

impl<T: ToString> Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

}

struct BadBox<T>(T);

impl<T> BadBox<T> {
    fn new(x: T) -> BadBox<T> {
        BadBox(x)
    }
}

struct GoodBox<T>(T);

impl<T> GoodBox<T> {
    fn new(x: T) -> GoodBox<T> {
        GoodBox(x)
    }
}

impl<T> Deref for GoodBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello {}!", name);
}

struct CustomSmartPointer {
    data: String
}

// Drop is included in the Prelude, no need to import it
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'", self.data);
    }
}

pub enum List2<T> {
    Cons(T, Rc<List2<T>>),
    Nil,
}

#[allow(dead_code)]
impl<T: ToString> List2<T> {
    fn to_string(&self) -> String {
        let mut s = String::new();

        match self {
            List2::Cons(val, next) => {
                s.push_str(&val.to_string()[..]);
                s.push_str(", ");
                s.push_str(&next.to_string()[..]);
            },
            List2::Nil             => ()
        };
        
        s
    }
}

