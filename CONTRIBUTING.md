# Contributing

Thanks for your interest in contributing to this project.

## Development setup

- **Rust:** 1.85+ (see [rust-toolchain.toml](./rust-toolchain.toml))
- **Node:** 24+ with [Bun](https://bun.sh) (see [package.json](./package.json))

```sh
# Install Rust (rustup will use rust-toolchain.toml)
rustup show

# Install JS tooling and hooks
bun install
```

## Commands

| Command | Description |
|--------|-------------|
| `bun run format` | Format Rust code (`cargo fmt`) |
| `bun run lint` | Lint with Clippy (`cargo clippy`) |
| `bun run test` | Run tests (`cargo test`) |
| `bun run test:coverage` | Coverage with [cargo-tarpaulin](https://github.com/cargo-tarpaulin/cargo-tarpaulin) (optional; run `cargo install cargo-tarpaulin` first) |
| `bun run check` | Lint/format JS/TS (Biome) |
| `bun run commit` | Commit with Commitizen (conventional commits) |

## Commit messages

We use [Conventional Commits](https://www.conventionalcommits.org/) and [commitlint](https://commitlint.js.org/). Use:

```sh
bun run commit
```

Examples: `feat: add foo`, `fix: bar`, `chore(deps): bump x`.

## Pull requests

1. Branch from `main`, make changes, run `bun run format`, `bun run lint`, `bun run test`.
2. Push and open a PR. CI will run format check, Clippy, and tests on Ubuntu, Windows, and macOS.
3. Keep the scope small and the title/commits clear.

## Releasing

Maintainers use:

- `bun run bump:patch` / `bump:minor` / `bump:major` to bump version in `Cargo.toml`, commit, tag, and push.
- `bun run release` to generate changelog and sync a GitHub release.

Publishing to crates.io is done manually when ready.
