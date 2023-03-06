//! Number wrapper type.
//!
//! This crate is representative of a wrapper type for an integer that implements some desired
//! abstraction.
//!
//! Used to demonstrate testing methodology, the API and logic may at times be contrived.

use std::fmt;

#[cfg(all(test, mutate))]
use mutagen::mutate;

/// A signed 32 bit integer type.
pub struct Num(i32);

impl Num {
    /// Constructs a new `Num` type from a signed integer.
    ///
    /// # Examples
    /// ```
    /// use num::Num;
    /// let x = -10;
    /// // Roundtrips with `to_signed`.
    /// assert_eq!(Num::from_signed(x).to_signed(), x)
    /// ```
    pub fn from_signed(x: i32) -> Self {
        Num(x)
    }

    /// Constructs a new `Num` type from an unsigned integer.
    ///
    /// # Errors
    ///
    /// An unsigned `Num` has a maximum value of 2^31 - 1, we error if `x` is greater than
    /// the maximum value.
    ///
    /// # Examples
    /// ```
    /// use num::Num;
    /// let x = 10;
    /// // Roundtrips with `to_unsigned`.
    /// assert_eq!(Num::from_unsigned(x)
    ///                .expect("x is positive")
    ///                .to_unsigned()
    ///                .expect("roundtrips without error"),
    ///            x
    ///           );
    /// ```
    #[cfg_attr(all(test, mutate), mutate)]
    pub fn from_unsigned(x: u32) -> Result<Self, Error> {
        if x <= i32::MAX as u32 {
            Ok(Num(x as i32))
        } else {
            Err(Error::Overflow(x))
        }
    }

    /// Returns the value of this number as a signed integer.
    pub fn to_signed(self) -> i32 {
        self.0
    }

    /// Returns the value of this number as a unsigned integer.
    ///
    /// # Errors
    ///
    /// It is an error to call `to_unsigned` if this number is negative.
    pub fn to_unsigned(self) -> Result<u32, Error> {
        if self.0 < 0 {
            return Err(Error::Negative);
        }
        self.0.try_into().map_err(|_| Error::Negative)
    }

    /// Returns the absolute value of this number.
    pub fn abs(self) -> u32 {
        let abs = if self.0 < 0 { 0 - self.0 } else { self.0 };

        abs.try_into().expect("won't panic, int is positive")
    }
}

impl From<i32> for Num {
    fn from(x: i32) -> Self {
        Num::from_signed(x)
    }
}

impl TryFrom<u32> for Num {
    type Error = Error;
    fn try_from(x: u32) -> Result<Self, Self::Error> {
        Num::from_unsigned(x)
    }
}

/// A `Num` related error.
#[derive(Debug)]
pub enum Error {
    /// Unsigned integer overflows `Num`.
    Overflow(u32),
    /// Unexpected negative value.
    Negative,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match *self {
            Overflow(_) => write!(f, "unsigned integer overflows signed number"),
            Negative => write!(f, "unexpected negative value"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abs_handles_positive_value() {
        let x: i32 = 10;
        let num = Num::from_signed(x);
        let abs = num.abs();
        assert_eq!(abs, x as u32)
    }

    #[test]
    fn abs_handles_negative_value() {
        let x: i32 = -10;
        let num = Num::from_signed(x);
        let abs = num.abs();
        assert_eq!(abs, -x as u32)
    }

    #[test]
    fn from_unsigned_i32_max() {
        let x = i32::MAX as u32;
        let n = Num::from_unsigned(x).expect("i32::MAX is within range");
        let got = n.to_unsigned().expect("i32::MAX is positive");
        assert_eq!(x, got)
    }
}
