use std::borrow::Cow;

use hashbrown::HashMap;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use intra::Ford;
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
        HashMap::from([("Gold", dec!(10)), ("Silver", dec!(5)), ("Iron", dec!(1))]);

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
        HashMap::from([("Gold", dec!(10)), ("Silver", dec!(5)), ("Iron", dec!(1))]);

    let shopping_bill = lang
        .query(&price_set, "How many Credits is pish glob prok Gold?")
        .unwrap();

    assert_eq!(shopping_bill, dec!(140));
}

#[test]
fn assistant() {
    let mut ford = Ford::new();

    assert_eq!(ford.query("glob is I").unwrap(), None);
    assert_eq!(ford.query("prok is V").unwrap(), None);
    assert_eq!(ford.query("pish is X").unwrap(), None);
    assert_eq!(ford.query("tegj is L").unwrap(), None);

    assert_eq!(ford.query("glob glob Silver is 34 Credits").unwrap(), None);
    assert_eq!(ford.query("glob prok Gold is 57800 Credits").unwrap(), None);
    assert_eq!(ford.query("pish pish Iron is 3910 Credits").unwrap(), None);

    assert_eq!(
        ford.query("how much is pish tegj glob glob ?").unwrap(),
        Some("pish tegj glob glob is 42".to_string())
    );
    assert_eq!(
        ford.query("how many Credits is glob prok Silver ?")
            .unwrap(),
        Some("glob prok Silver is 68 Credits".to_string())
    );
    assert_eq!(
        ford.query("how many Credits is glob prok Gold ?").unwrap(),
        Some("glob prok Gold is 57800 Credits".to_string())
    );
    assert_eq!(
        ford.query("how many Credits is glob prok Iron ?").unwrap(),
        Some("glob prok Iron is 782 Credits".to_string())
    );
}
