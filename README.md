ezconf
======

A library to add configuration options to your project with as little
boilerplate as possible.

All macros will cache the value for fast access times (Although, if it is
really time critical, you should save it to a variable yourself)

**Important**: Due to the way this crate was implemented, it is necessary
to import the `lazy_static` macro in your project

## Example ##

```rust
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate ezconf;

ezconf_file!(CONFIG = "tests/test.toml");

fn main() {
    let mut value = 100.0f64;
    // This is supposed to be a very complex algorithm
    for i in 0..1000 {
        // The default value (0.1) will be used if the value
        // does not exist in the config
        let CONSTANT = ezconf_float!(CONFIG: "float.a", 0.1);

        value = value.powf(CONSTANT);
    }
}
```

## License ##
ezconf is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
