# Freecell

A Freecell solver written in Rust

**THIS PROJECT IS A WORK IN PROGRESS AND NOT YET FUNCTIONAL**

## Explanation for Card Codes (Could be used in some top level doc)

Each card can be described using a card code, a string of two characters.
The first character denotes the card's rank:
- 'A' -> Ace (alternate form '1' is also allowed)
- '2' -> 2
- ...
- '9' -> 9
- 'T' -> 10
- 'J' -> Jack
- 'Q' -> Queen
- 'K' -> King

The second character denotes the suit:
- 'C' -> Club
- 'S' -> Spade
- 'H' -> Heart
- 'D' -> Diamond

card codes are case-insensitive.


## TODO

- measure and visualize performance
    - https://bheisler.github.io/criterion.rs/book/criterion_rs.html
    - https://github.com/ferrous-systems/flamegraph
- try binary heap instead of priority queue?

### API Guidelines

[Checklist](https://rust-lang.github.io/api-guidelines/checklist.html)

Some relevant items include:

- Data structures implement Serde's Serialize, Deserialize
    - https://rust-lang.github.io/api-guidelines/interoperability.html#c-serde
    - GameState is probably a data structure
    - "If a crate does not already depend on Serde for other reasons, it may wish to gate Serde impls behind a Cargo cfg."
    - The link includes a tutorial
- All checklist items regarding documentation
- Caller decides where to copy and place data
    - https://rust-lang.github.io/api-guidelines/flexibility.html#c-caller-control
    - "If a function does not require ownership of an argument, it should take a shared or exclusive borrow of the argument rather than taking ownership and dropping the argument."
- Functions minimize assumptions about parameters by using generics
    - https://rust-lang.github.io/api-guidelines/flexibility.html#c-generic
- All public types implement Debug
- Maybe relevant:
    - Functions expose intermediate results to avoid duplicate work
    - Traits are object-safe if they may be useful as a trait object
    - Functions validate their arguments
    - All checklist items under future proofing
    - All checklist items under necessities

### Style Guide

- [**Imports**](https://doc.rust-lang.org/1.0.0/style/style/imports.html)
    - Prefer fully importing types/traits while module-qualifying functions
- [Notes on doc comments](https://doc.rust-lang.org/1.0.0/style/style/comments.html#doc-comments)
- [Apparently there should be at most 99 characters in each line](https://doc.rust-lang.org/1.0.0/style/style/whitespace.html)
- [Naming convention for getters and setters](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html#getter/setter-methods-[rfc-344])
- [Organization](https://doc.rust-lang.org/1.0.0/style/style/organization.html)

Also chapter 3 and everything that comes after in this document: https://doc.rust-lang.org/1.0.0/style/README.html
