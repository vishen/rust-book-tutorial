fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        // self.length > other.length && self.width > other.width
        self.length < other.length && self.width > other.width
    }
}

pub fn this_should_panic() {
    panic!("This function panics!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "This function panics!")]
    fn should_panic_fn() {
        // should_panic(expected="<substring>");
        this_should_panic();
    }
    
    #[test]
    #[ignore]
    #[should_panic(expected = "werdwerdwerd")]
    fn other_should_panic_fn() {
        this_should_panic();
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 12, width: 23 };
        let smaller = Rectangle { length: 10, width: 20 };
        assert!(
            larger.can_hold(&smaller),
            "Some message because {:?}.can_hold({:?}) fails", larger, smaller);
    }
    
    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle { length: 12, width: 23 };
        let smaller = Rectangle { length: 10, width: 20 };
        assert!(!smaller.can_hold(&larger));
    }


    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn should_fail() {
        panic!("Fail this test.");
    }

}
