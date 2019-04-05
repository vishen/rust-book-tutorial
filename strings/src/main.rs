fn main() {
    println!("Strings!");

    strings();
    inner_representations();
}

fn inner_representations() {
    let len = String::from("Hello").len();
    println!("Hello.len={}", len);

    for c in "Hello, World!".chars() {
        println!("c={}", c);
    }
    for b in "Hello, World!".bytes() {
        println!("b={}", b);
    }
}

fn strings() {
    let mut s = String::from("Hello, Strings!");
    s.push_str("! Werd");
    println!("s={}", s);

    let data = "initial contents";
    // Calls the 'Display' trait if implemented.
    let s = data.to_string();
    let s2 = "other contents".to_string();
    println!("s={}, s2={}", s, s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("s={}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // s1 and s2 are no longer owned.
    // '+' operator uses the 'add' signature (basically without generics):
    //      add(self, s: &str) -> String
    let s3 = s1 + &s2;
    println!("s3={}", s3);
    // println!("s1={}, s2={}", s1, s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // Doesn't take ownership of any of the variables.
    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("s4={}", s4);
    println!("s1={}", s1);
}
