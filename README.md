# Enum toggles

A generic Rust library for managing toggles/flags using enums and bitvec.

This crate provides a toggle manager that can load from a file.
Toggle states are read-only and accessed in O(1) time.
There's a direct relationship where each string name corresponds to a unique name in the enum.

## Features

- Type-safe toggles based on enums
- Efficient storage with bitvec
- Load toggle states from file
- Display and serialization helpers

## Usage

### Example 1: Basic usage

- Add deppendency to `Cargo.toml`:
```bash
cargo add enum-toggles strum strum_macros
```

- File `toggles.yaml` conains:

```yaml
FeatureA: 0
FeatureB: 1
```

```rust
use enum_toggles::EnumToggles;
use strum_macros::{AsRefStr, EnumIter};

#[derive(AsRefStr, EnumIter, PartialEq)]
enum MyToggle {
    FeatureA,
    FeatureB,
}

let mut toggles: EnumToggles::<MyToggle> = EnumToggles::new();
toggles.set(MyToggle::FeatureA as usize, true);
toggles.set_by_name("FeatureB", true); // Mapped to MyToggle::FeatureB
// toggles.load_from_file("toggles.yaml"); // Load toggles state from file
println!("{:?}", toggles);
```

### Example 2: With concucrency context

```rust
use enum_toggles::EnumToggles;
use log::warn;
use once_cell::sync::Lazy;
use std::env;
use std::ops::Deref;
use strum_macros::{AsRefStr, EnumIter};

#[derive(AsRefStr, EnumIter, PartialEq)]
enum MyToggle {
    FeatureA,
    FeatureB,
}

pub static TOGGLES: Lazy<EnumToggles<MyToggle>> = Lazy::new(|| {
    let mut toggle:EnumToggles<MyToggle> = EnumToggles::new();
    let filepath = env::var("TOGGLES_FILE");
    match filepath {
        Ok(path) => {
            if !path.is_empty() {
                toggle.load_from_file(&path)
            }
        }
        Err(_) => warn!("Environment variable TOGGLES_FILE not set"),
    }
    toggle
});

println!("{:?}", TOGGLES.deref());
```
