use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read, Stdin},
    path::Path,
};

use clap::{Arg, ArgAction, ArgMatches, Command};

#[cfg(target_family = "unix")]
use libc::{self, STDIN_FILENO};

enum CountMode {
    Bytes,
    Lines,
    Words,
    All,
}

enum BufferReader {
    FileReader(BufReader<File>),
    StdinReader(BufReader<Stdin>),
}

fn main() {
    let args = command().get_matches();
    let result = run(args);
    println!("{}", result);
}

fn run(args: ArgMatches) -> String {
    let count_mode = count_mode(&args);
    match is_piped() {
        true => run_pipe(count_mode),
        false => run_file(args, count_mode),
    }
}

fn run_pipe(count_mode: CountMode) -> String {
    let buf_reader = BufferReader::StdinReader(BufReader::new(io::stdin()));

    count(count_mode, buf_reader, "")
}

fn run_file(args: ArgMatches, count_mode: CountMode) -> String {
    let file_arg = args
        .get_one::<String>("file")
        .expect("Error getting file argument");
    let file_path = Path::new(file_arg);
    let file = File::open(file_path).expect("Error open file");

    let buf_reader = BufferReader::FileReader(BufReader::new(file));

    count(
        count_mode,
        buf_reader,
        file_path
            .to_str()
            .expect("Error converting file path to utf-8 string"),
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
                .required(false)
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

fn count(count_mode: CountMode, buf_reader: BufferReader, file_name: &str) -> String {
    match count_mode {
        CountMode::Bytes => {
            let bytes = count_bytes(buf_reader).expect("Failed to count bytes");
            format!("  {} {}", bytes, file_name)
        }
        _ => {
            let boxed_buff_reader: Box<dyn BufRead> = match buf_reader {
                BufferReader::FileReader(reader) => Box::new(reader),
                BufferReader::StdinReader(reader) => Box::new(reader),
            };

            match count_mode {
                CountMode::Lines => {
                    let (_, lines, _) =
                        count_all(boxed_buff_reader).expect("Failed to count lines");
                    format!("  {} {}", lines, file_name)
                }
                CountMode::Words => {
                    let (_, _, words) =
                        count_all(boxed_buff_reader).expect("Failed to count words");
                    format!("  {} {}", words, file_name)
                }
                CountMode::All => {
                    let (bytes, lines, words) =
                        count_all(boxed_buff_reader).expect("Failed to count bytes");
                    format!("  {} {} {} {}", lines, words, bytes, file_name)
                }
                _ => unreachable!(),
            }
        }
    }
}

fn count_bytes(buf_reader: BufferReader) -> Result<i64, io::Error> {
    match buf_reader {
        BufferReader::FileReader(reader) => Ok(reader.into_inner().metadata()?.len() as i64),
        BufferReader::StdinReader(reader) => {
            let (bytes, _, _) = count_all(Box::new(reader))?;

            Ok(bytes)
        }
    }
}

fn count_all(buf_reader: Box<dyn BufRead>) -> Result<(i64, usize, i32), io::Error> {
    let mut bytes = 0;
    let mut lines = 0;
    let mut words = 0;

    let mut in_word = false;

    for byte in buf_reader.bytes() {
        bytes += 1;
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

    Ok((bytes, lines, words))
}

fn is_piped() -> bool {
    #[cfg(target_family = "unix")]
    unsafe {
        if libc::isatty(STDIN_FILENO) == 0 {
            return true;
        }
    }

    false
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
