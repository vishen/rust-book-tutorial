
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    // Associated functions
    //
    // Functions that DON'T take '&self' as a parameter.
    // You don't need an instance of the struct to call these,
    // similar to 'static methods' in other languages.
    fn square(size: u32) -> Rectangle {
        Rectangle{width: size, height: size}
    }

    // Methods are functions that take a self parameter.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

}

// Tuple Structs
//
// Structs that look similar to tuples. Tuple structs
// are useful when you want to gives tuples a name and
// make the whole tuple a different type and naming each
// field in a struct would be too verbose.

struct Colour(i32, i32, i32);
struct Point(i32, i32);

fn main() {
    println!("Using structs!");
    user_struct();
    tuple_structs();
    rectangle();
}

fn rectangle() {
    let r1 = Rectangle{width: 30, height: 50};
    println!("The area of r1 is {}", area(&r1));
    println!("The r1.area() is {}", r1.area());
    println!("r1={:?}", r1);
    println!("(formatted) r1={:#?}", r1);

    let r2 = Rectangle{width: 30, height: 50};
    let r3 = Rectangle{width: 10, height: 40};
    let r4 = Rectangle{width: 60, height: 45};
    println!("r2.can_hold(r3)={}", r2.can_hold(&r3));
    println!("r2.can_hold(r4)={}", r2.can_hold(&r4));

    let sq = Rectangle::square(12);
    println!("sq={:?} area={}", sq, sq.area());
}

fn area(r: &Rectangle) -> u32 {
    r.width * r.height
}

fn tuple_structs() {
    let colour = Colour(0, 1, 2);
    println!(
        "colour.0={} colour.1={} colour.2={}", 
        colour.0, colour.1, colour.2
    );
    let origin = Point(4, 5);
    println!("origin.0={} origin.1={}", origin.0, origin.1);
}

fn user_struct() {
    let u1 = User {
        email: String::from("x@example.com"),
        username: String::from("x"),
        active: true,
        sign_in_count: 1,
    };
    println!("u1={:?}", u1);

    let mut u2 = User {
        email: String::from("y@example.com"),
        username: String::from("y"),
        active: true,
        sign_in_count: 1,
    };
    u2.sign_in_count += 1;
    println!("u2={:?}", u2);

    let u3 = new_user(String::from("z"), String::from("z@example.com"));
    println!("u3={:?}", u3);

    let u4 = User{
        email: String::from("u4@example.com"),
        username: String::from("u4"),
        ..u2
    };
    println!("u4={:?}", u4);
}

fn new_user(username: String, email: String) -> User {
    User {
        email,
        username,
        // This needs to match exactly what is declared the struct!
        // usernamee,
        active: true,
        sign_in_count: 1,
    }
}
