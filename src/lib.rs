//! **Ti**ny **Va**lidation

#![no_std]

use core::ops::{Deref, DerefMut};

pub struct Validated<V: Validate> {
    value: V,
}

pub enum Validity<Reason> {
    Valid,
    InValid(Reason),
}

pub trait Validate: Sized {
    type Error;

    fn validity(&self) -> Validity<Self::Error>;
    fn validate(self) -> Result<Validated<Self>, Self::Error> {
        match self.validity() {
            Validity::Valid => Ok(Validated { value: self }),
            Validity::InValid(reason) => Err(reason),
        }
    }
}

impl<V: Validate> Deref for Validated<V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<V: Validate> DerefMut for Validated<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[cfg(test)]
mod tests {
    use crate::{Validate, Validity};

    pub enum Error {
        TooSmall,
        TooBig,
    }

    impl Validate for u8 {
        type Error = Error;

        fn validity(&self) -> Validity<Self::Error> {
            if *self < 5 {
                Validity::InValid(Error::TooSmall)
            } else if *self > 10 {
                Validity::InValid(Error::TooBig)
            } else {
                Validity::Valid
            }
        }
    }

    #[test]
    fn deref() {
        let validated = 0u8.validate();
        assert!(validated.is_err_and(|e| matches!(e, Error::TooSmall)));

        let validated = 7u8.validate();
        assert!(validated.is_ok_and(|v| *v == 7));

        let validated = 11u8.validate();
        assert!(validated.is_err_and(|e| matches!(e, Error::TooBig)));
    }

    #[test]
    fn deref_mut() {
        let validated = 7u8.validate();
        assert!(validated.is_ok_and(|mut v| {
            *v += 1;
            *v == 8
        }));
    }
}
