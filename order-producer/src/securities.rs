use rand::seq::SliceRandom;

pub fn get_securities_list<'a>() -> Vec<&'a str> {
    vec!["EUR/USD", "USD/GBP", "USD/JPY", "USD/CAD", "USD/CHF"]
}

pub fn random_security() -> String {
    let mut rng = rand::thread_rng();
    get_securities_list().choose(&mut rng).unwrap().to_string()
}

#[test]
fn get_all_securities_returns_5() {
    let length = crate::securities::get_securities_list().len();
    assert_eq!(length, 5);
}

#[test]
fn random_security_returns_atleast_1() {
    let sec = crate::securities::random_security();
    assert_eq!(sec.len(), 7);
}
