# Advent of Code 2020

Solutions for AoC 2020 written in not-very-idiomatic Rust.

Dumb, fast solutions are feature-gated behind a `dumb` flag. Run/compile with `--features dumb`
to activate.

## Project goals

First and foremost to make it all the way to the end for once. Additionally, I will try to
accomplish the following things:

### High-priority

* [x] Optimize for legibility and communication of intent rather than absolute speed
* [ ] Terminal UI for choosing a desired solution using Crossterm/Termion/something else
* [ ] Use traits to define solutions, create clean abstractions
* [ ] Don't copy-paste reusable parts from previous days, refactor into module instead

### Medium priority

* [ ] Find a way to make all files in eg. `solutions/` automatically appear as TUI options (build script?)
* [ ] Set up CI/CD pipeline
* [ ] Benchmarking setup using eg. Criterion

### Low priority

* [ ] Build a cross-platform binary

### Sure would be nice but not going to happen in a million years

* [ ] Compile to WASM and embed in browser
