extern crate serde_json;

use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::path::{Path};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        diff();
    }
}
