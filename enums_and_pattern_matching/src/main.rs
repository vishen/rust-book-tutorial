fn main() {
    println!("Enums and Pattern Matching!");
    enums();
    options();
    matching();
    if_let();
}

fn if_let() {
    let o = Some(3u8);
    match o {
        Some(2) => println!("2!"),
        Some(3) => println!("3!"),
        _ => (),
    }

    // Loses exhaustive checking that 'match' enforces.
    if let Some(3) = o {
        println!("three!");
    }
}

#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter state is {:?}", state);   
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}


fn matching() {
    println!("value_in_cents(Coin::Penny)={}", value_in_cents(Coin::Penny));
    println!("value_in_cents(Coin::Nickel)={}", value_in_cents(Coin::Nickel));
    println!("value_in_cents(Coin::Dime)={}", value_in_cents(Coin::Dime));
    println!(
        "value_in_cents(Coin::Quarter)={}", 
        value_in_cents(Coin::Quarter(State::Alabama))
    );

    let five = Some(5);
    println!("five={:?}", five);
    let six = plus_one(five);
    println!("six={:?}", six);
    let none = plus_one(None);
    println!("none={:?}", none);

    let some_u8_value = 0u8; // let some_u8_value: u8 = 0;
    match some_u8_value {
        1 => println!("prime!"),
        _ => println!("not sure of prime!"),
    }

}

fn options() {
    let some_number = Some(5);
    println!("some_number={:?}", some_number);

    let some_string = Some("a string");
    println!("some_string={:?}", some_string);

    let absent_number: Option<i32> = None;
    println!("absent_number={:?}", absent_number);
}
    
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_type: IpAddrKind) {
    println!("ip_type={:?}", ip_type);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32}, // Anonymous struct
    Write(String),
    ChangeColour(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("message={:?}", self);
    }
}

fn enums() {
    let four = IpAddrKind::V4(127,0,0,1);
    let six = IpAddrKind::V6(String::from("::1"));

    route(four);
    route(six);

    let m_quit = Message::Write(String::from("hello, world"));
    m_quit.call();

    // TODO: How do you initialise this?
    // let m_move::Move({x: 1, y: 2});
    // m_move.call();
}
