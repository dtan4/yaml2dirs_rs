use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct DirTree(HashMap<String, Option<DirTree>>);

fn parse_yaml_file(filename: String) -> Result<DirTree, Box<dyn Error>> {
    let body = fs::read_to_string(filename)?;

    let dir_tree: DirTree = serde_yaml::from_str(body.as_str())?;

    Ok(dir_tree)
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

    let dir_tree = match parse_yaml_file(filename) {
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
}
