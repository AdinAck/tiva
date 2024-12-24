//! **Ti**ny **Va**lidation

#![no_std]
#![deny(missing_docs)]

use core::ops::{Deref, DerefMut};

/// Wraps a value indicating it has
/// has been successfully validated.
pub struct Validated<V: Validate> {
    /// The wrapped value.
    value: V,
}

/// Indicates the validity
/// of a value.
pub enum Validity<Reason> {
    /// The value is valid.
    Valid,
    /// The value is invalid for the
    /// given reason.
    InValid(Reason),
}

/// Implementors of this trait
/// are granted the ability to be
/// fallibly wrapped by the [`Validated`]
/// type.
pub trait Validate: Sized {
    /// The possible errors encountered
    /// enumerating the reasons for invalidity.
    type Error;

    /// Determine the validity of the value.
    fn validity(&self) -> Validity<Self::Error>;

    /// Attempt to validate the value as dictated
    /// by the [`Validate::validity`] of the value.
    fn validate(self) -> Result<Validated<Self>, Self::Error> {
        match self.validity() {
            Validity::Valid => Ok(Validated { value: self }),
            Validity::InValid(reason) => Err(reason),
        }
    }
}

impl<V: Validate + AsRef<V>> AsRef<V> for Validated<V> {
    fn as_ref(&self) -> &V {
        self.deref().as_ref()
    }
}

impl<V: Validate + AsMut<V>> AsMut<V> for Validated<V> {
    fn as_mut(&mut self) -> &mut V {
        self.deref_mut().as_mut()
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
