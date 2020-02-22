<h1 align="center">:clubs::hearts::spades:️:diamonds:️<br/>FreeCell</h1>

<p align="center">
    <a style="margin: 0 5px" href="https://github.com/Arman-Mielke/freecell-rs/actions">
        <img src="https://img.shields.io/github/workflow/status/Arman-Mielke/freecell-rs/build" alt="build">
    </a>
    <a style="margin: 0 5px" href="https://codecov.io/gh/Arman-Mielke/freecell-rs">
        <img src="https://img.shields.io/codecov/c/github/Arman-Mielke/freecell-rs" alt="coverage">
    </a>
    <a style="margin: 0 5px" href="https://crates.io/crates/freecell">
        <img src="https://img.shields.io/crates/v/freecell" alt="crates.io">
    </a>
    <a style="margin: 0 5px" href="https://docs.rs/freecell">
        <img src="https://docs.rs/freecell/badge.svg" alt="docs.rs">
    </a>
</p>
<br/>

Game objects and rules for the solitaire card game [FreeCell](https://en.wikipedia.org/wiki/FreeCell), written in Rust.
The goal of this crate is to aid in the implementation of both a FreeCell game and solvers for FreeCell.

**THIS PROJECT IS A WORK IN PROGRESS.**
While the basic features are mostly implemented, most of the optimisations useful for efficient solvers are still missing.




## Table of Contents

- [FreeCell Rules](#FreeCell-Rules)
- [Usage](#Usage)
    - [Parsing](#Parsing)
    - [Serialization](#Serialization)
- [Roadmap](#Roadmap)
- [Contributing](#Contributing)
- [License](#License)




## FreeCell Rules

This library uses the following rules of FreeCell:

- Construction and layout
    - One [standard 52-card deck](https://en.wikipedia.org/wiki/Standard_52-card_deck) is used.
    - There are four open *freecells* and four open [*foundations*](https://en.wikipedia.org/wiki/Glossary_of_patience_terms#Foundation).
    - Cards are dealt face-up into eight [*cascades*](https://en.wikipedia.org/wiki/Glossary_of_patience_terms#Deal_terms), four of which comprise seven cards each and four of which comprise six cards each.
- Building during play
    - The top card of each cascade begins a [tableau](https://en.wikipedia.org/wiki/Glossary_of_patience_terms#Layout_terms).
    - Tableaux must be built down by alternating colors.
    - Foundations are built up by suit.
- Moves
    - Any freecell card or top card of any cascade may be moved to build on a tableau, or moved to an empty cell, an empty cascade, or its foundation.
    - Complete or partial tableaus may be moved to build on existing tableaus, or moved to empty cascades, by recursively placing and removing cards through intermediate locations.
- The game is won after all cards are moved to their foundation piles.

All cards are face-up at all times.
Not all deals are solvable, but the probability of an unsolvable deal is very low. It is estimated that 99.999% of possible deals are solvable.

Source: [Wikipedia](https://en.wikipedia.org/wiki/FreeCell#Rules)




## Usage

The central struct of this crate is [`GameState`](https://docs.rs/freecell/latest/freecell/struct.GameState.html), which represents the state of the board (i.e. the positions of all cards) at one point during the course of a game.
It can be used to check whether a given move is allowed in this state...
```rust
// TODO add example
```
...or to generate all moves that the player can make in this game state, each of them paired with the game state they would result in.
```rust
// TODO add example
```

On top of these, `GameState` provides numerous other methods useful for dealing with board states.



### Parsing

Almost all structs provided by this crate implement `FromStr`.
Parsing is case-insensitive for each of them.
The format for parsing any of those structs is the same as the format used by their `Debug` and `Display` implementations.
This means that the output of `Display` and `Debug` can be converted back to the original object:
```rust
// TODO add example (cascade)
```

In the following the formats for each of the structs will be explained in detail.

#### Parsing [`Card`s](https://docs.rs/freecell/latest/freecell/struct.Card.html)

`Card`s can be represented using a short or a long format.

The short format consists of two or three characters where the first one or two characters denote the card's rank and the last character denotes the suit.
The character(s) denoting the rank must either be a number between 1 and 13 or one of "A" for Ace, "T" for 10, "J" for Jack, "Q" for Queen or "K" for King.
The character denoting the suit must be one of "C" for Club, "S" for Spade, "H" for Heart or "D" for Diamond.
```rust
assert_eq!(Ok(Card { suit: Diamond, rank: ACE }), "AD".parse());
assert_eq!(Ok(Card { suit: Club, rank: 4 }), "4C".parse());
assert_eq!(Ok(Card { suit: Heart, rank: 10 }), "th".parse());
```

The long format is of the form `<rank> of <suit>`, optionally with an `s` at the end.
The rank can either be the rank's number (in digits) or its name.
```rust
assert_eq!(Ok(Card { suit: Diamond, rank: JACK }), "Jack of Diamonds".parse());
assert_eq!(Ok(Card { suit: Club, rank: 3 }), "3 of club".parse());
```

#### Parsing [`Cascade`s](https://docs.rs/freecell/latest/freecell/struct.Cascade.html)

**TODO**

#### Parsing [`Card`s](https://docs.rs/freecell/latest/freecell/struct.Card.html)

**TODO**

#### Parsing [`Card`s](https://docs.rs/freecell/latest/freecell/struct.Card.html)

**TODO**

#### Parsing [`GameState`s](https://docs.rs/freecell/latest/freecell/struct.GameState.html)

**TODO explain the format for `GameState`**

`GameState::from_file(path)` can be used if the game state's description is stored in a file.



### Serialization

If the `"serialization"` feature is enabled, the `Serialize` and `Deserialize` traits from
[`serde`](https://docs.rs/serde) are implemented for all types exported by this crate.




## Roadmap

- [ ] First release
    - [X] Provide structs for implementing a FreeCell game
    - [ ] Allow parsing those structs from strings
- [ ] Allow generating a random game
- [ ] Provide optimised structs for implementing a FreeCell solver

Features are only completed once they are thoroughly tested and documented.




## Contributing

Feedback, suggestions and other contributions are always welcome!

If you notice a bug, please open an [issue](https://github.com/Arman-Mielke/freecell-rs/issues).

For pull requests, we ask that all contributed code follows Rust's [style guidelines](https://doc.rust-lang.org/1.12.1/style/) (within reason).




## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
