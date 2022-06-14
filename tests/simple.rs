use intra::Ford;

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
