// Const's must be annotated, and are always immutable.
// Can only be set to a constant expression, not the result of
// a function call or any other vaule that could be computed 
// at runtime.
const MAX_POINTS: i32 = 100_000;

fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("x={}", x);
    x += 1;
    println!("x={}", x);

    // Shadowing is allowed.
    let x = "6";
    println!("x={}", x);

    let spaces = "      ";
    let spaces = spaces.len();
    println!("spaces={}", spaces);

    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess={}", guess);

    let tup = (500, 1.3, "asd");
    let (x, y, z) = tup;
    println!("x={} y={} z={}", x, y, z);
    println!("x={} y={} z={}", tup.0, tup.1, tup.2);

    // Arrays store data on the stack!!
    // Arrays store data on the stack!!
    // Arrays store data on the stack!!
    // Arrays store data on the stack!!
    // Arrays store data on the stack!!
    // Arrays store data on the stack!!
    // Arrays aren't flexible and can't grow (which is
    // why they can go on the stack).
    let arr = [1, 2, 3];
    println!("arr={:?}", arr);

    let arr_of_tup = [(1, "a"), (2, "b")];
    println!("arr_of_tup={:?}", arr_of_tup);

    some_function(arr[0]);
    
    let x = 3;
    some_function(x);

    let y = {
        let x = 5;
        x + 1
        // THIS IS NOT POSSIBLE;
        // return x + 1;
    };
    println!("y={}", y);

    println!("five()={}", five());

    let condition = false;
    let number = if condition {
        5
    } else {
        6
    };
    println!("number={}", number);

    let mut x = 3;
    println!("Start while loop");
    while x != 0 {
        println!("x={}", x);
        x -= 1;
    }

    let a = [10, 20, 30, 40];
    for e in a.iter() {
        println!("value {}", e);
    }

    for num in (1..10).rev() {
        println!("num={}", num);
    }
}

fn some_function(x: i32) {
    println!("CALLED SOME FUNCTION");
    let y = x + 1;
    println!("y={}", x);
}

fn five() -> i32 {
    5
}
