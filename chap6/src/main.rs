fn main() {
    // Let's see how to use enums now
    // We can combine enums and structs
    let localhost = IpAddr {
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1")
    };
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        addr: String::from("::1")
    };

    println!("localhost is {:?}", localhost);
    println!("loopback is {:?}", loopback);

    // Or we can be more concise and actually only use the enums but pass the string directly to them'
    // In thatm enums act more like algebraic types
    let localhost = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    println!("localhost is {:?}", localhost);
    println!("loopback is {:?}", loopback);

    // Enums elements can also contain different data, like so
    let localhost = IpAddr3::V4(172, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));

    println!("localhost is {:?}", localhost);
    println!("loopback is {:?}", loopback);

    // Now let's talk about somethion I love, the optional type, or Option in Rust
    // I love this type so much that I had implemented one in C++ (my favorite language back then) before it made it into the standard
    // The issue with optional in C++? So many things... First, it has an implicit conversion to bool, which can be implicitly converted
    // to a numeric type... which is like, the worst idea ever
    // It cause me so much trouble that I can't even recount
    // In Rust, T and Option<T> aren't the same types, so you HAVE to check them
    // For instance, this wouldn't work
    // let x1 = 5;
    // let x2 = Some(5);
    // let res = x1 < x2;

    println!("The value of a penny is {}", Coin::Penny.value_in_cents());
    println!("The value of a nickel is {}", Coin::Nickel.value_in_cents());
    println!("The value of a dime is {}", Coin::Dime.value_in_cents());
    println!("The value of a Quarter is {}", Coin::Quarter.value_in_cents());
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    addr: String
}

#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String)
}

#[derive(Debug)]
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25
        }
    }
} 