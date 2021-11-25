use std::collections::VecDeque;
use std::env;
use std::fs;
use regex::Regex;

fn process_queue(mut queue: VecDeque<String>, re: Regex) -> () {
    while let Some(path) = queue.pop_front() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        let path = entry.path();
                        let filename_str = path.file_name().unwrap().to_str().unwrap();
                        let path_str = entry.path().into_os_string().into_string().unwrap();
                        if metadata.is_dir() {
                            queue.push_back(path_str);
                        } else if re.is_match(&filename_str) {
                            println!("{}", path_str);
                        }
                    }
                }
            }
        }
    }
}

fn cmdline_arg(n:usize, default:String) -> String {
    return env::args().nth(n).unwrap_or_else(|| default)
}

fn main() {
    let re = Regex::new(&cmdline_arg(1, String::from(".*"))).unwrap();
    let path = cmdline_arg(2, String::from("."));
    let queue: VecDeque<_> = VecDeque::from([path]);
    process_queue(queue, re);
}
