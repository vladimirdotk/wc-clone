use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    os::unix::fs::MetadataExt,
    path::Path,
};

use clap::{Arg, ArgAction, ArgMatches, Command};

enum CountMode {
    Bytes,
    Lines,
    Words,
    All,
}

fn main() {
    let args = args();
    let count_mode = count_mode(&args);
    let file_path = Path::new(args.get_one::<String>("file").unwrap());
    match count_mode {
        CountMode::Bytes => count_bytes(file_path),
        CountMode::Lines => count_lines(file_path),
        CountMode::Words => count_words(file_path),
        _ => (),
    }
}

fn args() -> ArgMatches {
    Command::new("wc-clome")
        .version("0.1.0")
        .author("Vladimir K")
        .about("Count bytes, lines, words")
        .arg(
            Arg::new("bytes")
                .short('c')
                .action(ArgAction::SetTrue)
                .help("Count bytes"),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .action(ArgAction::SetTrue)
                .help("Count lines"),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .action(ArgAction::SetTrue)
                .help("Count words"),
        )
        .arg(
            Arg::new("file")
                .help("File to process")
                .required(true)
                .index(1),
        )
        .get_matches()
}

fn count_mode(args: &ArgMatches) -> CountMode {
    if args.get_flag("bytes") {
        CountMode::Bytes
    } else if args.get_flag("lines") {
        CountMode::Lines
    } else if args.get_flag("words") {
        CountMode::Words
    } else {
        CountMode::All
    }
}

fn count_bytes(file_path: &Path) {
    match File::open(file_path) {
        Ok(file) => {
            println!(
                "  {} {}",
                file.metadata().unwrap().size(),
                file_path.to_str().unwrap(),
            );
        }
        Err(e) => {
            println!("Failed to open file: {:?}", e);
        }
    }
}

fn count_lines(file_path: &Path) {
    match File::open(file_path) {
        Ok(file) => {
            println!(
                "  {} {}",
                BufReader::new(file).lines().count(),
                file_path.to_str().unwrap(),
            );
        }
        Err(e) => {
            println!("Failed to open file: {:?}", e);
        }
    }
}

fn count_words(file_path: &Path) {
    match File::open(file_path) {
        Ok(file) => {
            let mut count = 0;
            let mut in_word = false;

            for byte in BufReader::new(file).bytes() {
                match byte.unwrap() {
                    b' ' | b'\n' | b'\r' | b'\t' => {
                        if in_word {
                            count += 1;
                            in_word = false;
                        }
                    }
                    _ => {
                        in_word = true;
                    }
                }
            }

            println!("  {} {}", count, file_path.to_str().unwrap(),);
        }
        Err(e) => {
            println!("Failed to open file: {:?}", e);
        }
    }
}
