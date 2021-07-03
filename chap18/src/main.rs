fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    }
    else if is_tuesday {
        println!("Tuesday is a green day!");
    }
    else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        }
        else {
            println!("Using purple as the background color");
        }
    }
    else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("vec[{}] = {}", index, value);
    }

    let (_x, _y, _z) = (1, 2, 3);

    // The following won't wok though
    // expected a tuple with 3 elements, found one with 2 elements
    // let (x, y) = (1, 2, 3);

    let point = (3, 5);
    print_coordinates(&point);

    // Refutable vs irrefutable patterns
    // An irrefutable pattern is a pattern that matches for every value passed, a refutable pattern is... well, the other way around
    // if let Some(x) = opt_value is an example of a refutable pattern
    // On the othe hand, function parameters, let statement and for loops only accept irrefutable pattern, otherwise the program
    // couldn't use the values in any meaningful manner
    // though if let and while let expressions accept refutable and irrefutable patterns, the compiler will warn against irrefutable
    // patterns because they're by definition designed to handle a possible failure

    // The following code would not compile
    // let Some(x) = some_option_value;

    // We would need to use the following :
    let some_option_value: Option<i32> = None;
    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    // With the code below, Rust would warn us against using an irrefutable pattern in an if let statement
    // if let x = 5 {
    //     println!("{}", x)
    // }
    
    // Indeed, such a statement is useless and could be readiy replaced by a simple if statement

    println!("{}", matching_literal(2));
    println!("{}", matching_literal(42));

    // We can also match named variables

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // Here, y is a new variable unrelated to the one declared above
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    println!("{}", matching_literal2(2));
    println!("{}", matching_literal2(42));

    println!("{}", matching_literal3(2));
    println!("{}", matching_literal3(42));

    println!("a is an {}", ascii_letter_lateness('a'));
    println!("y is a {}", ascii_letter_lateness('y'));

    // We can also use pattern matching to perform destructuring

    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis at ({}, {})", x, y),
    }

    // Also on enums 
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }

    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to rgb = ({}, {}, {})", 
            r, g, b
        ),
        Message2::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hsv = ({}, {}, {})",
            h, s, v
        ),
        _ => (),
    }

    let ((_feet, _inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("{}, {}", x, y);

    // But we can also ignore values in a pattern
    foo(3, 4); // The first argument will be ignored

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("Setting value is now : {}", setting_value.unwrap());

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    // There's a difference between using a heading underscore for unused variables and a simple underscore

    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("Found a string");
    }

    // Now we couldn't do that
    // println!("{:?}", s)
    // The value has been moved into _s

    // But when using a simple underscore, it's different
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("Found a string");
    }

    println!("{:?}", s);

    // This time it works as the value isn't being moved to _. Indeed, _ is not a variable
    // We can also ignore the remaining parts of a value using ..

    let origin = Point3d::origin();

    match origin {
        Point3d { x, y: _, z: _ } => println!("x is {}", x),
    }

    // It is, however, much quicker to do the following
    match origin {
        Point3d { x, .. } => println!("x is {}", x),
    }

    // We can also ignore many values in between
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => println!("first = {} and last = {}", first, last),
    }

    // That being said, it needs to be unambiguous, for instance the following code wouldn't work :
    // match numbers {
    //     (.., second, ..) => println!("second = {}", second)
    // }
    
    // Instead, we would have to write the following
    match numbers {
        (_, second, ..) => println!("second = {}", second)
    }
 
    // We can also add extra conditionals in match patterns
    // We call these extra conditions "match guards"
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("just a number {}", x),
        None => {}
    }

    // It also allows us to solve the issue of the pattern shadowing our external variable
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if y == n => println!("Matched, y = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    // If we specify multiple patterns, the match guard will apply to all of them
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    let msg = SimpleMessage::Hello{ id: 5 };

    match msg {
        SimpleMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        SimpleMessage::Hello {
            id: 10..=12
        } => println!("Found an id in another range"),
        SimpleMessage::Hello{ 
            id 
        } => println!("Found some other id: {}", id),
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Coords : {}, {}", x, y);
}

fn matching_literal(x: i32) -> &'static str {
    match x {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "anything",
    }
}

fn matching_literal2(x: i32) -> &'static str {
    match x {
        1 | 2 => "one or two",
        3 => "three",
        _ => "anything",
    }
}

fn matching_literal3(x: i32) -> &'static str {
    match x {
        1..=5 => "one through two",
        _ => "anything",
    }
}

fn ascii_letter_lateness(x: char) -> &'static str {
    match x {
        'a'..='j' => "early ASCII letter",
        'k'..='z' => "late ASCII letter",
        _ => "not an ASCII letter"
    }
}

struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[allow(dead_code)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[allow(dead_code)]
enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

#[allow(dead_code)]
struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3d {
    fn origin() -> Point3d {
        Point3d { x: 0, y: 0, z: 0 }
    }
}

enum SimpleMessage {
    Hello { id: i32 },
}