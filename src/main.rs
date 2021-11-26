use clap::{App, Arg};
use regex::Regex;
use std::collections::VecDeque;
use std::fs;

fn process_queue(mut queue: VecDeque<String>, re: Regex, print0: bool) -> Result<(), ()> {
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
                            if !print0 {
                                print!("{}\0", path_str);
                            }
                            else {
                                println!("{}", path_str);
                            }
                            found_anything = true
                        }
                    }
                }
            }
        }
    }

    if found_anything {
        Ok(())
    } else {
        Err(())
    }
}

fn run_app() -> Result<(), ()> {
    let matches = App::new("bfs")
        .version("0.1.0")
        .author("Hackerman Jones <hckrmnjones@hack.gov>")
        .about("Breadth First Search the filesystem")
        .arg(
            Arg::with_name("regex")
                .help("The regex to match filenames against")
                .default_value(".*")
                .index(1),
        )
        .arg(
            Arg::with_name("path")
                .help("A path to start search from")
                .default_value(".")
                .index(2),
        )
        .arg(
            Arg::with_name("print0")
                .short("0")
                .long("print0")
                .takes_value(false)
                .help("Use NUL delimiters instead of newlines."),
        )
        .get_matches();


    let re = Regex::new(matches.value_of("regex").unwrap()).unwrap();
    let path = String::from(matches.value_of("path").unwrap());
    let print0 = matches.is_present("print0");

    let queue: VecDeque<_> = VecDeque::from([path]);
    return process_queue(queue, re, print0);
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(_) => 1,
    });
}
