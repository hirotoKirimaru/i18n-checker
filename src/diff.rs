extern crate serde_json;

use std::collections::HashSet;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::collections::HashMap;

pub fn diff(i18n_file_base_dir: &str)  -> Vec<I18nFile> {
    let dir_path = Path::new(i18n_file_base_dir);
    match group_files_in_dir_recursive(dir_path) {
        Ok(map) => {
            for (dir, files) in map {
                // ディレクトリに1つしかファイル無ければスキップ。
                if files.len() <= 1 {continue;}

                println!("Directory: {}", dir.display());
                for file_path in files {
                    println!(" - File: {}", file_path.display());
                }
            }
        }
        Err(err) => eprintln!("Error occurred: {}", err),
    }
    return vec![];
}

fn group_files_in_dir_recursive(dir_path: &Path) -> io::Result<HashMap<PathBuf, Vec<PathBuf>>> {
    let mut map: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
    for entry_result in fs::read_dir(dir_path)? {
        if let Ok(entry) = entry_result {
            let path = entry.path();
            if path.is_file() {
                let dir = path.parent().unwrap().to_path_buf();
                let file_list = map.entry(dir).or_insert_with(Vec::new);
                file_list.push(path);
            } else if path.is_dir() {
                let sub_map = group_files_in_dir_recursive(&path)?;
                map.extend(sub_map);
            }
        }
    }
    Ok(map)
}

#[derive(Eq, PartialEq, Debug)]
pub struct I18nFile {
    name: String,
    path: Option<String>,
    keys: HashSet<String>,
}

pub struct I18nFileBuilder {
    name: Option<String>,
    path: Option<String>,
    keys: Option<HashSet<String>>,
}

impl I18nFileBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            path: None,
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
            path: self.path,
            keys: self.keys?,
        })
    }
}

// TODO: argsはしばらくは実装しない。暇があったら実装する。
fn diff_key(one: &I18nFile, two: &I18nFile, _args: Vec<&I18nFile>) -> Vec<I18nFile> {
    if is_same(one, two) {
        return vec![];
    }

    // TODO: Aに比べてBにはこのキーがある、ないを表現できるといい？
    // 一旦は、全部のキーをまとめたうえで、差分を見る
    // たぶん、多すぎる、よりは少なすぎる、という方が必要なのが多いはず。
    // 差分のセット
    let mut rtn: Vec<I18nFile> = vec![];
    let mut all_keys: HashSet<String> = one.keys.clone();
    all_keys.extend(two.keys.clone());

    let one_difference_key: Vec<_> = all_keys.difference(&one.keys).cloned().collect();
    let one_difference = I18nFileBuilder::new()
        .name(one.name.clone())
        .keys(one_difference_key.iter().map(|x| x.to_string()).collect())
        .build();
    rtn.push(one_difference.unwrap());

    let two_difference_key: Vec<_> = all_keys.difference(&two.keys).cloned().collect();
    let two_difference = I18nFileBuilder::new()
        .name(two.name.clone())
        .keys(two_difference_key.iter().map(|x| x.to_string()).collect())
        .build();
    rtn.push(two_difference.unwrap());
    return rtn;
}

fn is_same(one: &I18nFile, two: &I18nFile) -> bool {
    one.keys == two.keys
}

const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";

fn format(args: Vec<I18nFile>) {
    if args.is_empty() {
        return;
    }

    println!("{}It has Difference!{}", RED, RESET);
    for arg in args {
        println!("{}======================{}", YELLOW, RESET);
        println!("paths: {:?}", arg.path);
        println!("name: {}", arg.name);
        println!("keys: {:?}", arg.keys);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_file_in_dir_is_return_empty() {
        let expected: Vec<I18nFile> =  vec![];
        assert_eq!(expected ,diff("tests/resources/diff/01"));
    }
}

#[cfg(test)]
mod diff_key {
    use super::*;

    #[test]
    fn same_key_is_empty() {
        // GIVEN
        let one = I18nFileBuilder::new()
            .name("ja.json".to_string())
            .keys(to_hash_set(&[
                "Parent.Child.GrandChild.01",
                "Parent.Child.GrandChild.02",
                "GrandParent",
            ]))
            .build();

        let two = I18nFileBuilder::new()
            .name("en.json".to_string())
            .keys(to_hash_set(&[
                "Parent.Child.GrandChild.01",
                "Parent.Child.GrandChild.02",
                "GrandParent",
            ]))
            .build();

        // WHEN
        let actual = diff_key(&one.unwrap(), &two.unwrap(), vec![]);

        // THEN
        let expected: Vec<I18nFile> = vec![];
        assert_eq!(expected, actual);
    }

    /// WHEN
    /// ・ja.jsonに "Parent.Child.GrandChild.02", "GrandParent"
    /// ・en.jsonに "Parent.Child.GrandChild.01", "Parent.Child.GrandChild.02"
    /// THEN
    /// ・ja.json に "Parent.Child.GrandChild.01"
    /// ・en.json に "GrandParent"
    /// が不足していることを伝える
    ///
    #[test]
    fn has_difference() {
        // GIVEN
        let one: I18nFile = I18nFileBuilder::new()
            .name("ja.json".to_string())
            .keys(to_hash_set(&["Parent.Child.GrandChild.02", "GrandParent"]))
            .build()
            .unwrap();

        let two: I18nFile = I18nFileBuilder::new()
            .name("en.json".to_string())
            .keys(to_hash_set(&[
                "Parent.Child.GrandChild.01",
                "Parent.Child.GrandChild.02",
            ]))
            .build()
            .unwrap();

        // WHEN
        let actual = diff_key(&one, &two, vec![]);

        // THEN
        assert_eq!(actual.len(), 2);

        assert_eq!(
            actual
                .iter()
                .find(|x| { x.name == one.name.clone() })
                .unwrap(),
            &I18nFileBuilder::new()
                .name(one.name)
                .keys(to_hash_set(&["Parent.Child.GrandChild.01"]))
                .build()
                .unwrap()
        );

        assert_eq!(
            actual
                .iter()
                .find(|x| { x.name == two.name.clone() })
                .unwrap(),
            &I18nFileBuilder::new()
                .name(two.name)
                .keys(to_hash_set(&["GrandParent"]))
                .build()
                .unwrap()
        );
    }
    fn to_hash_set(param: &[&'static str]) -> HashSet<String> {
        return param.iter().map(|s| s.to_string()).collect();
    }
}

#[cfg(test)]
mod format {
    use super::*;

    // NOTE: テストをサボっただけなので、後で作り直す。
    #[test]
    fn has_difference_format() {
        // GIVEN
        let one: I18nFile = I18nFileBuilder::new()
            .name("ja.json".to_string())
            .keys(to_hash_set(&["Parent.Child.GrandChild.02", "GrandParent"]))
            .build()
            .unwrap();

        let two: I18nFile = I18nFileBuilder::new()
            .name("en.json".to_string())
            .keys(to_hash_set(&[
                "Parent.Child.GrandChild.01",
                "Parent.Child.GrandChild.02",
            ]))
            .build()
            .unwrap();

        // WHEN
        let actual = diff_key(&one, &two, vec![]);

        // THEN
        format(actual);
    }

    fn to_hash_set(param: &[&'static str]) -> HashSet<String> {
        return param.iter().map(|s| s.to_string()).collect();
    }
}
