#[cfg(test)]
mod tests {

    use assert_cmd::Command;

    #[test]
    fn test_list() {
        let output = Command::cargo_bin("shellforge")
            .unwrap()
            .arg("list-formats")
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);

        insta::assert_snapshot!(stdout);
    }

    #[test]
    fn test_generate_default() {
        let output = Command::cargo_bin("shellforge")
            .unwrap()
            .args(["generate", "-i", "192.168.1.1"])
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);

        insta::assert_snapshot!(stdout);
    }

    #[test]
    fn test_generate_format() {
        let output = Command::cargo_bin("shellforge")
            .unwrap()
            .args(["generate", "-i", "192.168.1.1", "-f", "nc-c"])
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);

        insta::assert_snapshot!(stdout);
    }

    #[test]
    fn test_generate_format_base64_encoded() {
        let output = Command::cargo_bin("shellforge")
            .unwrap()
            .args(["generate", "-i", "192.168.1.1", "-f", "nc-c", "-b"])
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);

        insta::assert_snapshot!(stdout);
    }

    #[test]
    fn test_generate_format_url_encoded() {
        let output = Command::cargo_bin("shellforge")
            .unwrap()
            .args(["generate", "-i", "192.168.1.1", "-f", "nc-c", "-u"])
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);

        insta::assert_snapshot!(stdout);
    }
}