use regex::Regex;
use std::collections::VecDeque;
use std::env;
use std::fs;

fn process_queue(mut queue: VecDeque<String>, re: Regex) -> Result<(), ()> {
    let mut found_anything = false;

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
                            found_anything = true
                        }
                    }
                }
            }
        }
    }

    if found_anything {
        Ok(())
    }
    else {
        Err(())
    }
}

fn cmdline_arg(n: usize, default: &str) -> String {
    return env::args().nth(n).unwrap_or_else(|| String::from(default));
}

fn run_app() -> Result<(), ()>  {
    let re = Regex::new(&cmdline_arg(1, ".*")).unwrap();
    let path = cmdline_arg(2, ".");
    let queue: VecDeque<_> = VecDeque::from([path]);
    return process_queue(queue, re);
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(_) => 1,
    });
}
