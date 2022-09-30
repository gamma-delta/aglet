# Another Grid Library Except Terrible

This is a successor to my old crate [cogs_gamedev](https://crates.io/crates/cogs-gamedev)'s `grids` module.
It adds opinionated integer-based coordinates and directions, along with some other useful things:

- Iterators over areas, edges of areas, and lines
- `Grid<T>`, which is like a `HashMap<Coord, T>` but faster

Also supports serde, under the default feature `serde`.
