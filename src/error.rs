use thiserror::Error;

#[derive(Debug, Error)]
#[error("String is not a valid roman numeral")]
pub struct InvalidRomanNumeral;
