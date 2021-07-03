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
}
