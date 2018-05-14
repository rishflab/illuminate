pub mod world;
pub mod network;


fn square(x: i32) -> i32 {
    x*x
}


#[cfg(test)]
mod tests {
    use super::square;

    #[test]
    fn test_square() {
        assert_eq!(square(2), 4);
    }
}