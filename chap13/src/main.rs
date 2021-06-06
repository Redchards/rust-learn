use std::thread;
use std::time::Duration;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_lucky_number = 7;

    // A simple example using closures
    generate_workout_plan_closure(simulated_user_specified_value, simulated_lucky_number);
    
    // Closures can also be used to capture their environment
    let x = 5;
    
    let equal_to_x = |y| x == y;

    let y = 5;
    assert!(equal_to_x(y)); // OK

    // This is not the same for functions though
    // The following line will result in the following error : can't capture dynamic environment in a fn item
    // fn equal_to_x(y: u32) -> bool { x == y };

    // There are different types of closures, defined by what traits they implement. They can implement the following traits
    // FnOnce : take ownership of the captured variables and (potentially) consumes them
    // FnMut : borrow the captured variables mutably
    // Fn : borrow the captured variable immutably
    // What trait a closure implement is inferred by how it's using the captured environment. We can force a closure to take
    // ownership of the captured variables by using the keyword "move", that being said, such a closure can still implement 
    // Fn or FnMut, as what traits it implements is defined by how the closure is using the variables, not how it captures
    // them, the latter of which is the only thing defined by the "move" keyword.

    // All closures implement FnOnce, and then on top of it they may implement FnMut and Fn

    let x = vec![1, 2, 3];

    let equal_to_x = move |y| x == y;
    
    // We can't write the following line as the value has been moved
    // "Value borrowed here after move"
    // println!("Can't use x here {:?}", x);

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
    
    // Now onto iterators, pretty simple stuff
    let v1 = vec![1, 2, 3];

    // Iterators being lazy, this has no effect other than creating the iterator structure
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got {}", val);
    }

    // When implementing the iterator traits, many methods are already provided by default, like sum()
    // The only method that we have to implement is next()
    let v1  = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let result: i32 = v1_iter.sum();
    assert_eq!(result, 6);

    // We can't use v1_iter after that, as the sum() method has taken ownership of it
    // Then we have the iterator adapters, types that implement the Iterator trait as well but change iterators
    // into a different kind of iterator
    // That being said, iterators are lazy, hence

    let v1: Vec<i32> = vec![1, 2, 3, 3, 4, 4, 5];
    let adapted_iter = v1.iter().map(|x| x + 1);

    // This does nothing, the compilator will warn us against that
    // We have to "collect" our adapted iterators into a collection, here another vector

    let v2: Vec<_> = adapted_iter.collect();
    println!("New vec : {:?}", v2);

    // But we can use other collections
    let adapted_iter = v1.iter().map(|x| x + 1);
    let s1: HashSet<_> = adapted_iter.collect();

    println!("New set : {:?}", s1);
}

#[allow(dead_code)]
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculate slowly...");
    thread::sleep(Duration::from_secs(2));

    intensity
}

#[allow(dead_code)]
fn generate_workout_plan(intensity: u32, lucky_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Today, do {} situps",
            simulated_expensive_calculation(intensity)
        );
    } 
    else {
        if lucky_number == 3 {
            println!("Take a break");
        }
        else {
            println!(
                "Today, run for {} minutes",
                simulated_expensive_calculation(intensity)
            )
        }
    }
}

#[allow(dead_code)]
fn generate_workout_plan2(intensity: u32, lucky_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);
    
    if intensity < 25 {
        println!(
            "Today, do {} pushups",
            expensive_result
        );
        println!(
            "Today, do {} situps",
            expensive_result
        );
    } 
    else {
        if lucky_number == 3 {
            println!("Take a break");
        }
        else {
            println!(
                "Today, run for {} minutes",
                expensive_result
            )
        }
    }
}

#[allow(dead_code)]
fn generate_workout_plan_closure(intensity: u32, lucky_number: u32) {
    // Thanks to type inference, |intensity| {...} would work too
    let simulated_closure = |intensity: u32| -> u32 {
        println!("Calculate slowly...");
        thread::sleep(Duration::from_secs(2));

        intensity
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups",
            simulated_closure(intensity)
        );
        println!(
            "Today, do {} situps",
            simulated_closure(intensity)
        );
    } 
    else {
        if lucky_number == 3 {
            println!("Take a break");
        }
        else {
            println!(
                "Today, run for {} minutes",
                simulated_closure(intensity)
            )
        }
    }
}

// Implementation with simple cacher
// Note : once the cache is filled with a value, any subsequent call to "value" will return the same value, which is
// counterintuitive, see below for a better solution
struct Cacher<T: Fn(u32) -> u32>
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation: calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(value) => value,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);

                v
            },
        }
    }
}

#[allow(dead_code)]
fn generate_workout_plan_closure_cache(intensity: u32, lucky_number: u32) {
    let mut expensive_result = Cacher::new(|intensity| {
        println!("Calculate slowly...");
        thread::sleep(Duration::from_secs(2));

        intensity
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups",
            expensive_result.value(intensity)
        );
        println!(
            "Today, do {} situps",
            expensive_result.value(intensity)
        );
    } 
    else {
        if lucky_number == 3 {
            println!("Take a break");
        }
        else {
            println!(
                "Today, run for {} minutes",
                expensive_result.value(intensity)
            )
        }
    }
}

// Caching with Hashmap
// Only support copyable types, which is admittedly a big limitation but easier to deal with for the sake of example
struct HashCacher<T, U, V>
where 
    T: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Copy,
{
    calculation_callback: T,
    results: HashMap<U, Option<V>>,
}

impl<T, U, V> HashCacher<T, U, V> 
where
    T: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Copy,
{
    fn new(callback: T) -> HashCacher<T, U, V> {
        HashCacher {
            calculation_callback: callback,
            results: HashMap::new(),
        }
    }

    fn value(&mut self, val: U) -> V {
        self.results
            .entry(val)
            .or_insert(Some((self.calculation_callback)(val)))
            .unwrap()
    }
}

#[allow(dead_code)]
fn generate_workout_plan_closure_cache2(intensity: u32, lucky_number: u32) {
    let mut expensive_result = HashCacher::new(|intensity| {
        println!("Calculate slowly...");
        thread::sleep(Duration::from_secs(2));

        intensity
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups",
            expensive_result.value(intensity)
        );
        println!(
            "Today, do {} situps",
            expensive_result.value(intensity)
        );
    } 
    else {
        if lucky_number == 3 {
            println!("Take a break");
        }
        else {
            println!(
                "Today, run for {} minutes",
                expensive_result.value(intensity)
            )
        }
    }
}

#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

#[allow(dead_code)]
fn get_shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

// Below is a dummy implementation for an iterator that will only ever count from 1 to 5
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0,
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        }
        else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn cacher_call_with_different_values() {
        let mut c = Cacher::new(|x| x);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }

    #[test]
    fn hash_cacher_call_with_different_values() {
        let mut c = HashCacher::new(|x| x);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }

    #[test]
    fn iterator_demo1() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
    
    #[test]
    fn iterator_demo2() {
        let v1  = vec![1, 2, 3];
        let v1_iter = v1.iter();

        let result: i32 = v1_iter.sum();

        assert_eq!(result, 6);
    }

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let res = get_shoes_in_size(shoes, 10);

        assert_eq!(
            res,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        )
    }

    #[test]
    fn test_counter() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn test_counter_other_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(sum, 18);
    }
}