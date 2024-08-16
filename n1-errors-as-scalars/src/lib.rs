pub fn divide(x: i32, y: i32) -> Result<i32, String> {
    if x == 0 || y == 0 {
        return Err(String::from("cannot handle zero values"));
    }

    Ok(x / y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_handled() {
        assert_eq!(divide(10, 0), Err(String::from("cannot handle zero values")));
    }

    #[test]
    fn is_handled_with_pattern_1() {
        if let Err(_) = divide(10, 0) {
            return assert!(true);
        }

        assert!(false);
    }

    #[test]
    fn is_handled_with_pattern_2() {
        let Err(_) = divide(10, 0) else {
            return assert!(false);
        };

        assert!(true);
    }
}
