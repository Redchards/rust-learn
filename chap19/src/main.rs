use std::slice;
use std::ops::Add;
use std::fmt;
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

type Thunk = Box<dyn Fn() + Send + 'static>;

#[macro_export]
macro_rules! myvec {
    ( $( $x:expr),* ) => {
        {
            let mut tmp_vec = Vec::new();
            $(
                tmp_vec.push($x);
            )*

            tmp_vec
        }
    };
}

fn main() {
    // Unsafe rust gives us the following 5 superpowers :
    // * Dereference raw pointers
    // * Call an unsafe function or method
    // * Access or modify a mutable static variable
    // * Implement an unsafe trait
    // * Access fields of unions

    // The borrow checker will not be disabled inside of Rust's unsafe blocks, but these potentially memory unafe
    // operations will be allowed by the static analyzer.
    // Unsafe blocks are often enclosed within safe abstractions and APIs are provided for them

    // Though the compiler ensures that references my always be valid. Raw pointers work in the same way as references
    // but cannot be used in a safe context, can be both mutable, immutable and are written *const T and *mut T 
    // respectively.

    // Raw pointers :
    // * Are allowed to ignore the borrowing rules, having both multiple immutable and mutable pointers to the same
    // memory location
    // * Aren't guaranteed to point to valid memory
    // * Are allowed to be null
    // * Don't implement any automatic cleanup through RAII

    // Creating pointers is considered safe Rust as long as we don't dereference them

    let mut num = 5;

    let r1 = &num as *const i32; // *const means that it can't be assigned after being dereferenced
    let r2 = &mut num as *mut i32;

    // What we've just written wouldn't be allowed using references (both mutable and immutable refs to same object)

    let addr = 0x012345usize;
    let _r3 = addr as *const i32;

    unsafe {
        println!("*r1 : {}", *r1);
        println!("*r2 : {}", *r2);
    }

    unsafe {
        dangerous();
    }

    // We obviously can't call the "dangerous" function outside of an unsafe block
    // This is inconvenient, so very often we want to create a safe abstraction over an unsafe block of code
    // Let's look at the "split_at_mut" method :
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // Unsafe allows us to do things like accessing arbitrary memory locations, which can be dangerous
    // The following code would crash upon using the "slice"

    let addr = 0x012345usize;
    let r = addr as *mut i32;

    // We're creating a slice that is 1000 items long from an arbitrary memory address
    let _slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 1000) };

    // We can mark functions as extern "C" to interact with C libraries
    // C functions can only be called within an unsafe block
    // The other way around, but can tag a function #[no_mangle] and extern "C" to have C interact with Rust

    // Global mutable variables are, by nature, unsafe, as they could very easily lead to data races, we can't access
    // them without an "unsafe" block

    // Operator overloading through traits
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 1 };

    println!("p1 + p2 = {:?}", p1 + p2);

    // It must be noted that the Add trait has a default generic parameter :
    // trait Add<Rhs=Self> {
    //     type Output;

    //     fn add(self, rhs: Rhs) -> Self::Output;
    // }

    let d1 = Millimeters(250);
    let d2 = Meters(2);
    println!("d1 + d2 = {:?}", d1 + d2);

    // We can also disambiguate function calls with the same name implemented by different traits

    let dog = Doggo;

    Small::bark(&dog);
    Big::bark(&dog);
    Doggo::bark(&dog);
    dog.bark();

    // The two last ones are strictly equivalent
    // But what's up if you have class methods?
    println!("My baby doggo is called {}", Doggo::baby_name());
    
    // The following line wouldn't work though :        
    // println!("A baby dog is called a {}", Animal::baby_name());
    // It must be fully qualified
    println!("A baby dog is called a {}", <Doggo as Animal>::baby_name());

    // Example of display using the newtype pattern
    let w = WrapperVec(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    // The newtype pattern has one downside though : we don't have the methods of the wrapped type
    // If we wanted to replicate the same functionalities, we would have to implement all the methods ourselves
    // or simply implement the Deref trait
    // Type aliases are different from newtypes (like Millimeters and Meters), they're simply a new name for a 
    // type, but do not declare a new type

    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // It can be useful to remove long type names
    let f: Thunk = Box::new(|| println!("hi!"));
    takes_long_type(f);
    returns_long_type()();

    // There's also the ! return type that means never returns. It's useful for expressions like continue or break
    // as well as macros such as panic!
    // For instance, we can't write the following code :
    // let toto = Ok(5);
    // let guess = match toto {
    //     Ok(_) => 5,
    //     Err(_) => "hello",
    // }
    // Error : match arms have incompatible types

    // But as panic! has the ! never return type, we know that it will NEVER return, meaning that we don't need to
    // assign a value, and we can write the following :
    let toto: Result<i32, ()> = Ok(5);
    let _guess = match toto {
        Ok(_) => 5,
        Err(_) => panic!("NOOOOO!"),
    };

    // Dynamically Size Types (DST) or unsized types are types which size can only be known at runtime. One example
    // of such a type is the str type, not the &str type, the str type. For instance, the following code doesn't work
    // let s1: str = "Hello there!";
    // "doesn't have a size known at compile-time"
    // In fact, implicitely, every generic type implements the trait Sized
    // fn generic<T>(t: T) {
    // }
    // Is in fact equivalent to
    // fn generic<T: Sized>(t: T) {
    // }

    // However, if we want to say that the type may not be sized, we can use the ?Sized trait
    // fn generic<T: ?Sized>(t: T) {
    // }
    // This syntax only exists for the Sized trait

    // We've seen how we can use closures (lambdas), but we can also use function pointers :
    let answer = do_twice(add_one, 10);
    println!("The answer to everything is {}", answer);

    // fn is a type instead of a trait and implements all closure traits : FnOnce, FnMut and Fn

    let numbers = vec![1, 2, 3];
    let strnum : Vec<String> = numbers.iter().map(|i| i.to_string()).collect();
    println!("strnum = {:?}", strnum);

    let numbers = vec![1, 2, 3];
    let strnum : Vec<String> = numbers.iter().map(ToString::to_string).collect();
    println!("strnum = {:?}", strnum);

    // Tuple struct enum variants use the () syntax for initialization and are actually implemented
    // as functions returning an instance that is constructed from their arguments, meaning that we
    // can actually use them this way :
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("All the statuses : {:?}", list_of_statuses);

    let returned_closure = returns_closure();
    println!("returned_closure(11) = {}", returned_closure(11));

    // This is a declarative macro
    let x = myvec![1, 2, 3];
    println!("I made a macro! {:?}", x);

    // Procedural macros are pretty complex and make my brain hurt in terms of implementation. I suggest we stay
    // away from writing them in the foreseeable future
    Pancake::hello_macro();
}

unsafe fn dangerous() {}

// For instance, we can't write the split_at_mut function this way
// cannot borrow `*slice` as mutable more than once at a time
/* fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    (&mut slice[..mid], &mut slice[mid..])
} */

fn split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    
    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// We can also have unsafe traits
unsafe trait YouFool {
    // Implement
}

unsafe impl YouFool for i32 {
    // Implement
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Small {
    fn bark(&self);
}

trait Big {
    fn bark(&self);
}

struct Doggo;

impl Small for Doggo {
    fn bark(&self) {
        println!("*bork bork*");
    }
}

impl Big for Doggo {
    fn bark(&self) {
        println!("*BARK*");
    }
}

impl Doggo {
    fn bark(&self) {
        println!("*bark bark*");
    }

    fn baby_name() -> String {
        String::from("Spot")
    }
}

trait Animal {
    fn baby_name() -> String;
}

impl Animal for Doggo {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Here we write a trait, a supertrait, that requires another trait, in our case the display trait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// We can't do the following as Point doesn't implement fmt::Display
// impl OutlinePrint for Point {}

struct DisplayPoint {
    x: i32,
    y: i32,
}

impl fmt::Display for DisplayPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for DisplayPoint {}

// Remember that we can only implement a trait if either the trait or the type are local to our crate
// We can get around this rule using the newtype pattern
// Let's implement Display on Vec<T>

struct WrapperVec(Vec<String>);

impl fmt::Display for WrapperVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn takes_long_type(f: Thunk) {
    f();
}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("bye!"))
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug)]
#[allow(dead_code)]
enum Status {
    Value(u32),
    Stop,
}

// Let's return a closure!
// The following way won't work because Fn is unsized
// fn returns_closure() -> Fn(i32) -> i32 {
// }
// Using dyn won't work either because we need to Box the type
// fn returns_closure() -> dyn Fn(i32) -> i32 {
// }
// We can write it this way
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

#[derive(HelloMacro)]
struct Pancake;