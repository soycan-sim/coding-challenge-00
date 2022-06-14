//! A representation for standard roman numerals. Digits go up to M.

use lazy_static::lazy_static;
use regex::Regex;

use crate::error::InvalidRomanNumeral;

lazy_static! {
    static ref ROMAN_REGEX: Regex =
        Regex::new("^M{0,3}(C[MD]|D?C{0,3})(X[CL]|L?X{0,3})(I[XV]|V?I{0,3})$").unwrap();
}

/// `Roman` represents all valid roman numerals.
///
/// A `Roman` can be constructed using the `TryFrom` trait,
/// by providing an owned `String` or a slice `&str`.
/// Invalid numerals cannot be constructed.
/// `Roman` can be converted into a `u32` by calling `u32::from`.
///
/// # Examples
/// ```
/// use intra::Roman;
/// let roman = Roman::try_from("XLII").unwrap();
/// assert_eq!(u32::from(&roman), 42);
/// ```
///
/// ```
/// use intra::Roman;
/// let invalid = Roman::try_from("XXXXXX");
/// assert!(invalid.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Roman {
    value: String,
}

impl Roman {
    fn is_valid(value: &str) -> bool {
        ROMAN_REGEX.is_match(value)
    }

    fn digit_value(digit: char) -> u32 {
        match digit {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }
}

impl<'a> TryFrom<&'a str> for Roman {
    type Error = InvalidRomanNumeral;

    fn try_from(value: &'a str) -> Result<Self, InvalidRomanNumeral> {
        if value.is_empty() {
            return Err(InvalidRomanNumeral);
        }

        Roman::try_from(value.to_string())
    }
}

impl TryFrom<String> for Roman {
    type Error = InvalidRomanNumeral;

    fn try_from(value: String) -> Result<Self, InvalidRomanNumeral> {
        if value.is_empty() {
            return Err(InvalidRomanNumeral);
        }

        if Roman::is_valid(&value) {
            Ok(Self { value })
        } else {
            Err(InvalidRomanNumeral)
        }
    }
}

impl From<Roman> for u32 {
    fn from(roman: Roman) -> Self {
        Self::from(&roman)
    }
}

impl<'a> From<&'a Roman> for u32 {
    fn from(roman: &'a Roman) -> Self {
        let mut acc = 0;

        let mut previous = None;

        for current_char in roman.value.chars() {
            if let Some(last_char) = previous {
                let last_digit = Roman::digit_value(last_char);
                let current_digit = Roman::digit_value(current_char);
                if last_char == current_char {
                    acc += current_digit + last_digit;
                    previous = None;
                } else if last_digit < current_digit {
                    acc += current_digit - last_digit;
                    previous = None;
                } else {
                    acc += last_digit;
                    previous = Some(current_char);
                }
            } else {
                match current_char {
                    'I' | 'X' | 'C' => previous = Some(current_char),
                    _ => acc += Roman::digit_value(current_char),
                }
            }
        }

        if let Some(last_char) = previous {
            acc += Roman::digit_value(last_char);
        }

        acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syntax() {
        // positive tests
        assert!(Roman::try_from("I").is_ok());
        assert!(Roman::try_from("III").is_ok());
        assert!(Roman::try_from("IV").is_ok());
        assert!(Roman::try_from("XIV").is_ok());
        assert!(Roman::try_from("XXIV").is_ok());
        assert!(Roman::try_from("XXVI").is_ok());
        assert!(Roman::try_from("MMMCMXXXIII").is_ok());

        // negative tests
        assert!(Roman::try_from("").is_err());
        assert!(Roman::try_from("IIII").is_err());
        assert!(Roman::try_from("MMMM").is_err());
        assert!(Roman::try_from("XM").is_err());
        assert!(Roman::try_from("XM").is_err());
    }

    #[test]
    fn conversion() {
        assert_eq!(u32::from(Roman::try_from("I").unwrap()), 1);
        assert_eq!(u32::from(Roman::try_from("III").unwrap()), 3);
        assert_eq!(u32::from(Roman::try_from("X").unwrap()), 10);
        assert_eq!(u32::from(Roman::try_from("XXX").unwrap()), 30);
        assert_eq!(u32::from(Roman::try_from("XIV").unwrap()), 14);
        assert_eq!(u32::from(Roman::try_from("XXVI").unwrap()), 26);
        assert_eq!(u32::from(Roman::try_from("CXXIV").unwrap()), 124);
        assert_eq!(u32::from(Roman::try_from("MMMCMIX").unwrap()), 3909);
    }
}
