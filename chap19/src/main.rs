use std::slice;
use std::ops::Add;

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
}