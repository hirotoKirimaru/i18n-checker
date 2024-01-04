extern crate serde_json;

use std::collections::HashSet;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::path::Path;

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


pub struct I18nFile {
    name: String,
    keys: HashSet<String>,
}

pub struct I18nFileBuilder {
    name: Option<String>,
    keys: Option<HashSet<String>>,
}

impl I18nFileBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            keys: None,
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn keys(mut self, keys: HashSet<String>) -> Self {
        self.keys = Some(keys);
        self
    }

    pub fn build(self) -> Option<I18nFile> {
        Some(I18nFile {
            name: self.name?,
            keys: self.keys?,
        })
    }
}

fn diff_key(one: I18nFile, two: I18nFile, args: Vec<I18nFile>) -> HashSet<String> {
    for arg in args {
        println!("{}", arg.name);
    }

    return HashSet::new();
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
mod diff_key {
    use super::*;

    #[test]
    fn normal() {
        let expected = HashSet::new();

        let one = I18nFileBuilder::new()
            .name("ja.json".to_string())
            .keys(to_hash_set([
                "Parent.Child.GrandChild.01",
                "Parent.Child.GrandChild.02",
                "GrandParent",
            ]))
            .build();

        let two = I18nFileBuilder::new()
            .name("ja.json".to_string())
            .keys(to_hash_set([
                "Parent.Child.GrandChild.01",
                "Parent.Child.GrandChild.02",
                "GrandParent",
            ])).build();

        let actual = diff_key(one.unwrap(), two.unwrap(), vec![]);
        assert_eq!(actual, expected);
    }

    fn to_hash_set(param: [&str; 3]) -> HashSet<String> {
        return param
            .iter()
            .map(|s| s.to_string())
            .collect();
    }
}