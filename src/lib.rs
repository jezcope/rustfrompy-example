#[no_mangle]
pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[cfg(test)]
mod test {
    use ::*;
    
    #[test]
    fn test_add() {
        assert_eq!(4, add(2, 2));
    }
}
