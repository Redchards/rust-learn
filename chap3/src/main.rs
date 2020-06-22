fn main() {
    println!("Hello, world!");
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    shadow1();
    println!("This value has been shadowed initially: {}", shadow2());
    
    println!("This value has been parsed from 42: {}", parse_ex("42"));

    let x2: i32 = 100_000_000;
    println!("We have a fancy numbur which is: {}", x2);

    let z_math = 'ℤ';

    println!("MATH SYMBOL => {}", z_math);

    let tup: (i32, f64, u8) = (600, 6.4, 1);
    println!("We can print that!?!?! {:?}", tup);

    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("We can print that!?!?! {:?}", my_array);

    let my_array2 = [3; 5];
    println!("We can print that!?!?! {:?}", my_array2);

    println!("Add: 3 + 5 = {}", add(3, 5));
    println!("plus one {}", plus_one(5));

    let cond: bool = true;
    let my_num = if cond { 5 } else { 6 };
    println!("condition result: {}", my_num);
}

fn shadow1() {
    let x = 5;
    let x = x + 1;
    let x = x + 1;

    println!("The value of x is: {}", x);
}

fn shadow2() -> usize {
    let spaces = "     ";
    let spaces = spaces.len();

    spaces
}

fn parse_ex(x: &str) -> i32 {
    x.parse().expect("Not a number")
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn plus_one(x: i32) -> i32 {
    x + 1
}