//! This crate provides a library for translating intergalactic numerals to Roman numerals.
//!
//! # Usage
//! The primary interface is `Language`. `Language` provides functions to translate numerals
//! and query prices of individual items.
//! ```
//! // necessary imports
//! use intra::prelude::*;
//!
//! // Setup your dialect of intergalactic numerals.
//! let lang = Language::with(HashMap::from([
//!   (Cow::from("glob"), 'I'),
//!   (Cow::from("prok"), 'V'),
//!   (Cow::from("pish"), 'X'),
//!   (Cow::from("tegj"), 'L'),
//! ]));
//!
//! // Setup the prices of items you're interested in.
//! let price_set: HashMap<&str, Decimal> =
//!     HashMap::from([("Gold", dec!(10)), ("Silver", dec!(5)), ("Iron", dec!(1))]);
//!
//! // Query the price...
//! let price = lang.query(&price_set, "How many credits is glob glob Gold?").unwrap();
//!
//! /// or simply query a number.
//! let answer = lang.query(&price_set, "How much is pish tegj glob glob?").unwrap();
//! ```

#![warn(missing_docs)]

pub mod assistant;
pub mod error;
pub mod language;
pub mod roman;

#[allow(missing_docs)]
pub mod prelude {
    // re-export necessary items from std and other libraries
    pub use hashbrown::HashMap;
    pub use rust_decimal::Decimal;
    pub use rust_decimal_macros::dec;
    pub use std::borrow::Cow;

    // re-export commonly used items from Intra
    pub use crate::assistant::Ford;
    pub use crate::language::Language;
    pub use crate::roman::Roman;
}

pub use assistant::Ford;
pub use language::Language;
pub use roman::Roman;
