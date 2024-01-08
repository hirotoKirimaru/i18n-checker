extern crate serde_json;

use serde_json::Value;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

// pub fn export(file_path: &str) -> HashSet<String> {
pub fn export<P: AsRef<Path>>(file_path: P) -> HashSet<String> {
    let path = PathBuf::from(file_path.as_ref());
    let file_path = Path::new(path.as_path());

    let mut file = File::open(&file_path).expect("Unable to open file");
    let mut contents = String::new();

    // Read the file contents into a string, returns `Result` over `std::io::Error`.
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read {}: {}", file_path.display(), why),
        Ok(_) => print!("successfully read {}", file_path.display()),
    };

    let v: Value = serde_json::from_str(&contents).expect("Unable to parse json");
    let mut paths = HashSet::new();
    traverse(&v, "".to_string(), &mut paths);
    return paths;
}

fn traverse(val: &Value, path: String, paths: &mut HashSet<String>) {
    match val {
        Value::Object(map) => {
            for (key, value) in map {
                let new_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", path, key)
                };
                traverse(value, new_path, paths);
            }
        }
        _ => {
            paths.insert(path.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_json_key() {
        let expected: HashSet<String> = [
            "Parent.Child.GrandChild.01",
            "Parent.Child.GrandChild.02",
            "GrandParent",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let actual = export("tests/resources/export/01/en.json");
        assert_eq!(actual, expected);
    }
}
