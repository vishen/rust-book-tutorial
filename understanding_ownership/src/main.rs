fn main() {
    println!("Understanding ownership");
    understanding_ownership();
}

fn understanding_ownership() {
    // 1. Each value in Rust has a variable that's called it's owner.
    // 2. There can be only one owner at a time.
    // 3. When the owner goes out of scope, the value will be destroyed.

    // 'String' values are stored on the heap because they can grow and shrink.
    // It can store an amount of text that isn't known at compile time.
    // The data is freed when 's' goes out of scope. When a variable goes
    // out of scope, rust calls the 'drop' function for 's'.
    let mut s = String::from("hello");
    println!("s={}", s);

    s.push_str(", world");;
    println!("s={}", s);

    let s1 = s;

    // 's' is no longer valid because the ownership of the
    // data (variable) has been passed to 's1'.
    // The following is a compile time error:
    // println!("s={}", s);

    // Performing a "deep-copy" (which is costly).
    let s2 = s1.clone();
    println!("s1={} s2={}", s1, s2);
    
    let s2_len = calculate_length(&s2);
    println!("s2={} len={}", s2, s2_len);
}

fn calculate_length(s: &String) -> usize {
    return s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }
}
