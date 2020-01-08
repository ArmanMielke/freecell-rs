use freecell::game_state_parser::parse_file;

#[test]
fn test_easy_10_instead_of_t() {
    assert_eq!(
        parse_file("test-inputs/easy-10-instead-of-T.txt"),
        parse_file("test-inputs/easy.txt")
    );
}
