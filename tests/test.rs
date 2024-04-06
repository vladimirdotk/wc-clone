#[cfg(test)]
mod e2e {
    use std::process::{Command, Stdio};

    #[test]
    fn count_bytes() {
        let output = Command::new("./target/debug/wc-clone")
            .arg("-c")
            .arg("./tests/test.txt")
            .stdin(Stdio::inherit())
            .output()
            .expect("Failed to execute command");

        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            "  342190 ./tests/test.txt\n",
        );
    }

    #[test]
    #[cfg(target_family = "unix")]
    fn count_lines() {
        let output = Command::new("./target/debug/wc-clone")
            .arg("-l")
            .arg("./tests/test.txt")
            .stdin(Stdio::inherit())
            .output()
            .expect("Failed to execute command");

        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            "  7145 ./tests/test.txt\n",
        );
    }

    #[test]
    fn count_words() {
        let output = Command::new("./target/debug/wc-clone")
            .arg("-w")
            .arg("./tests/test.txt")
            .stdin(Stdio::inherit())
            .output()
            .expect("Failed to execute command");

        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            "  58164 ./tests/test.txt\n",
        );
    }

    #[test]
    fn count_all() {
        let output = Command::new("./target/debug/wc-clone")
            .arg("./tests/test.txt")
            .stdin(Stdio::inherit())
            .output()
            .expect("Failed to execute command");

        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            "  7145 58164 342190 ./tests/test.txt\n",
        );
    }

    #[test]
    fn pipe() {
        let output = Command::new("sh")
            .arg("-c")
            .arg("cat ./tests/test.txt | ./target/debug/wc-clone")
            .stdin(Stdio::inherit())
            .output()
            .expect("Failed to execute command");

        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            "  7145 58164 342190 \n",
        );
    }
}
