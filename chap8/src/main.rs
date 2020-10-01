use std::collections::HashMap;

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
    // It is very similar to iterator invalidation in C++, but here the compiler will prevent us from using an invalid "iterator", or in this
    // case reference to an element. Schwwwwwwweeeet
    // println!("Not so casually printing the first element after it is invalidated : {}", first);
    
    // Now let's try it with actual iterators right
    // let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut it = v.iter();

    // Here .unwrap can of course panic, but we know what we're doing in this example ;)
    println!("Hello, printing value from an iterator : {}", it.next().unwrap());
    println!("Vector is : {:?}", v);

    // Now what happens if we push another element?
    // Well, the answer is that we can't, because we have to be able to mutably borrow v, when we borrowed an immutable reference to it with v.iter()
    // So we can't push anything to the vector while an iterator is in scope
    // v.push(0);

    // Now let's iterate over a vector's value
    let mut v: Vec<i32> = vec![100, 32, 57];
    for x in &v {
        println!("Printing one element : {}", x);
    }

    // Let's modify the vector by adding 50 to every element
    for x in &mut v {
        *x += 50;
    }

    // Now let's print it using enumerate
    // Very sweet
    for (i, x) in v.iter().enumerate() {
        let ordinal_suffix: &'static str = get_ordinal_suffix(i + 1);
        println!("The {}{} element of the vector is {}", i + 1, ordinal_suffix, x)
    }

    // Now let's see heterogeneous collections using... enums! That's right, enums are simply tagged unions! Suss!
    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // And let's print this row... how would we do that ?
    println!("Printing a nice heterogeneous collection... as an spreadsheet row!");

    for cell in row {
        print!("| ");
        cell.print_value();
        print!(" ");
    }
    println!("|");
    
    // Next we will play with character strings a little, namely the types String and str
    // String is the owned type and str is the borrowed type, only str is a natvie built-in type.

    let init_str = "Hello World!";
    let s = init_str.to_string();
    println!("Newly created string : {}", s);

    // Also works directly on string literals of course
    let s = "Hello World!".to_string();
    println!("Newly created string : {}", s);

    // Another way is to use the "from" static method from the class String
    let s = String::from("Hello World!");
    println!("Newly created string : {}", s);

    // Important note : strings are utf-8 encoded, so we can display any character in utf-8
    let frog_str = "Enchanté, nous sommes des grenouilles !";
    println!("{}", frog_str);

    // We can of course grow a string using the .push_str or the .push methods for instance
    let mut s = String::from("Hello ");
    s.push_str("World");
    s.push('!');
    println!("{}", s);
    
    // An example of string concatenation
    let s1 = String::from("Hello ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;
    println!("{}", s3);

    // Note that the lhs will be moved (stolen reference) and can of course no longer be used afterwards
    // The following won't compile:
    // println!("{}", s1);

    // We can also chain concatenations like in any otehr language
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
    
    // Here again, only s1 is moved
    // A better method would be to use the format! macro though
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
    // Note that it doesn't take ownership of any of the parameters as it builds an entirely new string and doesn't reuse the memory
    // of any of the parameters, so no move occurs

    // There's a drawback to everything being encoded in utf-8 though (or is it? Not at all imo), and it's that we can't index strings
    // For instance, the following would not compile:
    // let s = String::from("hello");
    // let h = s[0];
    // That being said, we can access slices of strings
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    // Here every letter is represented by two bytes, what would happen if we were to slice "in between", like with &str[1..3]
    // Well, the following code will panic with "not a char boundary"
    // let s = &hello[1..3];
    // println!("{}", s);

    // To iterate over a string we can either use the method .char or the method bytes, which are pretty self explanatory
    println!("Printing chars:");
    for c in hello.chars() {
        println!("{}", c);
    }


    println!("Printing bytes:");
    for b in hello.bytes() {
        println!("{}", b);
    }

    // So strings are pretty complicated in Rust, but this complexity is inherent to the object and in return we get correct utf-8 string
    // handling by default!

    // Now let's have som hashmap silly fun
    // Notice the "use" at the beginning of the file: it brings the HashMap collection in our module
    let mut scores = HashMap::new();

    scores.insert(String::from("Billy"), 10);
    scores.insert(String::from("Jimmy"), 16);
    
    // Another way of filling them up is to use the "collect" method
    let players: Vec<String> = vec![String::from("Billy"), String::from("Jimmy")];
    let final_scores: Vec<i32> = vec![10, 16];
    let mut scores: HashMap<_, _> = players.into_iter().zip(final_scores.into_iter()).collect();
    println!("{:?}", scores);

    // The HashMaps are owner of the types that don't implement the Copy trait, like Strings for example
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(field_name, field_value);
    // You can't use these values anymore, the code below would not compile:
    // println!("{}:{}", field_name, field_value);

    // Retrieving a value from a HashMap is of course rather straightforward and indeed we have the luxury of Options, that's amazing!
    let billy_score = scores.get("Billy");

    if let Some(x) = billy_score {
        println!("Billy's score is {}", x);
    }
    else {
        println!("There's no such player!");
    }

    // We can also iterate over HashMaps
    for x in &scores {
        print_score(x);
    }

    // The .insert method allows us to update values
    scores.insert(String::from("Billy"), 25);

    for x in &scores {
        print_score(x);
    }

    // We can also insert only if there's not entry in the table with a given key
    scores.entry(String::from("Tommy")).or_insert(50);
    scores.entry(String::from("Billy")).or_insert(50);

    // As we can see, Billy's score remains the same
    for x in &scores {
        print_score(x);
    }
    
    // Note that the .or_insert method returns a mutable reference to the entry, which means that we can mutate it
    // Let's count word occurrences for instance

    let text = "hello world wonderful world hello hello and hello";
    let mut occ_map = HashMap::new();

    for word in text.split_whitespace() {
        let e = occ_map.entry(word).or_insert(0);
        *e += 1;
    }

    println!("{:?}", occ_map);

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("Mean of {:?} = {}", v, vec_mean(&v));
    println!("Median of {:?} = {}", v, vec_med(&v));
    println!("Median of {:?} = {}", v, vec_mode(&v));

    let s = "Hello I am gonna be converted to pig latin";
    println!("{}", pig_latin(s));
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
        match n % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th"
        }
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

impl SpreadsheetCell {
    fn print_value(self) {
        match self
        {
            SpreadsheetCell::Int(x) => print!("{}", x),
            SpreadsheetCell::Float(x) => print!("{}", x),
            SpreadsheetCell::Text(x) => print!("{}", x),
        }
    }
}

fn print_score((player, score): (&String, &i32)) {
    println!("{}'s score is {}", player, score);
}

// Stupid simple functions that the rust book asks you to write at the end of the chapter
// They're purposefully not using anything already present in the standard library

fn vec_mean(v: &Vec<i32>) -> f64 {
    let mut sum: i32 = 0;

    for x in v {
        sum += x;
    }

    let sum = sum as f64;

    return sum / (v.len() as f64);
}

fn vec_med(v: &Vec<i32>) -> i32 {
    let mut v_clone = v.clone();
    v_clone.sort();

    return v_clone[v_clone.len() / 2];
}

fn vec_mode(v: &Vec<i32>) -> i32 {
    let mut mode_map: HashMap<i32, i32> = HashMap::new();

    for x in v {
        let e = mode_map.entry(*x).or_insert(0);
        *e += 1;
    }

    let mut best = 0;
    for (x, num_occ) in mode_map {
        if num_occ > best {
            best = x;
        }
    }

    return best;
}

// Ok, for this one I just couldn't resist using iterators... So tempting
fn pig_latin(s: &str) -> String {
    let mut result = String::new();

    for substr in s.split_whitespace() {
        let mut tmp: String = substr.chars().into_iter().skip(1).collect();
        tmp.push('-');
        tmp.push(substr.chars().next().unwrap());
        tmp.push_str("ay");

        result.push_str((tmp + " ").as_str());
    }

    return result;
}