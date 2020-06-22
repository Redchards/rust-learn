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

    // The Option type is awesome!!!!
    let x = 5;
    let x1 = plus_one(Some(x));
    let x2 = plus_one(None);
    
    println!("x1 is {:?}, and x2 is {:?}", x1, x2);

    // Let's look at the match placeholder now

    let some_value = 5;
    match some_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        _ => println!("Unknown!!")
    }

    // If/Let shorthand for non-exhaustive matches
    // Before shorthand 
    let some_value = Some(5);
    match some_value {
        Some(3) => println!("Yay!"),
        _ => println!("Ohnoes...")
    }

    // After shorthand
    if let Some(3) => {
        println!("Yay!")
    }

    // Another example
    // Before shorthand
    let mut count = 0;
    let coin = Coin::Quarter;
    match coin {
        Coin::Quarter => println!("A quarter"),
        _ => count += 1
    }

    // After shorthand
    if let Coin::Quarter = coin {
        println!("A quarter");
    }
    else {
        count += 1;
    }

    // In this case it isn't shorter per se, but it definitely is more readable
    // It clearly shouldn't be used for exhaustive matching, as we completely lose 
    // the ability to handle more than these cases
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

fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        Some(i) => Some(i + 1),
        None => None
    }
}