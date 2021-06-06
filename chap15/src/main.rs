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

