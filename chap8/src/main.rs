fn main() {
    // We're gonna use quite a little bit of shadowing to demonstrate how to use different collections
    // Remember that doing 'let v' multiple times will simpl "shadow" the previous declaration
    
    // Here is a simple vector, it's empty
    let v: Vec<i32> = Vec::new();
    println!("Empty... {:?}", v);

    // Now we initalize a vector using the vec! macro
    let v = vec![1, 2, 3];
    println!("Hello vec! macro : {:?}", v);

    // We push some new elements and we print it again
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("New value after push : {:?}", v);

    // We're assigning in a new scope
    {
        v = vec![1, 2, 3, 4, 5];
        println!("New value without shadowing : {:?}", v);
    }

    // But the variable was in the outer scope, so it's kept
    println!("Values before shadowing in a new scope : {:?}", v);

    // Now we're shadowing in a new scope
    {

        let v: Vec<i32> = vec![5, 6, 7, 8]; // Shadowing from the outer scope
        println!("Values after shadowing in a new scope : {:?}", v);
    } // But here this new "shadowy" variable is dropped

    // Now ce can see that it has "reverted" back to the old values
    // It is because the v from the outer scope and the v from the inner scope were never the same variables
    println!("Values after shadowing in a new scope and getting out of the scope : {:?}", v);

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let third_element: &i32 = &v[2];
    println!("The third element element of the vector is {}", third_element);

    // We can also use match statements and the .get method to see if there's an element at a given index
    // For now there's a third element, print it
    match v.get(2) {
        Some(third_element) => println!("The third element of the vector is {}", third_element),
        None                => println!("There is no third element")
    }

    let v: Vec<i32> = vec![2];

    // No there isn't a third element, so print that there is not such element :'(
    match v.get(2) {
        Some(third_element) => println!("The third element of the vector is {}", third_element),
        None                => println!("There is no third element")
    }

    // Now we can use a more generic function to give us our answer 
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    print_nth_element_if_exists(&v, 2);
    let v: Vec<i32> = vec![2];
    print_nth_element_if_exists(&v, 2);

    // There is of a course a very important difference between the bracket operator and the .get method
    // The first will not perform bound checking and panic if we're going out of bounds whereas the other one will perform bound checking
    // and return and Option<T>

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    // For instance, this will panic!!!
    // let does_not_exist = &v[100]; 
    let does_not_exist = v.get(100);

    match does_not_exist {
        Some(elem) => println!("The 100th element of the vector is {}", elem),
        None       => println!("There is not 100th element")
    }

    // It is also worth mentioning that if you're taking a reference to a particular element in the collection,
    // then it will be invalidated by a mutable operation on the aforementioned collection

    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    println!("Just casually printing the first element before it is invalidated : {}", first);
    v.push(0);
    // And now the following is not gonna work because 'first' has been invalidated
    // println!("Not so casually printing the first element after it is invalidated : {}", first);
}

fn print_nth_element_if_exists(v: &Vec<i32>, n: usize) {
    let ordinal_suffix: &'static str = get_ordinal_suffix(n + 1);

    match v.get(n) {
        Some(elem) => println!("The {}{} element of the vector is {}", n + 1, ordinal_suffix, elem),
        None       => println!("There is not {}{} element", n + 1, ordinal_suffix)
    }
}

fn get_ordinal_suffix(n: usize) -> &'static str {
    if let 0..=9 = n / 10 {
        "th"
    }
    else
    {
        _ => match n % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th"
        }
    }
}