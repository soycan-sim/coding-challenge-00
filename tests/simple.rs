use std::borrow::Cow;

use hashbrown::HashMap;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use intra::Language;

#[test]
fn simple() {
    let lang = Language::with(HashMap::from([
        (Cow::from("glob"), 'I'),
        (Cow::from("prok"), 'V'),
        (Cow::from("pish"), 'X'),
        (Cow::from("tegj"), 'L'),
    ]));

    let price_set: HashMap<&str, Decimal> =
        HashMap::from([("Gold", dec!(10)), ("Silver", dec!(10)), ("Iron", dec!(10))]);

    let roman = lang.translate("pish glob prok").unwrap();
    let decimal = u32::from(roman);

    // unwrap is safe, since Gold is inserted just a few lines ago
    let gold_price = price_set.get(&"Gold").unwrap();
    let shopping_bill = *gold_price * Decimal::from(decimal);

    assert_eq!(shopping_bill, dec!(140));
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
        HashMap::from([("Gold", dec!(10)), ("Silver", dec!(10)), ("Iron", dec!(10))]);

    let shopping_bill = lang.query(&price_set, "How many Credits is pish glob prok Gold?");

    assert_eq!(shopping_bill, dec!(140));
}
