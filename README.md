# Enum toggles

A generic Rust library for managing toggles/flags using enums and bitvec.

## Features

- Type-safe toggles based on enums
- Efficient storage with bitvec
- Load toggle states from file
- Display and serialization helpers

## Usage

Add to your `Cargo.toml`:

```toml
enum-toggles = "0.1"
```

Example:

```rust
use enum_toggles::EnumToggles;
use strum_macros::{AsRefStr, EnumIter};

#[derive(AsRefStr, EnumIter, PartialEq, Copy, Clone, Debug)]
pub enum MyToggle {
    FeatureA,
    FeatureB,
}

let mut toggles = EnumToggles::<MyToggle>::new();
toggles.set_enum(MyToggle::FeatureA, true);
println!("{}", toggles);
```
