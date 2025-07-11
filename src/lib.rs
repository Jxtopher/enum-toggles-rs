//! This crate provides a toggle manager that can load from a file.
//! Toggle states are read-only and accessed in O(1) time.
//! There's a direct relationship where each string name corresponds to a unique name in the enum.
//!
//! # Example
//!
//! - File `toggles.yaml` conains:
//! ```yaml
//! FeatureA: 0
//! FeatureB: 1
//! ```
//!
//! - Basic usage
//! ```rust
//! use enum_toggles::EnumToggles;
//! use strum_macros::{AsRefStr, EnumIter};
//!
//! #[derive(AsRefStr, EnumIter, PartialEq)]
//! enum MyToggle {
//!     FeatureA,
//!     FeatureB,
//! }
//!
//! let mut toggles: EnumToggles::<MyToggle> = EnumToggles::new();
//! toggles.set(MyToggle::FeatureA as usize, true);
//! toggles.set_by_name("FeatureB", true); // Mapped to MyToggle::FeatureB
//! // toggles.load_from_file("toggles.yaml"); // Load toggles state from file
//! println!("{:?}", toggles);
//! ```
//!
//! - With concucrency context
//! ```rust
//! use enum_toggles::EnumToggles;
//! use log::warn;
//! use std::env;
//! use std::ops::Deref;
//! use std::sync::LazyLock;
//! use strum_macros::{AsRefStr, EnumIter};
//!
//! #[derive(AsRefStr, EnumIter, PartialEq)]
//! enum MyToggle {
//!     FeatureA,
//!     FeatureB,
//! }
//!
//! pub static TOGGLES: LazyLock<EnumToggles<MyToggle>> = LazyLock::new(|| {
//!     let mut toggle:EnumToggles<MyToggle> = EnumToggles::new();
//!     let filepath = env::var("TOGGLES_FILE");
//!     match filepath {
//!         Ok(path) => {
//!             if !path.is_empty() {
//!                 toggle.load_from_file(&path)
//!             }
//!         }
//!         Err(_) => warn!("Environment variable TOGGLES_FILE not set"),
//!     }
//!     toggle
//! });
//!
//! println!("{:?}", TOGGLES.deref());
//! ```
//!

use bitvec::prelude::*;
use std::fs;
use std::{collections::HashMap, fmt};
use yaml_rust::{Yaml, YamlLoader};

/// Contains the toggle value for each item of the enum T.
pub struct EnumToggles<T> {
    toggles_value: BitVec,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Default for EnumToggles<T>
where
    T: strum::IntoEnumIterator + AsRef<str> + 'static,
{
    fn default() -> Self {
        EnumToggles {
            toggles_value: bitvec![0; T::iter().count()],
            _marker: std::marker::PhantomData,
        }
    }
}

/// Handle the toggle value of an enum T.
impl<T> EnumToggles<T>
where
    T: strum::IntoEnumIterator + AsRef<str> + PartialEq + 'static,
{
    /// Create a new instance of `EnumToggles` with all toggles set to false.
    ///
    /// This operation is *O*(*n*).
    pub fn new() -> Self {
        let mut toggles: EnumToggles<T> = EnumToggles {
            toggles_value: bitvec![0; T::iter().count()],
            _marker: std::marker::PhantomData,
        };
        toggles.toggles_value.fill(false);
        toggles
    }

    /// Set all toggles value defiend in the yaml file.
    pub fn load_from_file(&mut self, filepath: &str) {
        match fs::read_to_string(filepath) {
            Ok(content) => {
                let docs = YamlLoader::load_from_str(&content).unwrap();
                let doc = &docs[0];

                if let Yaml::Hash(ref h) = doc {
                    for (key, value) in h {
                        self.set_by_name(
                            key.as_str().unwrap_or("<non-string>"),
                            value.as_i64().unwrap_or(0) == 1,
                        );
                    }
                }
            }
            Err(e) => println!("Error reading file: {}", e),
        }
    }

    /// Set the bool value of all toggles based on a HashMap.
    ///
    /// This operation is *O*(*n²*).
    pub fn set_all(&mut self, init: HashMap<String, bool>) {
        self.toggles_value.fill(false);
        for toggle in T::iter() {
            if init.contains_key(toggle.as_ref()) {
                if let Some(toggle_id) = T::iter().position(|x| x == toggle) {
                    self.set(toggle_id, init[toggle.as_ref()]);
                }
            }
        }
    }

    /// Set the bool value of a toggle by its name.
    ///
    /// This operation is *O*(*n*).
    pub fn set_by_name(&mut self, toggle_name: &str, value: bool) {
        if let Some(toggle) = T::iter().find(|t| toggle_name == t.as_ref()) {
            if let Some(toggle_id) = T::iter().position(|x| x == toggle) {
                self.set(toggle_id, value);
            }
        }
    }

    /// Set the bool value of a toggle by toggle id.
    ///
    /// This operation is *O*(*1*).
    pub fn set(&mut self, toggle_id: usize, value: bool) {
        if toggle_id >= self.toggles_value.len() {
            panic!(
                "Out-of-bounds access. The provided toggle_id is {}, but the array size is {}. Please use the default enum value.",
                toggle_id,
                self.toggles_value.len()
            );
        }
        self.toggles_value.set(toggle_id, value);
    }

    /// Get the bool value of a toggle by toggle id.
    ///
    /// This operation is *O*(*1*).
    pub fn get(&self, toggle_id: usize) -> bool {
        self.toggles_value[toggle_id]
    }
}

/// Diplay all toggles and their values.
impl<T> fmt::Debug for EnumToggles<T>
where
    T: strum::IntoEnumIterator + AsRef<str> + PartialEq + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for toggle in T::iter() {
            if let Some(toggle_id) = T::iter().position(|x| x == toggle) {
                let name = toggle.as_ref();
                writeln!(f, "{} {} ", self.get(toggle_id) as u8, name)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use strum::IntoEnumIterator;
    use strum_macros::{AsRefStr, EnumIter};

    #[derive(AsRefStr, EnumIter, PartialEq)]
    pub enum TestToggles {
        Toggle1,
        Toggle2,
    }

    #[test]
    fn test_default() {
        let toggles: EnumToggles<TestToggles> = EnumToggles::default();
        assert_eq!(toggles.toggles_value.len(), TestToggles::iter().count());
    }

    #[test]
    fn test_set_all() {
        let mut toggles: EnumToggles<TestToggles> = EnumToggles::new();
        toggles.set_all(HashMap::from([("Toggle1".to_string(), true)]));
        assert_eq!(toggles.get(TestToggles::Toggle1 as usize), true);
        assert_eq!(toggles.get(TestToggles::Toggle2 as usize), false);
    }

    #[test]
    fn test_set_by_name() {
        let mut toggles: EnumToggles<TestToggles> = EnumToggles::new();
        assert_eq!(toggles.get(TestToggles::Toggle1 as usize), false);
        toggles.set_by_name("Toggle1", true);
        assert_eq!(toggles.get(TestToggles::Toggle1 as usize), true);

        toggles.set_by_name("Undefined_Toggle", true);
    }

    #[test]
    fn test_display() {
        let toggles: EnumToggles<TestToggles> = EnumToggles::new();
        assert_eq!(format!("{:?}", toggles).is_empty(), false);
    }

    #[test]
    fn test_load_from_file() {
        // Create a temporary file
        let mut temp_file =
            tempfile::NamedTempFile::new().expect("Unable to create temporary file");

        // Write some data to the file
        writeln!(temp_file, "Toggle1: 1").expect("Unable to write to temporary file");
        writeln!(temp_file, "Toggle2: 0").expect("Unable to write to temporary file");
        writeln!(temp_file, "VAR1: 0").expect("Unable to write to temporary file");
        writeln!(temp_file, "").expect("Unable to write to temporary file");

        // Get the path of the temporary file
        let filepath = temp_file.path().to_str().unwrap();

        // Create a Toggles instance and load from the file
        let mut toggles: EnumToggles<TestToggles> = EnumToggles::new();
        toggles.load_from_file(filepath);

        // Verify that the toggles were set correctly
        assert_eq!(toggles.get(TestToggles::Toggle1 as usize), true);
        assert_eq!(toggles.get(TestToggles::Toggle2 as usize), false);
    }

    #[derive(AsRefStr, EnumIter, PartialEq)]
    pub enum DeviantToggles {
        Toggle1 = 5,
        Toggle2 = 10,
    }

    #[test]
    #[should_panic(
        expected = "Out-of-bounds access. The provided toggle_id is 5, but the array size is 2. Please use the default enum value."
    )]
    fn test_deviant_toggles() {
        let mut toggles: EnumToggles<DeviantToggles> = EnumToggles::new();
        toggles.set(DeviantToggles::Toggle1 as usize, true);
    }
}
