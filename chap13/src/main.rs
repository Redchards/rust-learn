use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_lucky_number = 7;

    generate_workout_plan2(simulated_user_specified_value, simulated_lucky_number);
}

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

#[cfg(test)]
mod tests {
    use super::Cacher;
    use super::HashCacher;

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
}