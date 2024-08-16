#[derive(Debug, PartialEq)]
pub enum DomainError {
    InvalidNumerator { msg: &'static str },
    InvalidDenominator { msg: &'static str },
}

impl DomainError {
    fn message(&self) -> &'static str {
        match self {
            Self::InvalidDenominator { msg } | Self::InvalidNumerator { msg } => msg,
        }
    }
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

#[derive(Debug, PartialEq)]
pub enum NewDomainError {
    InvalidOperation { msg: &'static str },
}

pub fn operate() -> Result<i32, NewDomainError> {
    divide(10, 0)
        .map(|value| value * 10)
        .map_err(|error| NewDomainError::InvalidOperation { msg: error.message() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_handled() {
        assert_eq!(
            operate(),
            Err(NewDomainError::InvalidOperation {
                msg: "cannot handle zero values"
            })
        );
    }
}
