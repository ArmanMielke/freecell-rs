use freecell::Foundations;

fn create_foundations() -> [Foundations; 3] {
    [
        // empty
        "".parse().unwrap(),
        // partly filled
        "5H QS".parse().unwrap(),
        // full
        "KD AH 7S TC".parse().unwrap(),
    ]
}

#[test]
fn test_display() {
    for foundation in &create_foundations() {
        assert_eq!(Ok(foundation.clone()), foundation.to_string().parse());
    }
}

#[test]
fn test_debug() {
    for foundation in &create_foundations() {
        assert_eq!(Ok(foundation.clone()), format!("{:?}", foundation).parse());
    }
}
