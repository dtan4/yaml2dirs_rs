use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::panic;
use std::path::Path;
use std::process;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct DirTree(HashMap<String, Option<DirTree>>);

fn parse_yaml_file(filename: &str) -> Result<DirTree, Box<dyn Error>> {
    let body = fs::read_to_string(filename)?;

    let dir_tree: DirTree = serde_yaml::from_str(body.as_str())?;

    Ok(dir_tree)
}

fn make_dirs(root_dir: &str, dir_tree: &DirTree) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&root_dir);

    println!("{:?}", path);

    for (d, v) in &dir_tree.0 {
        let dir = path.join(Path::new(&d));

        fs::create_dir(&dir)?;

        match v {
            Some(t) => {
                if let Err(e) = make_dirs(&dir.into_os_string().into_string().unwrap(), t) {
                    return Err(e);
                }
            }
            None => continue,
        };
    }

    Ok(())
}

fn main() {
    let mut args = env::args();
    args.next();

    let filename = match args.next() {
        Some(v) => v,
        None => {
            eprintln!("YAML file is required");
            process::exit(1);
        }
    };

    println!("{}", filename);

    let dir_tree = match parse_yaml_file(&filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("cannot parse the given file: {}", e);
            process::exit(1);
        }
    };

    println!("{:?}", dir_tree);

    let root_dir = match env::current_dir() {
        Ok(v) => v.into_os_string().into_string().unwrap(),
        Err(e) => {
            eprintln!("cannot get working directory: {}", e);
            process::exit(1);
        }
    };

    println!("{}", root_dir);

    if let Err(e) = make_dirs(&root_dir, &dir_tree) {
        eprintln!("cannot create directories: {}", e);
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_dirs() {
        let root_dir = std::env::temp_dir()
            .join("make_dirs")
            .into_os_string()
            .into_string()
            .unwrap();

        fs::create_dir_all(&root_dir).expect("failed to create tempdir");

        let mut dir_tree_japan_tokyo_map: HashMap<String, Option<DirTree>> = HashMap::new();
        dir_tree_japan_tokyo_map.insert(String::from("shibuya"), None);
        dir_tree_japan_tokyo_map.insert(String::from("shinjuku"), None);
        let dir_tree_japan_tokyo = DirTree(dir_tree_japan_tokyo_map);

        let mut dir_tree_japan_map: HashMap<String, Option<DirTree>> = HashMap::new();
        dir_tree_japan_map.insert(String::from("nagoya"), None);
        dir_tree_japan_map.insert(String::from("osaka"), None);
        dir_tree_japan_map.insert(String::from("tokyo"), Some(dir_tree_japan_tokyo));
        let dir_tree_japan = DirTree(dir_tree_japan_map);

        let mut dir_tree_malaysia_map: HashMap<String, Option<DirTree>> = HashMap::new();
        dir_tree_malaysia_map.insert(String::from("kuala_lumpur"), None);
        let dir_tree_malaysia = DirTree(dir_tree_malaysia_map);

        let mut dir_tree_map: HashMap<String, Option<DirTree>> = HashMap::new();
        dir_tree_map.insert(String::from("japan"), Some(dir_tree_japan));
        dir_tree_map.insert(String::from("malaysia"), Some(dir_tree_malaysia));
        dir_tree_map.insert(String::from("singapore"), None);
        let dir_tree = DirTree(dir_tree_map);

        let want_dirs = vec![
            "japan",
            "japan/nagoya",
            "japan/osaka",
            "japan/tokyo",
            "japan/tokyo/shibuya",
            "japan/tokyo/shinjuku",
            "malaysia",
            "malaysia/kuala_lumpur",
            "singapore",
        ];

        let result = panic::catch_unwind(|| {
            make_dirs(&root_dir, &dir_tree).expect("want no error");

            let root_dir_path = Path::new(&root_dir);

            for wd in want_dirs {
                let p = root_dir_path.join(&wd);
                assert!(
                    p.exists(),
                    "{} doesn't exist",
                    p.into_os_string().into_string().unwrap()
                );
                assert!(
                    p.is_dir(),
                    "{} is not a directory",
                    p.into_os_string().into_string().unwrap()
                );
            }
        });

        fs::remove_dir_all(&root_dir).expect("failed to clean up tempdir");

        assert!(result.is_ok());
    }
}
