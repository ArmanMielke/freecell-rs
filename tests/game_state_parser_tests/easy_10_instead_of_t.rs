use freecell::GameState;

#[test]
fn test_easy_10_instead_of_t() {
    assert_eq!(
        GameState::from_file("test-inputs/easy-10-instead-of-T.txt"),
        GameState::from_file("test-inputs/easy.txt")
    );
}
