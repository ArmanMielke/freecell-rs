use super::super::Foundations;

#[test]
fn test_default() {
    let default: Foundations = Default::default();
    assert_eq!(Foundations::new(), default)
}
