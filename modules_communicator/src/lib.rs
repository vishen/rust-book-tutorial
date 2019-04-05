pub mod client;
mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn other_check() {
        assert_eq!(2 + 2, 5);
    }
}
