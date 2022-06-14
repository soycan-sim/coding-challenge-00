//! This crate provides a library for translating intergalactic numerals to Roman numerals.
//!
//! # Usage
//! The primary interface is `Ford`. `Ford` can be queried to set translations
//! and enquire about numbers and prices.
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

pub use assistant::Ford;
