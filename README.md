# bytefmt

`bytefmt` is a dependency-free Rust library and command-line tool for parsing,
formatting, and converting human-readable byte sizes.

## Features

- Parse values such as `1024`, `1.5 MiB`, `2KB`, and `3.25 GB`.
- Distinguish binary units (`KiB`, `MiB`, ...) from decimal units (`KB`, `MB`, ...).
- Format byte counts with selectable unit systems and precision.
- Convert between binary and decimal representations.
- No runtime dependencies.

## Build

```sh
cargo build --release
```

The executable will be available at `target/release/bytefmt`.

## CLI usage

Parse a human-readable size into bytes:

```sh
$ bytefmt parse "1.5 MiB"
1572864
```

Format a byte count:

```sh
$ bytefmt format 1536 --unit binary
1.50 KiB
```

Convert a size to another unit system:

```sh
$ bytefmt convert "1.5 MB" --to binary
1.43 MiB
```

## Library usage

```rust
use bytefmt::{ByteSize, UnitSystem};

fn main() {
    let size = ByteSize::parse("2.5 GiB").unwrap();
    println!("{}", size.0);
    println!("{}", size.format(UnitSystem::Decimal, 2));
}
```

## Units

Binary units use powers of 1024: `KiB`, `MiB`, `GiB`, `TiB`, `PiB`, `EiB`.
Decimal units use powers of 1000: `KB`, `MB`, `GB`, `TB`, `PB`, `EB`.
Bare suffixes such as `K`, `M`, and `G` are treated as binary units.

## License

MIT
