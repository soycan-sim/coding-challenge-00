//! Language mapping and translator from the intergalactic numeric system to Roman numerals.
use std::borrow::Cow;

use hashbrown::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use rust_decimal::Decimal;

use crate::error::TranslationError;
use crate::Roman;

lazy_static! {
    static ref QUERY_NUMERAL: Regex = Regex::new(r"(?i:how\s+much\s+is\s+)([a-z\s]*)\?").unwrap();
    static ref QUERY_PRICE: Regex =
        Regex::new(r"(?i:how\s+many\s+credits\s+is\s+)([a-z\s]*)\s+([A-Z].*)\?").unwrap();
}

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

    /// Query the translation of a number or the price of an item.
    ///
    /// Valid queries are of one of the following forms:
    /// - How much is <number>?
    /// - How many credits is <number> <Item>?
    ///
    /// Numbers must always be all lowercase, while items must always be capitalized.
    ///
    /// # Examples
    /// ```
    /// # use std::borrow::Cow;
    /// # use hashbrown::HashMap;
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    /// # use intra::Language;
    /// # let lang = Language::with(HashMap::from([
    /// #   (Cow::from("glob"), 'I'),
    /// #   (Cow::from("prok"), 'V'),
    /// #   (Cow::from("pish"), 'X'),
    /// #   (Cow::from("tegj"), 'L'),
    /// # ]));
    /// # let price_set: HashMap<&str, Decimal> =
    /// # HashMap::from([("Gold", dec!(10)), ("Silver", dec!(5)), ("Iron", dec!(1))]);
    /// assert_eq!(lang.query(&price_set, "How much is pish tegj glob glob?").unwrap(), dec!(42));
    /// ```
    ///
    /// ```
    /// # use std::borrow::Cow;
    /// # use hashbrown::HashMap;
    /// # use rust_decimal::Decimal;
    /// # use rust_decimal_macros::dec;
    /// # use intra::Language;
    /// # let lang = Language::with(HashMap::from([
    /// #   (Cow::from("glob"), 'I'),
    /// #   (Cow::from("prok"), 'V'),
    /// #   (Cow::from("pish"), 'X'),
    /// #   (Cow::from("tegj"), 'L'),
    /// # ]));
    /// # let price_set: HashMap<&str, Decimal> =
    /// # HashMap::from([("Gold", dec!(10)), ("Silver", dec!(5)), ("Iron", dec!(1))]);
    /// // Gold costs 10 credits per unit.
    /// assert_eq!(lang.query(&price_set, "How many credits is glob glob Gold?").unwrap(), dec!(20));
    /// ```
    pub fn query(
        &self,
        price_set: &HashMap<&str, Decimal>,
        text: &str,
    ) -> Result<Decimal, TranslationError> {
        if let Some(captures) = QUERY_NUMERAL.captures(text) {
            let roman = self.translate(captures.get(1).unwrap().as_str())?;
            Ok(Decimal::from(u32::from(roman)))
        } else if let Some(captures) = QUERY_PRICE.captures(text) {
            let roman = self.translate(captures.get(1).unwrap().as_str())?;
            let count = Decimal::from(u32::from(roman));

            let item = captures.get(2).unwrap().as_str();
            let price = price_set
                .get(&item)
                .ok_or_else(|| TranslationError::UnrecognizedItem(item.to_string()))?;

            Ok(count * price)
        } else {
            Err(TranslationError::UnrecognizedQuery(text.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

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

    #[test]
    fn query() {
        let lang = Language::with(HashMap::from([
            (Cow::from("glob"), 'I'),
            (Cow::from("prok"), 'V'),
            (Cow::from("pish"), 'X'),
            (Cow::from("tegj"), 'L'),
        ]));

        let price_set: HashMap<&str, Decimal> =
            HashMap::from([("Gold", dec!(10)), ("Silver", dec!(5)), ("Iron", dec!(1))]);

        // positive tests
        assert_eq!(
            lang.query(&price_set, "How much is pish tegj glob glob?")
                .unwrap(),
            dec!(42),
        );
        assert_eq!(
            lang.query(&price_set, "How many credits is glob glob Gold?")
                .unwrap(),
            dec!(20),
        );

        // negative tests
        assert!(lang.query(&price_set, "How much is foo bar?").is_err());
        assert!(lang
            .query(&price_set, "What is pish tegj glob glob?")
            .is_err());
        assert!(lang
            .query(&price_set, "How many credits is glob glob Copper?")
            .is_err());
        assert!(lang
            .query(&price_set, "How many credits is glob glob glob glob Gold?")
            .is_err());
    }
}
