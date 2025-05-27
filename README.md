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

Add to your `Cargo.toml`:

```toml
enum-toggles = "version = "0.1.2"
```

### Example 1: Basic usage

- File `toggles.txt` conains:

```txt
0 FeatureA
1 FeatureB
```

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
toggles.set_by_name("FeatureB", true); // Mapped to MyToggle::FeatureB
toggles.load_from_file("toggles.txt"); // Load toggles state from file
println!("{}", toggles);
```

### Example 2: With concucrency context

```rust
use once_cell::sync::Lazy;
use std::env;
use log::{warn};
#[derive(AsRefStr, EnumIter, PartialEq, Copy, Clone, Debug)]
pub enum MyToggle {
    FeatureA,
    FeatureB,
}
pub static TOGGLES: Lazy<Toggles<EnumToggle>> = Lazy::new(|| {
    let mut toggle = Toggles::new();
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
```
