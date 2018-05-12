fn square(x: i32) -> i32 {
    x*x
} 

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_square() {
        assert_eq!(square(2), 4);
    }


}
