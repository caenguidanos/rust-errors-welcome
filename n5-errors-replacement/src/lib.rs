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

#[derive(Debug, PartialEq)]
pub enum NewDomainError {
    InvalidOperation { msg: &'static str },
}

impl From<DomainError> for NewDomainError {
    fn from(value: DomainError) -> Self {
        match value {
            DomainError::InvalidNumerator { msg } | DomainError::InvalidDenominator { msg } => Self::InvalidOperation { msg },
        }
    }
}

pub fn operate() -> Result<i32, NewDomainError> {
    divide(10, 0)?; // strict propagation to parent with implicit casting
    let mut result = divide(10, 1)?;
    result *= 10;
    Ok(result)
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
