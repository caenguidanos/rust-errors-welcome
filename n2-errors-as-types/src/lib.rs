#[derive(Debug, PartialEq)]
pub enum DomainError {
    InvalidNumerator { msg: &'static str },
    InvalidDenominator { msg: &'static str },
}

pub fn divide(x: i32, y: i32) -> Result<i32, DomainError> {
    if x == 0 {
        return Err(DomainError::InvalidNumerator {
            msg: "cannot handle zero values",
        });
    }
    if y == 0 {
        return Err(DomainError::InvalidDenominator {
            msg: "cannot handle zero values",
        });
    }

    Ok(x / y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_handled() {
        assert_eq!(
            divide(10, 0),
            Err(DomainError::InvalidDenominator {
                msg: "cannot handle zero values"
            })
        );
    }

    #[test]
    fn is_handled_with_pattern_1() {
        if let Err(DomainError::InvalidDenominator { msg }) = divide(10, 0) {
            println!("{msg}");
            return assert!(true);
        }

        assert!(false);
    }

    #[test]
    fn is_handled_with_pattern_2() {
        let Err(DomainError::InvalidDenominator { msg }) = divide(10, 0) else {
            return assert!(false);
        };

        println!("{msg}");
        assert!(true);
    }

    #[test]
    fn is_handled_with_pattern_3() {
        match divide(10, 0) {
            Ok(_) => assert!(false),
            Err(error) => match error {
                DomainError::InvalidNumerator { msg } => {
                    println!("{msg}");
                    assert!(true);
                }
                DomainError::InvalidDenominator { msg } => {
                    println!("{msg}");
                    assert!(true);
                }
            },
        }
    }
}
