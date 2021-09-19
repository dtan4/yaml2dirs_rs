use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn parse_yaml_file(filename: String) -> Result<String, Box<dyn Error>> {
    let body = fs::read_to_string(filename)?;

    Ok(body)
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

    let body = match parse_yaml_file(filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("cannot parse the given file: {}", e);
            process::exit(1);
        }
    };

    let root_dir = match env::current_dir() {
        Ok(v) => v.into_os_string().into_string().unwrap(),
        Err(e) => {
            eprintln!("cannot get working directory: {}", e);
            process::exit(1);
        }
    };

    println!("{}", body);
    println!("{}", root_dir);
}
