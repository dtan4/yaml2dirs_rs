use std::env;
use std::process;

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

    let root_dir = match env::current_dir() {
        Ok(v) => v.into_os_string().into_string().unwrap(),
        Err(e) => {
            eprintln!("cannot get working directory: {}", e);
            process::exit(1);
        }
    };

    println!("{}", filename);
    println!("{}", root_dir);
}
