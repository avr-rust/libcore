# avr-rust libcore

This is a copied version of libcore with various pieces disabled to work around limitations in the current AVR backend.

The goal of this library is to *no longer exist*. The [issues][] are intended to track each piece of libcore that AVR cannot currently support.

[issues]: https://github.com/avr-rust/libcore/issues

## Usage

The version of this library needs to match the original Rust commit
that your compiler is built from. This code is from
[rust-lang/rust@26015da0][fork-point].

You can add it to your [xargo][] configuration file (`Xargo.toml`) as such:

```
[target.avr-atmega328p.dependencies]
core = { git = "https://github.com/avr-rust/libcore", branch = "rust-26015da0" }
```

[fork-point]: https://github.com/rust-lang/rust/commit/26015da01497b4014fc4f2ecedee5a7090c354e6
[xargo]: https://github.com/japaric/xargo

## Contributing

Check out the [issues][] to find a piece of missing functionality that interests you. Revert the corresponding commit and build the new libcore to ensure that there's still a code generation problem. Then start tracing through the LLVM code to figure out what needs to change! 
