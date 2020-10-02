use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::fmt;
use std::io;
use std::io::Read;

fn main() {
    // Oye oye, let's talk about panicking
    // Upon panicking, stack unwinding will be performed, this is a lot of work and can lead to bigger binaries
    // If we want to avoid that, we can enable the following in our Cargo.toml:
    // [profile.release]
    // panic = 'abort'
    // Handy!

    // The simplest way to panic is to use the panic! macro:
    // panic!("CRASH AND BURN!!! (Well, maybe not burn, ya know, fire safety and all...)");
    // By default, no stacktrace will be produced, we need to set the env variable RUST_BACKTRACE to 1
    // The debug symbols need to be actived (they're by default by using cargo build and cargo run without the --release option)

    // When it comes to out of bounds, the same happens
    let v = vec![1, 2, 3];

    println!("{:?}", v);
    // The following will panic of course
    // v[99];
    // If we don't want to panic, we can use the .get method that will return an option instead

    let elem_not_exists = v.get(99);
    let elem_exists = v.get(1);

    only_print_if_exists(elem_not_exists);
    only_print_if_exists(elem_exists);
    
    // As we can see, elem_not_exists won't be printed but no panic occured
    // We do use the same kind of error handling while reading from the filesystem but using the Result enum
    // enum Result<T, E> {
    //   Ok(T),
    //   Err(E)
    // }

    print_file_access_result("exist.txt");
    print_file_access_result("does_not_exist.txt");

    // We can also panic if we need to. Two ways :
    // let f = File::open("does_not_exist.txt");
    // let f = match f {
    //   Ok(file) => file,
    //   Err(error) => panic!("Problem opening the file: {:?}", error),
    // }
    //           
    // or        
    //
    // let f: File = File::open("does_not_exist.txt").unwrap();
    // Unwrap will panic if the return is Err
    // Now, let's say that we want to know more about the error type, so that if the file is not found we create it

    let f = File::open("exist.txt");
    let f = match f {
        Ok(file) => file,
        Err(err_msg) => match err_msg.kind() {
            ErrorKind::NotFound => match File::create("exist.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    println!("succesfylly got a handle on the file '{:?}'", f);

    // Let's make that into a function now...
    let f = get_file_handle_or_create("to_be_created.txt");
    println!("succesfylly got a handle on the file '{:?}'", f);
    let deletion_result = fs::remove_file("to_be_created.txt");

    match deletion_result {
        Ok(_) => println!("Deletion successful"),
        Err(error) => println!("Deletion failed with error {}", error),
    }
    
    let f = get_file_handle_or_create("exist.txt");
    println!("succesfylly got a handle on the file '{:?}'", f);
    
    // There's a better way to write this function though, let's see...
    let f = get_file_handle_or_create2("to_be_created.txt");
    println!("succesfylly got a handle on the file '{:?}'", f);
    let deletion_result = fs::remove_file("to_be_created.txt");
    
    match deletion_result {
        Ok(_) => println!("Deletion successful"),
        Err(error) => println!("Deletion failed with error {}", error),
    }
    
    let f = get_file_handle_or_create2("exist.txt");
    println!("succesfylly got a handle on the file '{:?}'", f);
    
    // A nice shortcut for panicking if we can't open a file for example is to use the .unwrap method of the Result<T, E> type
    File::open("exist.txt").unwrap();
    // The code below will panic
    // File::open("does_not_exist.txt").unwrap();

    // We can also provide a panic error message using the .expect method
    File::open("exist.txt").expect("AAAAAAAAAAAAAAAAAAAAAAAH!");
    // File::open("does_not_exist.txt").expect("AAAAAAAAAAAAAAAAAAAAAAAH!");

    // Let's see how error propagation works now with the function read_username_from_file
    let res = read_username_from_file("username.txt").expect("Couldn't read username");
    println!("Ahah! I found your username, {}", res);

    // The code below will of course panic
    //let res = read_username_from_file("bogus_filename.txt").expect("Couldn't read username");

    // The pattern of propagating error is EXTREMELY common, and as such, Rust provide a shorthand for doing so instead of writing match statements
    // all over the place. Behold the question mark operator!
    // It works in a very simple way, it's put at the end of an expression, if the result contains a value it will simply return it, otherwise it will
    // return from the whole function with an error (of course stored in a result)
    // The new function is much shorter and sweeter

    let res = read_username_from_file2("username.txt").expect("Couldn't read username");
    println!("Ahah! I found your username, {}", res);

    // The code below will of course panic
    //let res = read_username_from_file2("bogus_filename.txt").expect("Couldn't read username");
    
    // Please note that each and every error returned with the ? operator will go through the from function from the trait From of the std library
 
    let res = read_username_from_file3("username.txt").expect("Couldn't read username");
    println!("Ahah! I found your username, {}", res);

    // The code below will of course panic
    //let res = read_username_from_file3("bogus_filename.txt").expect("Couldn't read username");

    let res = read_username_from_file_definitive_edition("username.txt").expect("Couldn't read username");
    println!("Ahah! I found your username, {}", res);

    // The code below will of course panic
    //let res = read_username_from_file_definitive_edition("bogus_filename.txt").expect("Couldn't read username");

    // It is also important to note that the ? operator can only be used in functions that return a Result<T, E>, which makes sense considering that
    // it's made to work like a match on a result
    // For instance, the code below would cause a compilation error if uncommented, explaining us the reason of our stupidity
    // let f = File::open("hello.txt")?;

    // Interstingly enough, the main function can perfectly return a Result type like so: 
    // fn main() -> Result<(), Box<dyn Error>> {            
    //   let f = File::open("hello.txt");
    //
    //   Ok(())
    // }

    // The code above is perfectly valid Rust. Awfully neat ain't it?
}

fn only_print_if_exists<T: fmt::Display>(x: Option<T>) {
    if let Some(val) = x {
        println!("{}", val);
    }
}

fn print_file_access_result(filename: &str) {
    match File::open(filename) {
        Ok(_) => println!("File can be accessed"),
        Err(err_msg) => println!("Error while opening file : {}", err_msg)
    }
}

// Cheesy bugger c:
fn get_file_handle_or_create(filename: &str) -> File {
    match File::open(filename) {
        Ok(file) => file,
        Err(err_msg) => match err_msg.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(fc) => {
                    println!("File not found, but don't worry, we created it for you :)");
                    fc
                },
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    }
}

// Less verbose cheesy bugger c:
fn get_file_handle_or_create2(filename: &str) -> File {
    File::open(filename).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(filename).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error)
            })
        }
        else {
            panic!("Problem opening the file: {:?}", error)
        }
    })
}

// Demonstrate error propagation in Rust, not the best way though
fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Demonstrate error propagation in Rust with the fancy ? operator
fn read_username_from_file2(filename: &str) -> Result<String, io::Error> {
    let mut f = File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

}

// Demonstrate error propagation in Rust with the fancy ? operator, and even better this time around by chaining things together
fn read_username_from_file3(filename: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s)
}

// Or just cut through the fat and use the fs::read_to_string method
fn read_username_from_file_definitive_edition(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}