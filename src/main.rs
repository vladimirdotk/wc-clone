use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
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
    let args = command().get_matches();
    let result = run(args);
    println!("{}", result);
}

fn run(args: ArgMatches) -> String {
    let count_mode = count_mode(&args);

    let file_arg = args
        .get_one::<String>("file")
        .expect("Error getting file argument");
    let file_path = Path::new(file_arg);
    let file = File::open(file_path).expect("Error open file");

    count(
        count_mode,
        file_path
            .to_str()
            .expect("Error converting file path to utf-8 string"),
        file,
    )
}

fn command() -> Command {
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

fn count(count_mode: CountMode, file_path: &str, file: File) -> String {
    match count_mode {
        CountMode::Bytes => {
            let bytes = file.metadata().expect("Error counting bytes").size();
            format!("  {} {}", bytes, file_path)
        }
        CountMode::Lines => {
            let lines = BufReader::new(file).lines().count();
            format!("  {} {}", lines, file_path)
        }
        CountMode::Words => {
            let (_, words) = count_lines_and_words(&file).expect("Error counting words");
            format!("  {} {}", words, file_path)
        }
        CountMode::All => {
            let (lines, words) =
                count_lines_and_words(&file).expect("Error counting words and lines");
            let bytes = file.metadata().expect("Error counting bytes").size();
            format!("  {} {} {} {}", lines, words, bytes, file_path)
        }
    }
}

fn count_lines_and_words(file: &File) -> Result<(usize, i32), io::Error> {
    let mut lines = 0;
    let mut words = 0;

    let mut in_word = false;

    for byte in BufReader::new(file).bytes() {
        match byte? {
            b @ (b' ' | b'\n' | b'\r' | b'\t') => {
                if in_word {
                    words += 1;
                    in_word = false;
                }
                if b == b'\n' {
                    lines += 1;
                }
            }
            _ => {
                in_word = true;
            }
        }
    }

    Ok((lines, words))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_bytes() {
        let args = vec!["wc-clone", "-c", "src/test.txt"];
        let args = command().get_matches_from(args);
        let result = run(args);
        assert_eq!("  342190 src/test.txt", result);
    }

    #[test]
    fn count_lines() {
        let args = vec!["wc-clone", "-l", "src/test.txt"];
        let args = command().get_matches_from(args);
        let result = run(args);
        assert_eq!("  7145 src/test.txt", result);
    }

    #[test]
    fn count_words() {
        let args = vec!["wc-clone", "-w", "src/test.txt"];
        let args = command().get_matches_from(args);
        let result = run(args);
        assert_eq!("  58164 src/test.txt", result);
    }

    #[test]
    fn count_all() {
        let args = vec!["wc-clone", "src/test.txt"];
        let args = command().get_matches_from(args);
        let result = run(args);
        assert_eq!("  7145 58164 342190 src/test.txt", result);
    }
}
