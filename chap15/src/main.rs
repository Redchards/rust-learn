use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

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
    use lispy::List2;

    let a = Rc::new(List2::Cons(5, Rc::new(List2::Cons(10, Rc::new(List2::Nil)))));
    println!("Count after creating a : {}", Rc::strong_count(&a));
    let _b = List2::Cons(3, Rc::clone(&a));
    println!("Count after creating _b : {}", Rc::strong_count(&a));
    {
        let _c = List2::Cons(4, Rc::clone(&a));
        println!("Count after creating _c : {}", Rc::strong_count(&a));
    }
    println!("Count after _c goes out : {}", Rc::strong_count(&a));

    // The concept of interior mutability is an interesting one. Indeed, consider the following piece of code :
    // let a = 5;
    // let a_ref = &mut a;

    // The compiler will obviously not allow that, we can't borrow mutably a value that is immutable. But it could sometimes be
    // useful to have a value mutate itself, inside of a function or method for instance, but appear immutable to the outisde
    // world : this is where interior mutability intervenes (examples in the test section)

    // The RefCell still obeys to the same rule, but at compile time. Meaning that the following code would panic :
    // let cell_x = RefCell::new(Vec::<String>::new());
    // 
    // let ref_1 = cell_x.borrow_mut();
    // let ref_2 = cell_x.borrow_mut();

    // Combining Rc (that gives us multiple owners but only immutable access to the data) and RefCall (that gives us mutable access
    // to a single piece of data), we can have mutable access from multiple owners.
    // Using the Cons list that we made earlier :
    use lispy::List3;

    let a = Rc::new(RefCell::new(List3::Cons(
        5, 
        Rc::new(RefCell::new(List3::Cons(
            10, 
            Rc::new(RefCell::new(
                List3::Nil))
        )))
    )));

    println!("Count after creating a : {}", Rc::strong_count(&a));
    let b = List3::Cons(3, Rc::clone(&a));
    println!("Count after creating _b : {}", Rc::strong_count(&a));
    {
        let c = List3::Cons(4, Rc::clone(&a));
        println!("Count after creating _c : {}", Rc::strong_count(&a));

        println!("Displaying the lists...");
        println!("a = {}", a.borrow());
        println!("b = {}", b);
        println!("c = {}", c);

        match *a.borrow_mut() {
            List3::Cons(ref mut val, _) => *val += 10,
            List3::Nil              => (),
        }

        println!("We changed the first value of a, here are the results :");
        println!("a = {}", a.borrow());
        println!("b = {}", b);
        println!("c = {}", c);
    }

    println!("Count after c goes out : {}", Rc::strong_count(&a));


    // Though with Rc we can get cycles and never free anything, causing a leak (Rust will still consider it "memory safe")
    // Here's an example :
    {
        use lispy::List4;

        let a = Rc::new(List4::Cons(5, RefCell::new(Rc::new(List4::Nil))));
        println!("Initial state after initializing a, refcount = {}", Rc::strong_count(&a));
        println!("a = {:?}", a);

        let b = Rc::new(List4::Cons(5, RefCell::new(Rc::clone(&a))));
        println!("State after creating b, refcount = {}", Rc::strong_count(&a));
        println!(" b initial refcount = {}", Rc::strong_count(&b));
        println!("b = {:?}", b);

        if let Some(t) = a.tail() {
            *t.borrow_mut() = Rc::clone(&b);
        }

        println!("a refcount after cycle create = {}", Rc::strong_count(&a));
        println!("b refcount after cycle create = {}", Rc::strong_count(&b));

        // The following line would create a stack overflow because of the cycle :
        // println!("a = {:?}", a);
    }

    // Here the memory still hasn't be freed, because a has been destroyed, the strong ref counter went down from 2 to 1, because
    // the tail of b is still pointing to a. Similarly, for b the tail of a is pointing to b, so they will both keep each other 
    // alive for the remainder of the program's lifetime

    // To prevent that, we can use a Weak pointer through Rc::downgrade. Before accessing the value, we have to use Weak::upgrade 
    // which will promote the Weak pointer into an Option<Rc> (only valid if the strong_counter > 0)

    let leaf = Rc::new( Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("Leaf parent : {:?}", leaf.parent.borrow().upgrade());
    
    let branch = Rc::new( Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("Leaf parent : {:?}", leaf.parent.borrow().upgrade());

    // Here we created a cycle, but as getting a Weak from an Rc using Rc::downgrade doesn't increment the strong reference counter,
    // this cycle won't make it so that the smart pointers keep each other alive. A weak_count > 0 will not prevent the resource from
    // being freed

    // Let's explore how the strong_count and weak_count evolve
    let leaf = Rc::new( Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("Leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    
    {
        let branch = Rc::new( Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("Leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
        println!("Branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
    }

    println!("Leaf parent : {:?}", leaf.parent.borrow().upgrade());
    println!("Leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}

mod lispy {

use std::fmt::Display;
use std::fmt::Debug;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

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

#[derive(Debug)]
pub enum List3<T> {
    Cons(T, Rc<RefCell<List3<T>>>),
    Nil,
}

#[allow(dead_code)]
impl<T: ToString> List3<T> {
    fn to_string(&self) -> String {
        let mut s = String::new();

        match self {
            List3::Cons(val, next) => {
                s.push_str(&val.to_string()[..]);
                s.push_str(", ");
                s.push_str(&((*next).borrow().to_string()[..]));
            },
            List3::Nil             => ()
        };
        
        s
    }

    pub fn tail(&self) -> Option<&Rc<RefCell<List3<T>>>> {
        match self {
            List3::Cons(_, t) => Some(t),
            List3::Nil        => None
        }
    }
}

impl<T: ToString> Display for List3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug)]
pub enum List4<T> {
    Cons(T, RefCell<Rc<List4<T>>>),
    Nil,
}

impl<T: ToString> List4<T> {
    pub fn tail(&self) -> Option<&RefCell<Rc<List4<T>>>> {
        match self {
            List4::Cons(_, t) => Some(t),
            List4::Nil        => None
        }
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


// Agnostic message sending trait
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger: messenger,
            value: 0,
            max: max
        }
    }

    fn capacity_used(&self) -> f64 {
        self.value as f64 / self.max as f64
    }

    pub fn set_value(&mut self, new_value: usize) {
        self.value = new_value;

        let percent_of_max = self.capacity_used();

        if percent_of_max >= 1.0 {
            self.messenger.send("Error : you are over the quota!");
        }
        else if percent_of_max >= 0.9 {
            self.messenger.send("Critical : you have used over 90% of your quota!");
        }
        else if percent_of_max >= 0.75 {
            self.messenger.send("Warning : you are using over 75% of your quota.");
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg))
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning() {
        let messenger = MockMessenger::new();

        let mut tracker = LimitTracker::new(
            &messenger,
            100,
        );

        tracker.set_value(80);

        // We couldn't do that without RefCell, because the reference to the Messenger is not mut
        assert_eq!(tracker.messenger.sent_messages.borrow().len(), 1);
        assert_eq!(*tracker.messenger.sent_messages.borrow(), vec![String::from("Warning : you are using over 75% of your quota.")]);
    }
}