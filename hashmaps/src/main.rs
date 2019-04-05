use std::collections::HashMap;

fn main() {
    println!("Hashmaps!");

    hashmaps();
}

fn hashmaps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    // Invalid because the hashmap has been type coerced.
    // scores.insert(40, 50);
    scores.insert("green".to_string(), 40);
    println!("scores={:?}", scores);

    let teams = vec!["red".to_string(), String::from("yellow")];
    let initial_scores = vec![12, 23];

    let scores: HashMap<_, _> = teams.iter().zip(
        initial_scores.iter(),
     ).collect();
    println!("teams={:?}", teams);
    println!("initial_scores={:?}", initial_scores);
    println!("scores={:?}", scores);
    
    // For values that implement the 'Copy' trait, the value
    // will be *copied*. For owned values such as String, the
    // values will be *moved* and the hashmap will become the
    // owner of those values.
    let field_name = "one".to_string();

    for (key, value) in &scores {
        println!("{} -> {}", key, value); 
    }

    // Overwriting a value
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 1);
    scores.insert("blue".to_string(), 2);
    println!("scores={:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 12);
    scores.entry(String::from("yellow")).or_insert(23);
    // Insert 25 if a value for "blue" doesn't already exist.
    scores.entry(String::from("blue")).or_insert(25);
    println!("scores={:?}", scores)

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map={:?}", map);
}
