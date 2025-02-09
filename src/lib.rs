//! **Ti**ny **Va**lidation

#![no_std]
#![deny(missing_docs)]

/// Implementors of this trait provide
/// a validation service for values of the given
/// source type.
pub trait Validator<S: Validate<Self>>: Sized {
    /// Validation error.
    type Error;

    /// Attempt to validate the source value.
    fn from_unvalidated(src: S) -> Result<Self, Self::Error>;
}

/// Implementors of this trait have a validator
/// available to conduct validation.
pub trait Validate<V: Validator<Self>>: Sized {
    /// Validate the value via the designated
    /// [`Validator`].
    fn validate(self) -> Result<V, V::Error> {
        Validator::from_unvalidated(self)
    }
}

impl<S, V> Validate<V> for S where V: Validator<S> {}

#[cfg(test)]
mod tests {
    use crate::{Validate, Validator};

    pub enum Error {
        TooSmall,
        TooBig,
    }

    pub struct ValidU8(u8);

    impl Validator<u8> for ValidU8 {
        type Error = Error;

        fn from_unvalidated(src: u8) -> Result<Self, Self::Error> {
            if src < 5 {
                Err(Error::TooSmall)
            } else if src > 10 {
                Err(Error::TooBig)
            } else {
                Ok(Self(src))
            }
        }
    }

    #[test]
    fn integer() {
        let validated: Result<ValidU8, _> = 0u8.validate();
        assert!(validated.is_err_and(|e| matches!(e, Error::TooSmall)));

        let validated: Result<ValidU8, _> = 7u8.validate();
        assert!(validated.is_ok_and(|v| v.0 == 7));

        let validated: Result<ValidU8, _> = 11u8.validate();
        assert!(validated.is_err_and(|e| matches!(e, Error::TooBig)));
    }
}
