# Advent of Code

- `export AOC_SESSION_TOKEN='TOKEN'`
- in a new buffer within helix: `:insert-output curl -sSL 'https://adventofcode.com/<YEAR>/day/<DAY>/input' -H \"cookie: session=${AOC_SESSION_TOKEN}\"`
- in a new buffer within helix: `:insert-output cheat rust/aoc` then `:set-language rust` or write the file

## Tests

`cargo install --locked bacon`
`bacon test -- --bin BINARY_NAME`

---

see the [Examples](./examples/) and [Solutions](./src/bin/) for more.
