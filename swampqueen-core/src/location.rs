use std::{collections::HashMap, io};

use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

use crate::files::try_load_json;

// TODO this path is only for testing purposes; eventually it will need to be determined
// dynamically
const LOCATION_DIR: &str = "testfiles/locations/";

/// A given location within the game.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    id: String,
    name: String,
    text: HashMap<String, Vec<String>>,
}
impl Location {
    /// Load a [`Location`] from the filesystem.
    ///
    /// # Arguments
    ///
    /// * `location_id` - A [`std::str`] corresponding to the ID of the desired location. The ID
    ///   must match the name of the location `.json` file on the disk.
    ///
    /// # Errors
    ///
    /// This function returns errors typical to loading and deserializing a struct from the disk
    /// (file path does not exist, deserialization failure, filesystem error, etc.).
    pub fn try_load_location(location_id: &str) -> io::Result<Self> {
        let path = Self::get_location_path(location_id);
        try_load_json(path)
    }

    /// Helper function to construct the [`Location`] path from the given ID.
    fn get_location_path(location_id: &str) -> Utf8PathBuf {
        let mut path = Utf8PathBuf::new();
        path.push(LOCATION_DIR);
        path.push(location_id);
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTPLACE_ID: &str = "testplace";
    const TESTPLACE_NAME: &str = "Test Place";
    const TESTPLACE_PATH: &str = "testfiles/locations/testplace.json";
    const TESTPLACE_P0: &str = "You awake to find yourself in a rusty laboratory. Dilapidated equipment surrounds you; your pounding headache is amplified by the lightly swaying fluorescent lights dangling by their frayed cables. Suddenly, a tinny loudspeaker splits the silence...";
    const TESTPLACE_P1: &str =
        "\"This is a test,\" the loudspeaker barked. \"and you have just passed.\"";
    const TESTPLACE_P2: &str = "What would you like to do now?";

    #[test]
    fn load_testplace() {
        let file_str = std::fs::read_to_string(TESTPLACE_PATH).unwrap();
        let location: Location = serde_json::from_str(&file_str).unwrap();
        let intro_text = location.text.get("intro").unwrap();
        assert_eq!(location.id, TESTPLACE_ID);
        assert_eq!(location.name, TESTPLACE_NAME);
        assert_eq!(intro_text[0], TESTPLACE_P0);
        assert_eq!(intro_text[1], TESTPLACE_P1);
        assert_eq!(intro_text[2], TESTPLACE_P2);
    }
}
