#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greet(name: &str) -> String {
    format!("Hello {}", name)
}

#[allow(dead_code)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}", value);
        }
        else if value > 100 {
            panic!("Guess value must be lower than or equal to 100, got {}", value);
        }
        
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::Rectangle;
    use super::Guess;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // We can also use the Result type
    #[test]
    fn it_works_too() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        }
        else {
            Err(String::from("Two plus two does not equl four"))
        }
    }
    // #[test]
    // fn failing_test() {
    //     panic!("Nooooooo!");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 9,
            height: 10,
        };

        let smaller = Rectangle {
            width: 2,
            height: 4,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle {
            width: 9,
            height: 10,
        };

        let smaller1 = Rectangle {
            width: 2,
            height: 4,
        };

        let smaller2 = Rectangle {
            width: 2,
            height: 14,
        };

        let smaller3 = Rectangle {
            width: 12,
            height: 4,
        };

        assert!(!smaller1.can_hold(&larger));
        assert!(!smaller2.can_hold(&larger));
        assert!(!smaller3.can_hold(&larger));
    }

    #[test]
    fn add_two_test() {
        assert_eq!(4, super::add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = super::greet("Vlad");
        assert!(
            result.contains("Vlad"),
            "Greeting did not contain name 'Vlad', value was '{}'",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be lower than or equal to 100")]
    fn greater_than_100() {
        Guess::new(101);
    }

    #[test]
    #[should_panic]
    fn less_than_1() {
        Guess::new(0);
    }

    #[test]
    #[should_panic(expected = "Guess value must be greater than or equal to 1")]
    fn negative_guess() {
        Guess::new(-1);
    }
}
