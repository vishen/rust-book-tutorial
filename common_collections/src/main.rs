fn main() {
    println!("Common collections!");

    strings();
    vectors();
    enum_vectors();
}

fn strings() {

}

#[derive(Debug)]
enum Cell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn enum_vectors() {
    let row = vec![
        Cell::Int(1),
        Cell::Text(String::from("Cell Enum")),
        Cell::Float(3.145),
    ];
    println!("row={:?}", row);
}

fn vectors() {
    let mut v: Vec<i32> = Vec::new();

    // Rust can infer the type required for vector
    // using this macro.
    let mut v1 = vec![1, 2, 3];

    println!("v={:?}", v);
    println!("v1={:?}", v1);

    v.push(5);
    v1.push(5);
    println!("v={:?}", v);
    println!("v1={:?}", v1);

    let vfirst: i32 = v[0];
    let v1first: Option<&i32> = v.get(0);

    let v = vec![1, 200, 4, 23];
    for i in v {
        println!("vi={}", i);
    }

    let mut v = vec![1, 200, 4, 23];
    for i in &mut v {
        *i += 50;
    }
    println!("v={:?}", v);
}
