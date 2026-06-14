# mew

> a tiny coding cat with sharp claws.

```bash
mew
mew init
mew ask "what does this repo do?"
mew edit "fix this bug"
mew name set paww
mew style preview
```

## What is mew?

`mew` is an open-source Rust AI coding agent built for CLI-first workflows, low-resource users, Termux, Linux, macOS, Windows, and custom AI providers.

It is designed to be cute, fast, safe, token-efficient, and powerful.

## Current Phase

See [`docs/PHASES.md`](docs/PHASES.md).

## Development

```bash
cargo build
cargo run -p mew-cli -- --help
cargo run -p mew-cli
cargo run -p mew-cli -- doctor
cargo run -p mew-cli -- init --dry-run
cargo run -p mew-cli -- style preview
cargo run -p mew-cli -- name set paww
cargo run -p mew-cli -- name show
```

## License

Apache-2.0
