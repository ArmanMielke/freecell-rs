# Changelog

## Unreleased Changes

- Use `FromStr` instead of `TryFrom<String>` and `TryFrom<&str>`
- Add serialization and deserialization using `serde`
- Change freecells from `ArrayVec<[Card; 4]>` to `[Option<Card>; 4]`
- Add functions for parsing types
    - `parse_cascade(string) -> Result<Cascade, String>`
    - `parse_freecells(string) -> Result<Freecells, String>`
    - Implement `FromStr` for `Foundations`
    - Implement `FromStr` for `GameState`
- Rename function `game_state_parser::parse_file(path)` to `GameState::from_file(path)`
- Rename function `rank_from_string(string)` to `parse_rank(string)`


## Releases

- **0.1.0**: Initial release

