use std::cmp;
use std::cmp::PartialOrd;
use std::fmt;
use std::fmt::{Display, Formatter};

fn main() {
    // Now let's talk about generics. Yay!
    // As we can see, for now we can't use our function max_elemt both for char and i32, we have to duplicate the exact same logic
    // Also Rust doesn't natively support overlaoding (though it can be simulated using traits)
    // At first it struck me as odd: function overloading is one thing I loved moving from C to C++, not having to write functions
    // with names such as add_v3f, add_v3d, add_v4f and add_v4d that functionally perform the same operations
    // The justification for Rust not supporting overloading is as follows:
    //
    // *****************************************************************************************************************************
    // * Overloading. Rust does not support traditional overloading where the same method is defined with multiple signatures. But *
    // * traits provide much of the benefit of overloading: if a method is defined generically over a trait, it can be called with *
    // * any type implementing that trait. Compared to traditional overloading, this has two advantages. First, it means the       *
    // * overloading is less ad hoc: once you understand a trait, you immediately understand the overloading pattern of any APIs   *
    // * using it. Second, it is extensible: you can effectively provide new overloads downstream from a method by providing new   *
    // * trait implementations.                                                                                                    *
    // *****************************************************************************************************************************
    //
    // I spent a little more time reading on the justification for not supporting function overloading because you ain't taking 
    // function overloading away from me without a good justification!
    // A very compelling reason and maybe the only really compelling one that I found was that function overloading makes it
    // more difficult to know the real behavior of a function. Though it is not function overloading, we all remember the infamous
    // ballad of the std::vector<bool> specialization which acts exactly like a bitset and not a vector of boolean, making its
    // behavior different. A very simple example could be something like this (though it would be more readily implemented as a 
    // method:
    // void plus_one(int& x) { x += 1; }
    // void plus_one(std::vector<int>& x) { x.push_back(1); }
    //
    // Of course writing such a function doesn't really make sense, but nothing prevents us from doing so in C++
    // So in Rust we can't have two functions with the same name that act different which is honestly a pretty good reason to not
    // have overloading, but unlike C which does not give us any mechanism to handle such a situation where we want to operate 
    // on a type abiding by a particular concept other than simply renaming the function or use *shivers* ... macros... Well
    // Rust gives us a nice tool in the form of traits as we will see a little bit later
    // Another good reason is to avoid monomorphization-time errors. Monomorphization is a fancy word to express a very simple idea:
    // In Rust, like in C++, generics produce code specific to an instantiation using a particular type. So for instance, if we have
    // a generic function foo<T> with unbounded T, and then we call it with an i32 and a Vec<i32>, code for both calls will be
    // generated (the "template" will be instantiated two times)
    // If we had function overloading, it would be much harder to determine whether an instantiation is correct or not. Take this
    // example:
    // fn foo<T>(x: T) { bar(x) }
    //
    // What do we know about bar? Well, first, it must return a (), but also it must accept an unbounded generic type as T is
    // unbounded and we aren't gonna compromise. If bar does not accept an unbounded type, then we know that we can't call it in this
    // context as there exists no implementation of bar that will satisfy our constraints. But now, what if bar could be overloaded,
    // what would happen? Simply put, we would have to make sure that for each and every instantiation there is a valid overload of 
    // bar in scope
    // Oh no, my precious Rust is taking inspiration from this ugly sob C++ D: In this case it's actually very good as generics
    // through type erasure always induce performance loss and less flexibility in generic code. Now, Rust disallowing function
    // overloading makes our life easier as we don't have to peform very extensive and complex name lookup (ADL and friends, oh my...)
    // I will say that I'm pleasantly surprised by Rust's generics, despite them inducing a compilation overhead. In the world of 
    // generics it's a "pick your poison" kind of deal, do you want slower compile time but virtually zero runtime overhead generics
    // or do you prefer virtually free generics at compile time but them having a runtime overhead (and less flexibility as well)?
    // Of course Rust was gonna choose the former and I'm very pleased with that
    //
    // Ok now let's go back to our simple code...

    let num_list = vec![32, 24, 525, 23, 14, 156];
    println!("The largest element in {:?} is {}", num_list, max_elem_i32(&num_list));
    
    // Let's just have fun with slices because it's so darn neat 
    println!("The largest element in {:?} is {}", &num_list[3..], max_elem_i32(&num_list[3..]));

    let char_list = vec!['H', 'e', 'l', 'l', 'o', '!'];
    println!("The largest element in {:?} is {}", char_list, max_elem_char(&char_list));


    println!("The largest element in {:?} is {}", num_list, max_elem(&num_list));
    println!("The largest element in {:?} is {}", char_list, max_elem(&char_list));

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    // Of course T is a unique type, we can't mix types together
    // The following line would not compile
    // let wrong_point = Point { x: 1, y: 4.0 };

    println!("{:?}", integer_point);
    println!("{:?}", float_point);
    println!("{}", integer_point.x());
    println!("{}", float_point.x());

    println!("{:?}", float_point.mixup(integer_point));

    let right_point = MixPoint { x: 1, y: 4.0 };
    let float_point = MixPoint { x: 1.0, y: 4.0 };

    println!("{:?}", right_point.mixup(float_point));
    
    let p1 = Point { x: 1.0, y: 4.0 };
    let p2 = Point { x: 2.0, y: 7.35 };
    println!("Distance between {:?} and {:?} = {}", p1, p2, p1.distance_fron(&p2));

    // Example traits
    let tweet = Tweet {
        username: String::from("mother_horse_eyes"),
        content: String::from(
            "what awaits us beyond the curtains?"
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // Slightly politicized example
    let article = NewsArticle {
        headline: String::from("Orange man bad"),
        location: String::from("The MSM"),
        author: String::from("Anton McCuck"),
        content: String::from("Orange man got COVID, ahah, orange man dead"),
    };

    println!("1 new tweet: {}", article.summarize());

    // This one everyone will agree with, most likely anyway
    let tabloid_article = TabloidNewsArticle {
        headline: String::from("THIS SINGLE MOM OF 3 FOUND A REVOLUTIONARY WAY TO LOOK 10 YEARS YOUNGER, DOCTORS HATE HER"),
        location: String::from("BuzzBizzFeedzies"),
        author: String::from("CoolLife69"),
        content: String::from("Amazing trick, but first a shitload of ads!!!"),
    };

    println!("1 new tweet: {}", tabloid_article.summarize());

    notify(&tweet);
    notify2(&tweet);
    dual_notify(&tweet, &article);
    // Can't do the following:
    // dual_notify_bound(&tweet, &article);

    dual_notify_bound(&tweet, &tweet);

    notify_spam(&tabloid_article);
    notify_spam2(&tabloid_article);
    notify_spam3(&tabloid_article);

    returns_summarizable();

    let p1 = Pair::new(3, 5);
    p1.cmp();
    p1.cmp_display();

    // The following won't compile because the type Tweet does not implement the proper traits
    // let p2 = Pair { first: &tweet, second: &tweet };
    // p2.cmp();
    // p2.cmp_display();

    // Now let's move onto Lifetimes
    // This time, instead of wanting to know if a type has the correct behavior, we want to determine the lifetime of a particular resource, which
    // is the scope in which a reference to it is valid
    // We've already seen the following example

    // let r;
    // 
    // {
    //    let x = 5;
    //    r = &x;
    //}
    // println!("r: {}", r);

    // Here r lives longer than x (the borrowed value) because it's declared in the outer scope, Rust uses a borrow checker to determine that this code
    // is indeed invalide
    // Notice that we have not initialized r, it doesn't mean that r is null, it actually can't be used before being initialized

    // The borrow checker can actually be understood using "lifetime annotations"
    // {
    //     let r;                // ---------+-- 'a
    //                           //          |
    //     {                     //          |
    //         let x = 5;        // -+-- 'b  |
    //         r = &x;           //  |       |
    //     }                     // -+       |
    //                           //          |
    //     println!("r: {}", r); //          |
    // }                         // ---------+

    // The lifetime of r has been annotate with 'a and the lifetime of x with 'b, the inner 'b block is much smaller than the outer 'a lifetime bloock
    // At compile time, we will compare the two lifetimes and see that r has a lifetime denoted by 'a and refers to x that has a lifetime denoted by 'b
    // As 'b is shorter than 'a, the program is simply rejected, the referenced borrowed variable doesn't live long enough

    // The following is fine though

    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+

    // By applying the same logic, it is easy to see that this code is correct, at least as long as the lifetime consistency goes
    // Sometimes it is imperative to annotate our lifetimes to make sure that we're not doing anything wrong

    // We can also define generic lifetime annotations ourselves, see the "longest" function

    println!("Longest = {}", longest("ZA", "WARUDO"));
    
    let za = String::from("ZA");
    let warudo = String::from("WARUDO");
    println!("Longest = {}", longest(&za, &warudo));
    
    // Still ok, both arguments live as long as the generic lifetime
    let za = String::from("ZA");
    {
        let warudo = String::from("WARUDO");
        println!("Longest = {}", longest(&za, &warudo));
    }

    // The code below wouldn't work though
    // let za = String::from("ZA");
    // let result;
    // {
    //     let warudo = String::from("WARUDO");
    //     result = longest(&za, &warudo);
    // }
    // println!("Longest = {}", result);
    
    // We can get structs to hold references, but one more, we need to annotate the lifetime of every reference that the structure holds
    let fairy_tale = String::from("Once upon a time, there was a young boy");
    let first_sentence = fairy_tale.split(',').next().expect("Couldn't find a split!");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    excerpt.print();

    println!("{}", first_word(&fairy_tale));

    // Lifetime annotations on function or method parameters are called input lifetimes and lifetimes on return values are output lifetimes
    // There are three rules for lifetime elision : 
    // - At first all parameters are automatically annotated with their respective lifetimes
    // - If there's exactly one input lifetime, the output lifetimes will be the same
    // - If it's a class method (with input &self or &mut self), the outputs will be assigned the lifetime of self

    longest_with_announcement(&za, &warudo, "That's all folks");
}       

// VERBOTTEN!!!
// fn max_element(x: &[i32]) -> i32 {...}
// fn max_element(x: &[char]) -> char {...}

// So we have to do this for now
// The code is of course kind of unsafe, but we won't be bothered with it as it's a simple example
fn max_elem_i32(l: &[i32]) -> i32 {
    let mut curr_max = l[0];

    for x in l {
        if *x > curr_max {
            curr_max = *x;
        }
    }

    curr_max
}

// And now for the char... God, that is the EXACT SAME CODE (I actually just copy/pasted it)
// So much for avoiding repetition, DRY is rolling in its textbook rn
fn max_elem_char(l: &[char]) -> char {
    let mut curr_max = l[0];

    for x in l {
        if *x > curr_max {
            curr_max = *x;
        }
    }

    curr_max
}

// We could try to define the function as follows:
// fn max_elem<T>(l: &[T]) -> &T {  // Returning a ref here because the element might not be Copy
//    let mut curr_max = &l[0];
//
//    for x in l {
//        if x > curr_max {
//            curr_max = x;
//        }
//    }
//
//    curr_max
// }
// Rust will advise us to restrict the parameter T to std::cmp::PartialOrd. so let's do that

fn max_elem<T:cmp::PartialOrd>(l: &[T]) -> &T {  // Returning a ref here because the element might not be Copy
   let mut curr_max = &l[0];

   for x in l {
       if x > curr_max {
           curr_max = x;
       }
   }

   curr_max
}

// Using generics in structs is very straightforward as well
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// And in method definitions
impl<T> Point<T> {
    fn x(&self) -> &T {
        return &self.x;
    }
}


#[derive(Debug)]
struct MixPoint<T, U> {
    x: T,
    y: U
}

impl<T1, U1> MixPoint<T1, U1> {
    fn mixup<T2, U2>(self, other: MixPoint<T2, U2>) -> MixPoint<T1, U2> {
        MixPoint {
            x: self.x,
            y: other.y,
        }
    }
}

impl<T1> Point<T1> {
    fn mixup<T2>(self, other: Point<T2>) -> MixPoint<T1, T2> {
        MixPoint {
            x: self.x,
            y: other.y,
        }

    }
}

// Implementing a method only on f64
impl Point<f64> {
    fn distance_fron(&self, other: &Point<f64>) -> f64 {
        let x = other.x - self.x;
        let y = other.y - self.y;

        (x*x + y*y).sqrt()
    }
}

// Here's a little trait
pub trait Summarizable {
    fn summarize(&self) -> String;
}

// As we can see, implementing a trait on a type is fairly straightforward
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// We can also provide a default implementation
pub trait SummarizableWithDefault {
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

pub struct TabloidNewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl SummarizableWithDefault for TabloidNewsArticle {}
impl Display for TabloidNewsArticle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}...", &self.content[..20])
    }
}

// We can simply take a trait as parameter
fn notify(item: &impl Summarizable)  {
    println!("Breaking new! {}", item.summarize());
}

// The syntax above is a shorthand for the following one
fn notify2<T: Summarizable>(item: &T) {
    println!("Breaking new! {}", item.summarize());
}

// The "trait bound syntax" is not useless though, consider the following case
fn dual_notify(item1: &impl Summarizable, item2: &impl Summarizable) {
    println!("Breaking new! {}", item1.summarize());
    println!("Breaking new! {}", item2.summarize());
}

// Using this syntax, we can't force the types to be the same though, we have to use the trait bound syntax
fn dual_notify_bound<T: Summarizable>(item1: &T, item2: &T) {
    println!("Breaking new! {}", item1.summarize());
    println!("Breaking new! {}", item2.summarize());
}

// We can also specify multiple traits using the + operator
fn notify_spam(item: &(impl SummarizableWithDefault + Display)) {
    println!("Spam found! {}", item.summarize());
    println!("Peek content => {}", item);
}

// Which is of course equivalent to the following syntax
fn notify_spam2<T: SummarizableWithDefault + Display>(item: &T) {
    println!("Spam found! {}", item.summarize());
    println!("Peek content => {}", item);
}

// We can also use the handy 'where' syntax
fn notify_spam3<T>(item: &T)
    where T: SummarizableWithDefault + Display
{
    
    println!("Spam found! {}", item.summarize());
    println!("Peek content => {}", item);
}

// We can also write a function that returns a type that implements a traits, that being said, we can't return
// either one type or another, so the following function is correct for instance: 
fn returns_summarizable() -> impl Summarizable {
    Tweet {
        username: String::from("ProbablySomethingQuirky"),
        content: String::from(
            "Something very witty and imaginative, because twitter users are big brained"
        ),
        reply: false,
        retweet: false,
    }
}

// But this one is not allowed, giving us an error 'expected struct Tweet, found struct NewsArticle
// if and else have incompatible types
// Quite exlicit honestly
/* fn returns_summarizable_switch(switch: bool) -> impl Summarizable {
    if switch {
        Tweet {
            username: String::from("ProbablySomethingQuirky"),
            content: String::from(
                "Something very witty and imaginative, because twitter users are big brained"
            ),
            reply: false,
            retweet: false,
        }
    }
    else {
        NewsArticle {
            headline: String::from("Breaking News : very interesting, you want to know!!!"),
            location: String::from("Some big news outlet"),
            author: String::from("Big McChungus"),
            content: String::from("Blah blah blah, the world is ending, blah blah blah"),
        }
    }
} */

// Traits also allow us to conditionally implement methods
// Let's consider the following struct
struct Pair<T> {
    first: T,
    second: T,
}

enum PairElementComparisonResult {
    FirstGtSecond,
    SecondGtFirst,
    BothEqual
}

// The new method is implemented for any type
impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }
}

// The cmp method is implemented only for T bounded with PartialOrd
impl<T: PartialOrd> Pair<T> {
    fn cmp(&self) -> PairElementComparisonResult {
        if self.first > self.second {
            PairElementComparisonResult::FirstGtSecond
        }
        else if self.first < self.second {
            PairElementComparisonResult::SecondGtFirst
        }
        else {
            PairElementComparisonResult::BothEqual
        }
    }
} 

// The cmp_display method is implemented only for T bounded with PartialOrd and Display
impl<T: PartialOrd + Display> Pair<T> {
    fn cmp_display(&self) {
        match self.cmp() {
            PairElementComparisonResult::FirstGtSecond => println!("The largest member is {}", self.first),
            PairElementComparisonResult::SecondGtFirst => println!("The largest member is {}", self.second),
            PairElementComparisonResult::BothEqual => println!("Both members are equal"),
        }
    }
}

// We can also conditionally implement a trait for any type that implement another trait, which is neat
// impl<T: Display> ToString for T {...}
// The standard library actually defined this trait, so for any type that implements Display, we can us the
// .to_string method
// let s = 3.to_string();
// We call it a blanket implementation, they appear in the documentation for the trait in the "Implementors" section

// Here we have a function for which the lifetimes of the parameters aren't clear
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     }
//     else {
//         y
//     }
// }

// The function won't compile because the lifetime of the returned value is not clear at compile time : we will only know 
// at runtime whether we are returning x or y
// We can use a generic lifetime annotation to indicate that both parameters will are referencing resources that live for
// as long as the generic lifetime
// Generic lifetime annotations start with an apostrophe and the name tends to be very short
// Using the 'a annotation below, we ensure that the borrow checker will verify that both arguments live for as long as the 
// generic lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn print(&self) {
        println!("A particularly important excerpt {}", self.part);
    }
}

// The following function is an example of lifetime elision, some common lifetime patterns have been programmed into the Rust
// compiler so that it can infer the lifetime of parameters and return types in these cases
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

// A combination of what we've seen in this (huge) chapter
fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    announcement: T
) -> &'a str
where 
    T: Display,
{
    println!("Oye oye, {}", announcement);

    if x.len() > y.len() {
        x
    } 
    else {
        y
    }
}