//! Decimal type providing equivalent semantics to Cosmos [`sdk.Dec`]
//!
//! [`sdk.Dec`]: https://godoc.org/github.com/cosmos/cosmos-sdk/types#Dec

use crate::error::{Error, ErrorKind};
use anomaly::{ensure, fail};
use std::{
    convert::{TryFrom, TryInto},
    fmt::{self, Debug, Display},
    str::FromStr,
};

/// Number of decimal places used by `sdk.Dec`
/// See: <https://github.com/cosmos/cosmos-sdk/blob/26d6e49/types/decimal.go#L23>
pub const PRECISION: u32 = 18;

/// Maximum value of the decimal part of an `sdk.Dec`
pub const FRACTIONAL_DIGITS_MAX: u64 = 9_999_999_999_999_999_999;

/// Decimal type which follows Cosmos [`sdk.Dec`] conventions.
///
/// [`sdk.Dec`]: https://godoc.org/github.com/cosmos/cosmos-sdk/types#Dec
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Decimal(rust_decimal::Decimal);

impl Decimal {
    /// Create a new [`Decimal`] with the given whole number and decimal
    /// parts. The decimal part assumes 18 digits of precision e.g. a
    /// decimal with `(1, 1)` is `1.000000000000000001`.
    ///
    /// 18 digits required by the Cosmos SDK. See:
    /// See: <https://github.com/cosmos/cosmos-sdk/blob/26d6e49/types/decimal.go#L23>
    pub fn new(integral_digits: i64, fractional_digits: u64) -> Result<Self, Error> {
        ensure!(
            fractional_digits <= FRACTIONAL_DIGITS_MAX,
            ErrorKind::Decimal,
            "fractional digits exceed available precision: {}",
            fractional_digits
        );

        let integral_digits: rust_decimal::Decimal = integral_digits.into();
        let fractional_digits: rust_decimal::Decimal = fractional_digits.into();
        let precision_exp: rust_decimal::Decimal = 10u64.pow(PRECISION).into();

        let mut combined_decimal = (integral_digits * precision_exp) + fractional_digits;
        combined_decimal.set_scale(PRECISION)?;
        Ok(Decimal(combined_decimal))
    }

    /// Serialize this [`Decimal`] as Amino-encoded bytes
    pub fn to_amino_bytes(mut self) -> Vec<u8> {
        self.0
            .set_scale(0)
            .expect("can't rescale decimal for Amino serialization");
        self.to_string().into_bytes()
    }
}

impl Debug for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Decimal {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        s.parse::<rust_decimal::Decimal>()?.try_into()
    }
}

impl TryFrom<rust_decimal::Decimal> for Decimal {
    type Error = Error;

    fn try_from(mut decimal_value: rust_decimal::Decimal) -> Result<Self, Error> {
        match decimal_value.scale() {
            0 => {
                let exp: rust_decimal::Decimal = 10u64.pow(PRECISION).into();
                decimal_value *= exp;
                decimal_value.set_scale(PRECISION)?;
            }
            PRECISION => (),
            other => fail!(
                ErrorKind::Decimal,
                "invalid decimal precision: {} (must be 0 or 18)",
                other
            ),
        }

        Ok(Decimal(decimal_value))
    }
}

macro_rules! impl_from_primitive_int_for_decimal {
    ($($int:ty),+) => {
        $(impl From<$int> for Decimal {
            fn from(num: $int) -> Decimal {
                Decimal::new(num as i64, 0).unwrap()
            }
        })+
    };
}

impl_from_primitive_int_for_decimal!(i8, i16, i32, i64, isize);
impl_from_primitive_int_for_decimal!(u8, u16, u32, u64, usize);

#[cfg(test)]
mod tests {
    use super::Decimal;

    /// Used by e.g. JSON
    #[test]
    fn string_serialization_test() {
        let num = Decimal::from(-1i8);
        assert_eq!(num.to_string(), "-1.000000000000000000")
    }

    #[test]
    fn amino_serialization_test() {
        let num = Decimal::from(-1i8);
        assert_eq!(b"-1000000000000000000", num.to_amino_bytes().as_slice());
    }
}
