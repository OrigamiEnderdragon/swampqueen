use std::{fs::File, io};

use camino::Utf8Path;
use serde::de::DeserializeOwned;

/// Load a `.json` file into the given type.
///
/// # Arguments
///
/// * `path` - Any type implementing [`AsRef<Utf8Path>`], denoting the path to the `.json` file.
///
/// # Errors
///
/// * When there is a problem calling [`File::open`].
///
/// * When there is a problem calling [`serde_json::from_reader`].
pub fn try_load_json<P, D>(path: P) -> io::Result<D>
where
    P: AsRef<Utf8Path>,
    D: DeserializeOwned,
{
    let file = File::open(path.as_ref())?;
    Ok(serde_json::from_reader(file)?)
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Deserialize, Debug)]
    struct TestStruct {
        name: String,
        numbers: Vec<i32>,
    }

    #[test]
    fn load_test_json() {
        let expected_name = "hello!";
        let expected_numbers = vec![1, 2, 3, 4, 5];
        let test_json_path = "testfiles/test.json";

        let my_test_struct: TestStruct = try_load_json(test_json_path).unwrap();
        assert_eq!(expected_name, my_test_struct.name);
        assert_eq!(expected_numbers, my_test_struct.numbers);
    }
}
