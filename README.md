# Noship

[![Build](https://github.com/kafji/noship/workflows/Build/badge.svg)](https://github.com/kafji/noship/actions?query=workflow%3ABuild)
[![crates.io](https://img.shields.io/crates/v/noship)](https://crates.io/crates/noship)
[![Docs](https://docs.rs/noship/badge.svg)](https://docs.rs/noship)
[![Changelog](https://img.shields.io/badge/Changelog-666)](CHANGELOG.md)
[![Source](https://img.shields.io/badge/Source-666)](https://github.com/kafji/noship)

Noship is a macro that acts similar to [todo!](https://doc.rust-lang.org/std/macro.todo.html) but will throw a compilation error when compiled on release profile.

Think of this as todo but it refuses to compile on release mode thus prohibit you to release incomplete code in case you forgot about it.

## Usage

```rust
// src/main.rs

use noship::noship;

fn main() {
    let raining = false;

    if raining {
        noship!();
    } else {
        println!("going out...");
    }
}

```

```
$ cargo run
going out...
```

```
$ cargo run --release
error: release blocked
 --> src/main.rs:7:9
  |
7 |         noship!();
  |         ^^^^^^^^^^
  |
```

<br>

## License & Contribution

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
