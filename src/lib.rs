fn square(x: i32) -> i32 {
    x*x
} 

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_square() {
        assert_eq!(square(2), 4);
    }

}
