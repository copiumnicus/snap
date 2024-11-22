# Snappy

A minimal, dependency-free Rust crate for snapshot testing. `snappy` allows you to perform snapshot tests by comparing output values against previously recorded snapshots and includes a binary tool to review and manage snapshots.

## Features

- **Dependency-Free**: No external crates required.
- **Snapshot Testing**: Compare test outputs against expected results.
- **Binary Tool**: Interactively approve or reject snapshot changes.
- **Colorized Output**: Highlights differences in terminal outputs.

## Installation

Install the `snappy` binary directly from GitHub, pinning to the `v0.1.0` tag:

```bash
cargo install --git https://github.com/copiumnicus/snappy --tag v0.1.0
```

## Usage

### Adding `snappy` as a Dependency

Add `snappy` to your `Cargo.toml`, specifying the `v0.1.0` tag to pin the dependency:

```toml
[dependencies]
snappy = { git = "https://github.com/copiumnicus/snappy", tag = "v0.1.0" }
```

### Using the `snap` Function in Tests

```rust
use snappy::snap;

#[test]
fn test_example_output() {
    let output = your_function_to_test();
    snap("example_output", output);
}
```

### Reviewing Snapshots with `snappy`

After running your tests, review and manage snapshots:

```bash
snappy
```

- **Approve**: Accept the new snapshot.
- **Reject**: Discard the changes.
- **Skip**: Review later.

## License

Licensed under the [MIT License](LICENSE).
