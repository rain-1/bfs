use std::env;

fn main_path() -> String {
    match env::args().nth(1) {
        Some(argument) => return argument,
        None => return String::from("."),
    }
}

fn main() {
    let path = main_path();
    println!("Path {}", path);
}
