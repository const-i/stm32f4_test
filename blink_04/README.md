# App Template - Rust Embedded

Based on the Rust Embedded book: <https://docs.rust-embedded.org/book/start/qemu.html>

Specifically, uses the template linked here: <https://github.com/rust-embedded/cortex-m-quickstart>    -- This is depricated

Hence, we use the template linked here: <https://github.com/knurling-rs/app-template>

With the files in this repo, we are able to run the following commands:

```
cargo build

cargo run

cargo embed
```


## Dependencies

### 1. `flip-link`:

```bash
cargo install flip-link
```

### 2. `probe-rs`:

Install probe-rs by following the instructions at <https://probe.rs/docs/getting-started/installation/>.

### 3. [`cargo-generate`]:

```bash
cargo install cargo-generate
```

[`cargo-generate`]: https://crates.io/crates/cargo-generate

