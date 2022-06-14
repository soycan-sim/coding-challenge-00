//! All possible error states in `intra`.
use thiserror::Error;

/// `InvalidRomanNumeral` represents an error that results from trying to construct an invalid `Roman`.
#[derive(Debug, Error)]
#[error("String is not a valid roman numeral")]
pub struct InvalidRomanNumeral;

/// `TranslationError` represents an error that results from not recognizing a word or phrase.
#[derive(Debug, Error)]
pub enum QueryError {
    #[allow(missing_docs)]
    #[error("{0}")]
    InvalidRomanNumeral(InvalidRomanNumeral),
    #[allow(missing_docs)]
    #[error("Unrecognized word: `{0}`")]
    UnrecognizedWord(String),
    #[allow(missing_docs)]
    #[error("Unrecognized query: `{0}`")]
    UnrecognizedQuery(String),
    #[allow(missing_docs)]
    #[error("Unrecognized item: `{0}`")]
    UnrecognizedItem(String),
    #[allow(missing_docs)]
    #[error("Word already exists: `{0}`")]
    WordAlreadyExists(String),
    #[allow(missing_docs)]
    #[error("Digit already exists: `{0}`")]
    DigitAlreadyExists(char),
    #[allow(missing_docs)]
    #[error("Item already exists: `{0}`")]
    ItemAlreadyExists(String),
}

impl From<InvalidRomanNumeral> for QueryError {
    fn from(err: InvalidRomanNumeral) -> Self {
        Self::InvalidRomanNumeral(err)
    }
}
