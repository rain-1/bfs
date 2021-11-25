use std::env;
use std::{fs, io};
use std::path::{PathBuf};

fn main_path() -> String {
    match env::args().nth(1) {
        Some(argument) => return argument,
        None => return String::from("."),
    }
}

fn path_list_directory(path:String) -> io::Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort();
    return Ok(entries);
}

fn main() {
    let path = main_path();
    println!("Path {}", path);
    println!("Things: {:?}", path_list_directory(path).unwrap());
}
