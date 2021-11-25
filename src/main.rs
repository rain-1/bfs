use std::collections::VecDeque;
use std::env;
use std::fs;

fn main_path() -> String {
    match env::args().nth(1) {
        Some(argument) => return argument,
        None => return String::from("."),
    }
}

fn process_queue(mut queue: VecDeque<String>) -> () {
    while let Some(path) = queue.pop_front() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        let path_str = entry.path().into_os_string().into_string().unwrap();
                        if metadata.is_dir() {
                            queue.push_back(path_str);
                        } else {
                            println!("{}", path_str);
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let path = main_path();
    let queue: VecDeque<_> = VecDeque::from([path]);
    process_queue(queue);
}
