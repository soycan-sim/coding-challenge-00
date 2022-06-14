//! All possible error states in `intra`.
use thiserror::Error;

/// `InvalidRomanNumeral` represents an error that results from trying to construct an invalid `Roman`.
#[derive(Debug, Error)]
#[error("String is not a valid roman numeral")]
pub struct InvalidRomanNumeral;
