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

pub fn operate() -> Result<i32, DomainError> {
    divide(10, 0)?; // strict propagation to parent
    divide(10, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_handled() {
        assert_eq!(
            operate(),
            Err(DomainError::InvalidDenominator {
                msg: "cannot handle zero values"
            })
        );
    }
}
