# Changelog

## Unreleased Changes

- Add serialization and deserialization using `serde`
- Change `Freecells` from `ArrayVec<[Card; 4]>` to `[Option<Card>; 4]`
- Change `Cascade` from a type alias to a tuple struct
- Add functions for parsing types
    - `parse_freecells(string) -> Result<Freecells, String>`
    - Implement `FromStr` for
        - `Card` (this replaces `TryFrom<String>` and `TryFrom<&str>`)
            - **TODO** check whether I did the same thing for any other struct
        - `Cascade`
        - `Foundations`
        - `GameState`
- Rename function `game_state_parser::parse_file(path)` to `GameState::from_file(path)`
- Rename function `rank_from_string(string)` to `parse_rank(string)`


## Releases

- **0.1.0**: Initial release

