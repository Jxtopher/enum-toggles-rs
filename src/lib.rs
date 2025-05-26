use bitvec::prelude::*;
use log::error;
use std::io::BufRead;
use std::{collections::HashMap, fmt};

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
impl<T> EnumToggles<T>
where
    T: strum::IntoEnumIterator + AsRef<str> + PartialEq + 'static,
{
    pub fn new() -> Self {
        let mut toggles: EnumToggles<T> = EnumToggles {
            toggles_value: bitvec![0; T::iter().count()],
            _marker: std::marker::PhantomData,
        };
        toggles.toggles_value.fill(false);
        toggles
    }

    pub fn load_from_file(&mut self, filepath: &str) {
        let file = std::fs::File::open(filepath).expect("Unable to open file");
        let reader = std::io::BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() == 2 {
                        if let Ok(value) = parts[0].parse::<u8>() {
                            self.set_by_name(parts[1], value != 0);
                        }
                    }
                }
                Err(e) => {
                    error!("Error reading line: {e}");
                }
            }
        }
    }

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

    fn set_by_name(&mut self, toggle_name: &str, value: bool) {
        if let Some(toggle) = T::iter().find(|t| toggle_name == t.as_ref()) {
            if let Some(toggle_id) = T::iter().position(|x| x == toggle) {
                self.set(toggle_id, value);
            }
        }
    }

    pub fn set(&mut self, toggle_id: usize, value: bool) {
        self.toggles_value.set(toggle_id, value);
    }

    pub fn get(&self, toggle_id: usize) -> bool {
        self.toggles_value[toggle_id]
    }
}

impl<T> fmt::Display for EnumToggles<T>
where
    T: strum::IntoEnumIterator + AsRef<str> + PartialEq + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for toggle in T::iter() {
            if let Some(toggle_id) = T::iter().position(|x| x == toggle) {
                let name = toggle.as_ref();
                writeln!(f, "{} {} ", self.toggles_value[toggle_id] as u8, name)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use strum_macros::{AsRefStr, EnumIter};

    #[derive(AsRefStr, EnumIter, PartialEq)]
    pub enum TestToggles {
        Toggle1,
        Toggle2,
    }

    #[test]
    fn set_all() {
        let mut toggles: EnumToggles<TestToggles> = EnumToggles::new();
        toggles.set_all(HashMap::from([("Toggle1".to_string(), true)]));
        assert_eq!(toggles.get(TestToggles::Toggle1 as usize), true);
        assert_eq!(toggles.get(TestToggles::Toggle2 as usize), false);
    }

    #[test]
    fn set_by_name() {
        let mut toggles: EnumToggles<TestToggles> = EnumToggles::new();
        assert_eq!(toggles.get(TestToggles::Toggle1 as usize), false);
        toggles.set_by_name("Toggle1", true);
        assert_eq!(toggles.get(TestToggles::Toggle1 as usize), true);
    }

    #[test]
    fn display() {
        let toggles: EnumToggles<TestToggles> = EnumToggles::new();
        assert_eq!(format!("{}", toggles).is_empty(), false);
    }

    #[test]
    fn load_from_file() {
        // Create a temporary file
        let mut temp_file =
            tempfile::NamedTempFile::new().expect("Unable to create temporary file");

        // Write some data to the file
        writeln!(temp_file, "1 Toggle1").expect("Unable to write to temporary file");
        writeln!(temp_file, "0 Toggle2").expect("Unable to write to temporary file");
        writeln!(temp_file, "0 VAR1").expect("Unable to write to temporary file");
        writeln!(temp_file, "TESTTEST").expect("Unable to write to temporary file");
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
}
