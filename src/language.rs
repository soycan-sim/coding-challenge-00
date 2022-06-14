use hashbrown::HashMap;
use rust_decimal::Decimal;

use crate::Roman;

pub struct Language {}

impl Language {
    pub fn with(map: HashMap<&str, char>) -> Self {
        todo!()
    }

    pub fn translate(&self, text: &str) -> Roman {
        todo!()
    }

    pub fn query(&self, price_set: &HashMap<&str, Decimal>, text: &str) -> Decimal {
        todo!()
    }
}
