# Noship

[![Build](https://github.com/kafji/noship/workflows/Build/badge.svg)](https://github.com/kafji/noship/actions?query=workflow%3ABuild)

[![Docs](https://docs.rs/noship/badge.svg)](https://docs.rs/noship)

Noship is a macro that acts similar as [todo!](https://doc.rust-lang.org/std/macro.todo.html) but will throw a compilation error when compiled on release profile.

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

## Installation

```yml
# Cargo.toml

noship = "0.0.*"
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