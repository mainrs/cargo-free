# cargo-free

[![Maintenance](https://img.shields.io/badge/maintenance-actively%20maintained-brightgreen.svg)](https://github.com/SirWindfield/cargo-free)
[![crates.io](https://img.shields.io/crates/v/cargo-free.svg)](https://crates.io/crates/cargo-free)
[![crates.io](https://img.shields.io/crates/d/cargo-free)](https://crates.io/crates/cargo-free)

> A cargo subcommand to check if a given crate name is available.

## Installation

```text
cargo install cargo-free --locked
```

If you do not want a colored help message and colored output, you can install without the feature:

```text
cargo install cargo-free --locked --no-default-features
```

## Usage

### CLI

```text
$ cargo free name-to-check
Available
```

```text
$ cargo free name1 name2 name3
name1: Available
name2: Available
name3: Unavailable
```

### Library

```rust
use cargo_free::{check_availability, Availability};

let availability = check_availability("serde");
assert_eq!(availability, Availability::Unavailable);
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
