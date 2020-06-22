fn main() {
    let user1 = User {
        email: String::from("coucou@gmail.com"),
        username: String::from("coucou"),
        active: true,
        sign_in_count: 1
    };

    println!("Our structure is {}, {}, {}, {}", user1.email, user1.username, user1.active, user1.sign_in_count);
    
    let user2 = create_new_user("Toto", "toto@gmail.com");
    println!("Our structure is {}, {}, {}, {}", user2.email, user2.username, user2.active, user2.sign_in_count);

    let user3 = update_user(user2, "Titi", "tiio@gmail.com");
    println!("Our structure is {}, {}, {}, {}", user3.email, user3.username, user3.active, user3.sign_in_count);

    let coord = Vector3(3.4, 5.3, 64.3);
    println!("Here's a vector ({}, {}, {})", coord.0, coord.1, coord.2);

    let rect = Rectangle{ width: 43.5, height: 355.3 };
    println!("The area of our rectangle is {:.2}", compute_area_of(&rect));

    let rect = Rectangle2{ width: 43.5, height: 355.3 };
    println!("We can print our rectangle!!! {:?}", rect);
    println!("We can also pretty print it {:#?}", rect);
    println!("The area of our rectangle is {:.2}", compute_area_of2(&rect));

    let rect = Rectangle3{ width: 43.5, height: 355.3 };
    println!("We can print our rectangle!!! {:?}", rect);
    println!("We can also pretty print it {:#?}", rect);
    println!("The area of our rectangle is {:.2}", rect.area());

    let rect1 = Rectangle3{ width: 43.5, height: 355.3 };
    let rect2 = Rectangle3{ width: 143.5, height: 3355.3 };
    let rect3 = Rectangle3{ width: 4.5, height: 35.3 };
    println!("Can our first rectangle contain the second one? {}", rect1.can_contain(&rect2));
    println!("Can our first rectangle contain the third one? {}", rect1.can_contain(&rect3));

    let square = Rectangle3::make_square(34.3);
    println!("Mommy, I made a square!!! {:#?}", square)
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn create_new_user(username: &str, email: &str) -> User {
    return User {
        email: String::from(email),
        username: String::from(username),
        active: true,
        sign_in_count: 1
    };
}

fn update_user(user: User, username: &str, email: &str) -> User {
    return User {
        username: String::from(username),
        email: String::from(email),
        ..user
    };
}

struct Vector3(f64, f64, f64);

struct Rectangle {
    width: f64,
    height: f64
}

fn compute_area_of(rect: &Rectangle) -> f64 {
    rect.width * rect.height
}

#[derive(Debug)]
struct Rectangle2 {
    width: f64,
    height: f64
}

fn compute_area_of2(rect: &Rectangle2) -> f64 {
    rect.width * rect.height
}

#[derive(Debug)]
struct Rectangle3 {
    width: f64,
    height: f64
}

// We could also split this block in multiple ones, we don't need to do it all at once
impl Rectangle3 {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn can_contain(&self, other: &Rectangle3) -> bool {
        self.height > other.height && self.width > other.width
    }
    
    fn  make_square(size: f64) -> Rectangle3 {
        Rectangle3 {
            height: size,
            width: size
        }
    }
}