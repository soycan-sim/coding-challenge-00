//! A personal assistant for all your galaxy hitchhiking needs.
use std::borrow::Cow;

use hashbrown::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use rust_decimal::Decimal;

use crate::error::TranslationError;
use crate::language::Language;

lazy_static! {
    static ref QUERY_NUMERAL: Regex = Regex::new(r"(?i:how\s+much\s+is\s+)([a-z\s]*)\?").unwrap();
    static ref QUERY_PRICE: Regex =
        Regex::new(r"(?i:how\s+many\s+credits\s+is\s+)([a-z\s]*)\s+([A-Z].*)\?").unwrap();
}

/// Fast Omniscient Robotic guiDe is a personal assistant on your hitchhike through the galaxy.
#[derive(Default, Debug)]
pub struct Ford<'a> {
    language: Language<'a>,
    price_set: HashMap<Cow<'a, str>, Decimal>,
}

impl<'a> Ford<'a> {
    /// Constructs a new empty `Ford`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructs a new `Ford` with a `Language` and a set of prices.
    pub fn with(language: Language<'a>, price_set: HashMap<Cow<'a, str>, Decimal>) -> Self {
        Self {
            language,
            price_set,
        }
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
    /// # let price_set = HashMap::from([
    /// #   (Cow::from("Gold"), dec!(10)),
    /// #   (Cow::from("Silver"), dec!(5)),
    /// #   (Cow::from("Iron"), dec!(1)),
    /// # ]);
    /// # let mut ford = Ford::with(lang, price_set);
    /// assert_eq!(ford.query("How much is pish tegj glob glob?").unwrap(), dec!(42));
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
    /// # let price_set = HashMap::from([
    /// #   (Cow::from("Gold"), dec!(10)),
    /// #   (Cow::from("Silver"), dec!(5)),
    /// #   (Cow::from("Iron"), dec!(1)),
    /// # ]);
    /// # let mut ford = Ford::with(lang, price_set);
    /// // Gold costs 10 credits per unit.
    /// assert_eq!(ford.query("How many credits is glob glob Gold?").unwrap(), dec!(20));
    /// ```
    pub fn query(&mut self, query: &str) -> Result<Option<String>, TranslationError> {
        if let Some(captures) = QUERY_NUMERAL.captures(query) {
            let intergalactic = captures.get(1).unwrap().as_str();
            let roman = self.language.translate(intergalactic)?;

            let decimal = u32::from(roman);

            Ok(Some(format!("{intergalactic} is {decimal}")))
        } else if let Some(captures) = QUERY_PRICE.captures(query) {
            let intergalactic = captures.get(1).unwrap().as_str();
            let roman = self.language.translate(intergalactic)?;
            let count = Decimal::from(u32::from(roman));

            let item = captures.get(2).unwrap().as_str();
            let price = self
                .price_set
                .get(&Cow::from(item))
                .ok_or_else(|| TranslationError::UnrecognizedItem(item.to_string()))?;

            let total_price = count * price;

            Ok(Some(format!(
                "{intergalactic} {item} is {total_price} Credits"
            )))
        } else {
            Err(TranslationError::UnrecognizedQuery(query.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn query() {
        let lang = Language::with(HashMap::from([
            (Cow::from("glob"), 'I'),
            (Cow::from("prok"), 'V'),
            (Cow::from("pish"), 'X'),
            (Cow::from("tegj"), 'L'),
        ]));

        let price_set = HashMap::from([
            (Cow::from("Gold"), dec!(10)),
            (Cow::from("Silver"), dec!(5)),
            (Cow::from("Iron"), dec!(1)),
        ]);

        let mut ford = Ford::with(lang, price_set);

        // positive tests
        assert_eq!(
            ford.query("How much is pish tegj glob glob?").unwrap(),
            Some("pish tegj glob glob is 42".to_string()),
        );
        assert_eq!(
            ford.query("How many credits is glob glob Gold?").unwrap(),
            Some("glob glob Gold is 20 Credits".to_string())
        );

        // negative tests
        assert!(ford.query("How much is foo bar?").is_err());
        assert!(ford.query("What is pish tegj glob glob?").is_err());
        assert!(ford.query("How many credits is glob glob Copper?").is_err());
        assert!(ford
            .query("How many credits is glob glob glob glob Gold?")
            .is_err());
    }
}
