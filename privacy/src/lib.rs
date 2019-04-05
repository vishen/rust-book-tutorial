
mod outermost {
    pub fn middle_fn(){}

    fn middle_secret_fn(){}

    pub mod inside {
        pub fn inner_fn(){}
        fn secret_fn(){
            super::middle_fn();
        }
    }
}

fn try_me() {
    outermost::middle_fn();
    // outermost::middle_secret_fn(); // Private to this scope
    outermost::inside::inner_fn(); // Private to this scope
    // outermost::inside::secret_fn(); // Private to this scope
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
