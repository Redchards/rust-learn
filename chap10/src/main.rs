use std::cmp;

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