use super::super::parse_file;
use super::super::error_messages::ERR_TOO_MANY_CASCADES;


#[test]
fn test_easy_10_instead_of_t() {
    let actual = parse_file("example-inputs/invalid/easy-10-instead-of-T.txt");
    let expected = Err(err_card_code_not_length_2!("10S"));
    assert_eq!(actual, expected);
}

#[test]
fn test_easy_duplicate_card() {
    /*
    let actual = parse_file("example-inputs/invalid/easy-duplicate-card.txt");
    let expected = Err("TODO".to_string());
    assert_eq!(actual, expected);
    */
}

#[test]
fn test_easy_extra_cascade() {
    let actual = parse_file("example-inputs/invalid/easy-extra-cascade.txt");
    let expected = Err(String::from(ERR_TOO_MANY_CASCADES));
    assert_eq!(actual, expected);
}

#[test]
fn test_easy_invalid_card_value() {
    let actual = parse_file("example-inputs/invalid/easy-invalid-card-value.txt");
    let expected = Err(err_could_not_parse_card_value!('X'));
    assert_eq!(actual, expected);
}

#[test]
fn test_easy_invalid_suit() {
    let actual = parse_file("example-inputs/invalid/easy-invalid-suit.txt");
    let expected = Err(err_could_not_parse_suit!('X'));
    assert_eq!(actual, expected);
}

#[test]
fn test_easy_missing_card() {
    // TODO
}

#[test]
fn test_easy_missing_card_2() {
    // TODO
}
