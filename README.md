# Snappy

A minimal, dependency-free Rust crate for snapshot testing. `snappy` allows you to perform snapshot tests by comparing output values against previously recorded snapshots and includes a binary tool to review and manage snapshots.

## Features

- **Dependency-Free**: No external crates required.
- **Snapshot Testing**: Compare test outputs against expected results.
- **Binary Tool**: Interactively approve or reject snapshot changes.
- **Colorized Output**: Highlights differences in terminal outputs.

## Installation

Install the `snappy` binary directly from GitHub:

```bash
cargo install --git https://github.com/copiumnicus/snap
```

## Usage

### Adding `snappy` as a Dependency

Add `snappy` to your `Cargo.toml`:

```toml
[dependencies]
snappy = { git = "https://github.com/copiumnicus/snap" }
```

### Using the `snappy` Function in Tests

```rust
use snappy::snap;

#[test]
fn test_example_output() {
    let output = your_function_to_test();
    snappy("example_output", output);
}
```

### Reviewing Snapshots with `snapshot_tool`

After running your tests, review and manage snapshots:

```bash
snappy
```

- **Approve**: Accept the new snapshot.
- **Reject**: Discard the changes.
- **Skip**: Review later.
