# TODO

- **[v2+]** add `fn random_game() -> GameState` or `GameState::random() -> GameState`
    - there may be a better name
    - can be a separate feature so not everyone needs to compile the rand crate and its 4 dependencies
    - how much effort would it be to add some separate GameState generation that is compatible with the original Windows FreeCell?
- **[v1]** remove optimisations from GameState
    - document those optimisations so that they can be re-implemented later
    - e.g. player can choose in which freecell to put a card (Freecells as `[Option<Card>; 4]`)
- **[v2+]** add a second game state struct with performance optimisation
    - introduce a trait implemented by both structs?? (if not, consider renaming `legal_moves()` to `moves()`)
    - optimisations:
        - different implementation of `eq()` where order of cascades doesn't matter
        - all freecells are one card collection together, implemented as set-like data structure
        - different implementation of `legal_moves()` that returns fewer moves if some of them are equivalent (e.g. if there are multiple empty cascades)
        - if there are safe moves, `legal_moves()` only returns the first one of those
        - note: a possible optimisation would be that aces cannot be put anywhere other than on foundations, but that is already included in the safe move optimisation
- **[v2+]** measure and visualize performance
    - https://bheisler.github.io/criterion.rs/book/criterion_rs.html
    - https://github.com/ferrous-systems/flamegraph
- **[v1]** explain the [rules](README.md) in more detail (find a better source?)


## API Guidelines **[v1]**

[Checklist](https://rust-lang.github.io/api-guidelines/checklist.html)

Some relevant items include:

- All checklist items regarding documentation
- Caller decides where to copy and place data
    - https://rust-lang.github.io/api-guidelines/flexibility.html#c-caller-control
    - "If a function does not require ownership of an argument, it should take a shared or exclusive borrow of the argument rather than taking ownership and dropping the argument."
- Functions minimize assumptions about parameters by using generics
    - https://rust-lang.github.io/api-guidelines/flexibility.html#c-generic
- Maybe relevant:
    - Functions expose intermediate results to avoid duplicate work
    - Traits are object-safe if they may be useful as a trait object
    - Functions validate their arguments
    - All checklist items under future proofing
    - All checklist items under necessities


### Style Guide

- [**Imports**](https://doc.rust-lang.org/1.0.0/style/style/imports.html)
    - Prefer fully importing types/traits while module-qualifying functions
- [Apparently there should be at most 99 characters in each line](https://doc.rust-lang.org/1.0.0/style/style/whitespace.html)

Also chapter 3 and everything that comes after in this document: https://doc.rust-lang.org/1.0.0/style/README.html
