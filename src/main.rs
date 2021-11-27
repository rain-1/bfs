use clap::{App, Arg};
use regex::Regex;
use std::collections::VecDeque;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use os_str_bytes::OsStrBytes;

fn process_queue(mut queue: VecDeque<PathBuf>, re: Regex, print0: bool) -> Result<(), ()> {
    let mut found_anything = false;

    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();

    while let Some(path) = queue.pop_front() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    match entry.metadata() {
                        Err(e) => eprintln!("{}", e),
                        Ok(metadata) => {
                            let path = entry.path();
                            let filename_str = path.file_name().unwrap().to_string_lossy();
                            if metadata.is_dir() {
                                queue.push_back(path);
                            } else if re.is_match(&filename_str) {
                                if print0 {
                                    stdout_handle.write_all(&path.into_os_string().to_raw_bytes());
                                    stdout_handle.write_all(&[0]);
                                } else {
                                    println!("{}", path.to_string_lossy());
                                }
                                found_anything = true
                            }
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

    let queue: VecDeque<_> = VecDeque::from([PathBuf::from(path)]);
    return process_queue(queue, re, print0);
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(_) => 1,
    });
}
