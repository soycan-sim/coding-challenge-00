//! Language mapping and translator from the intergalactic numeric system to Roman numerals.
use std::borrow::Cow;

use hashbrown::HashMap;
use rust_decimal::Decimal;

use crate::error::TranslationError;
use crate::Roman;

/// `Language` is a mapping of intergalactic numerals to terran Roman numerals.
#[derive(Default, Debug, Clone)]
pub struct Language<'a> {
    map: HashMap<Cow<'a, str>, char>,
}

impl<'a> Language<'a> {
    /// Construct an empty `Language`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a `Language` with an existing map.
    pub fn with(map: HashMap<Cow<'a, str>, char>) -> Self {
        Self { map }
    }

    /// Translate an intergalactic numeral to `Roman`.
    pub fn translate(&self, text: &str) -> Result<Roman, TranslationError> {
        let text = text
            // split at whitespace
            .split(char::is_whitespace)
            // for every word
            .map(|word| {
                // find the translation
                self.map
                    .get(&Cow::from(word))
                    // error if not found
                    .ok_or_else(|| TranslationError::UnrecognizedWord(word.to_string()))
            })
            // collect into string or first error
            .collect::<Result<String, TranslationError>>();
        // construct a roman numeral
        text.and_then(|text| Ok(Roman::try_from(text)?))
    }

    pub fn query(&self, price_set: &HashMap<&str, Decimal>, text: &str) -> Decimal {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate() {
        let lang = Language::with(HashMap::from([
            (Cow::from("glob"), 'I'),
            (Cow::from("prok"), 'V'),
            (Cow::from("pish"), 'X'),
            (Cow::from("tegj"), 'L'),
        ]));

        // positive tests
        assert_eq!(
            lang.translate("glob").unwrap(),
            Roman::try_from("I").unwrap()
        );
        assert_eq!(
            lang.translate("glob glob glob").unwrap(),
            Roman::try_from("III").unwrap()
        );
        assert_eq!(
            lang.translate("pish tegj glob glob").unwrap(),
            Roman::try_from("XLII").unwrap()
        );

        // negative tests
        assert!(lang.translate("foo").is_err());
        assert!(lang.translate("glob foo").is_err());
        assert!(lang.translate("foo glob").is_err());
        assert!(lang.translate("glob glob glob glob").is_err());
    }
}
