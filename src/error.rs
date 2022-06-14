//! All possible error states in `intra`.
use thiserror::Error;

/// `InvalidRomanNumeral` represents an error that results from trying to construct an invalid `Roman`.
#[derive(Debug, Error)]
#[error("String is not a valid roman numeral")]
pub struct InvalidRomanNumeral;

/// `TranslationError` represents an error that results from not recognizing a word or phrase.
#[derive(Debug, Error)]
pub enum TranslationError {
    #[allow(missing_docs)]
    #[error("{0}")]
    InvalidRomanNumeral(InvalidRomanNumeral),
    #[allow(missing_docs)]
    #[error("Unrecognized word: `{0}`")]
    UnrecognizedWord(String),
}

impl From<InvalidRomanNumeral> for TranslationError {
    fn from(err: InvalidRomanNumeral) -> Self {
        Self::InvalidRomanNumeral(err)
    }
}
