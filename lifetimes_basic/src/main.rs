use std::fmt::Display;

fn main() {

    /*
        Compiler liftetime passes:

        fn first_word(s: &str) -> &str {

        // Assign each parameter a lifetime
        1. fn first_word<'a>(s: &a' str) -> &str

        // The lifefime of the one input parameter gets
        // assigned to the output lifetime.
        2. fn first_word<'a>(s: &a' str) -> &'a str
        
        fn longest(x: &str, y: &str) -> &str{

        // Assign each parement a lifetime
        1. fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str

        // Since there is more than one input parameter, the 
        // second rule doesn't apply!

    */

    // Liftime "'static' denotes the entire duration of the program.
    // All string literals have the 'static lifetime.
    let msg: &'static str = "Lifetime basic!";
    println!("{}", msg);
    
    lifetime_basics();
    lifetime_structs();
}

// Syntax for all these features!
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("WERD");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    // This implies that an instance of 'ImportantExcerpt' 
    // can't outlive the reference it holds in its 'part'
    // field.
    part: &'a str,
}

fn lifetime_structs() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("important_excerpt={:?}", i);

    /*
    // This does not work because 'i' outlives 's1', but
    // we told the compiler that 'i' needs to live at least as
    // long as 's1'.
    let i;
    {
        let s1 = String::from("Should not work");
        i = ImportantExcerpt { part: s1.as_str() };
    }
    println!("important_excerpt={:?}", i);
    */

}

fn lifetime_basics() {
    /* 
    // Doesn't work because x goes out of scope
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r={}", r);
    */

    let s1 = String::from("abcd");
    let s2 = "xyz";
    let result = longest(s1.as_str(), s2);
    println!("longest({}, {}) = {}", s1, s2, result);

    let s1 = String::from("Long stringggg");
    {
        let s2 = "yyyyyy";
        let result = longest(s1.as_str(), s2);
        println!("longest({}, {}) = {}", s1, s2, result);
    }

    /*
    // This does not work since s2 does not live long-enough:
    // "s2 -> borrowed value does not live long enough"
    let s1 = String::from("Some long string");
    let result;
    {
        let s2 = String::from("one");
        result = longest(s1.as_str(), s2.as_str());
    }
    println!("longest({}, {}) = {}", s1, s2, result);
    */
}

// All the references passed and returned from this function
// MUST have the same lifetime. Borrow checker will should
// reject any values that don't adhere to these constraints.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
