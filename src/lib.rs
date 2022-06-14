//! This crate provides a library for translating intergalactic numerals to Roman numerals.
//!
//! # Usage
//! The primary interface is `Ford`. `Ford` can be queried to set translations
//! and enquire about numbers and prices.
//! ```
//! // necessary imports
//! use intra::prelude::*;
//!
//! // Create a new Ford instance.
//! let mut ford = Ford::new();
//!
//! // Setup your dialect of intergalactic numerals.
//! ford.query("glob is I").unwrap();
//! ford.query("prok is V").unwrap();
//! ford.query("pish is X").unwrap();
//! ford.query("tegj is L").unwrap();
//!
//! // Setup the prices of items you're interested in.
//! ford.query("glob glob Silver is 34 Credits").unwrap();
//! ford.query("glob prok Gold is 57800 Credits").unwrap();
//! ford.query("pish pish Iron is 3910 Credits").unwrap();
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
