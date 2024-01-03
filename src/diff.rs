extern crate serde_json;

use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn diff() {
    let file_path = Path::new("tests/resources/diff/01/en.json");
    let mut file = File::open(&file_path).expect("Unable to open file");
    let mut contents = String::new();

    // Read the file contents into a string, returns `Result` over `std::io::Error`.
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read {}: {}", file_path.display(), why),
        Ok(_) => print!("successfully read {}", file_path.display()),
    };

    let v: Value = serde_json::from_str(&contents).expect("Unable to parse json");
    if let Value::Object(map) = v {
        // for key in map.keys() {
        //     println!("{}", key);
        // }

        let keys_as_strings: Vec<String> = map.keys().map(|k| k.to_string()).collect();
        println!("{:?}", keys_as_strings);
    }
}

// pub fn export(base_dir: &str) -> Vec<String> {
//
//     let mut path = PathBuf::from(base_dir);
//     path.push("en.json");
pub fn export(file_path: &str) -> Vec<String> {

    let mut path = PathBuf::from(file_path);
    let file_path = Path::new(path.as_path());

    let mut file = File::open(&file_path).expect("Unable to open file");
    let mut contents = String::new();

    // Read the file contents into a string, returns `Result` over `std::io::Error`.
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read {}: {}", file_path.display(), why),
        Ok(_) => print!("successfully read {}", file_path.display()),
    };

    let v: Value = serde_json::from_str(&contents).expect("Unable to parse json");
    let mut paths = Vec::new();
    traverse(&v, "".to_string(), &mut paths);
    return paths;
}

fn traverse(val: &Value, path: String, paths: &mut Vec<String>) {
    match val {
        Value::Object(map) => {
            for (key, value) in map {
                let new_path = if path.is_empty() { key.clone() } else { format!("{}.{}", path, key) };
                traverse(value, new_path, paths);
            }
        }
        _ => paths.push(path.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        diff();
    }
}

#[cfg(test)]
mod export {
    use crate::diff::{export};

    #[test]
    fn json_path() {
        let expected: [&str; 2] = ["Parent.Child.GrandChild.01", "Parent.Child.GrandChild.02"];
        let actual = export("tests/resources/export/01/en.json");
        assert_eq!(actual, expected);
    }
}