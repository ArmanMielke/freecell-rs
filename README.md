# FreeCell

[![build](https://github.com/Arman-Mielke/freecell-rs/workflows/build/badge.svg)](https://github.com/Arman-Mielke/freecell-rs/actions)
[![crates.io](https://img.shields.io/crates/v/freecell)](https://crates.io/crates/freecell)
[![docs.rs](https://docs.rs/freecell/badge.svg)](https://docs.rs/freecell)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/freecell)](#license)


Game objects and rules for the solitaire card game [FreeCell](https://en.wikipedia.org/wiki/FreeCell), written in Rust.
The goal of this library is to aid in the implementation of a FreeCell game, as well as solvers for that game.

**THIS PROJECT IS A WORK IN PROGRESS.**
While the basic features are mostly implemented, most of the optimisations useful for efficient solvers are still missing.



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



## Examples

**TODO**



## Contributing

**TODO**



## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
