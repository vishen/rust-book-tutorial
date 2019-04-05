use std::cmp::PartialOrd;

fn main() {
    println!("Generics!");
    generics();
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn val(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn unique(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct PointDifferent<T, U> {
    x: T,
    y: U,
}

enum SomeOption<T> {
    Some(T),
    None,
}

enum SomeResult<T, E> {
    Ok(T),
    Err(E),
}

fn generics() {
    let integer = Point{x:1, y:12};
    println!("{:?}, val={}", integer, integer.val());
    let float = Point{x:1.1, y:12.4};
    println!("{:?}, unique={}", float, float.unique());
    let int = PointDifferent{ x:3, y: 5 };
    println!("{:?}", int);
    let combined = PointDifferent{x: 12, y:10.1};
    println!("{:?}", combined);

    let number_list = vec![1, 4, 3, 5, 2];
    println!("largest of {:?} is {}", number_list, largest(&number_list));

    let char_list = vec!['a', 'f', 'd'];
    println!("largest of {:?} is {}", char_list, largest(&char_list));

}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
