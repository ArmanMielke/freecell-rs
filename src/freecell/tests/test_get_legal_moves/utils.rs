use std::fmt::Debug;



pub fn assert_eq_ignore_order<T: PartialEq + Debug>(actual: Vec<T>, expected: Vec<T>) {
    let expected_len = expected.len();

    for e in expected {
        if !actual.contains(&e) {
            panic!("actual does not contain {:?}", e)
        }
    }

    assert_eq!(actual.len(), expected_len);
}
