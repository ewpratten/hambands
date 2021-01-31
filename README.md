# HamBands
[![Crates.io](https://img.shields.io/crates/v/hambands)](https://crates.io/crates/hambands) [![Documentation](https://docs.rs/hambands/badge.svg)](https://docs.rs/hambands) ![Build](https://github.com/Ewpratten/hambands/workflows/Build/badge.svg)

HamBands is a small Rust library for checking which amateur radio band a frequency belongs to.

## Installation

### From Crates.io

```sh
cargo install hambands
```

### From Source

```sh
git clone https://github.com/ewpratten/hambands
cd rbn
cargo install --path .
```

## Contributing

If you are interested in adding support for more amateur (or broadcast) radio bands, check out [`src/band/mod.rs`](https://github.com/Ewpratten/hambands/blob/master/src/band/mod.rs). This file should be fairly self-explanatory. All entries are automatically validated at unit test time by `cargo`.

