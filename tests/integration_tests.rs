mod integration_tests {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_cli_with_existing_file() {
        let mut cmd = Command::cargo_bin("path").unwrap();

        // Test with the main.rs file that definitely exists
        cmd.arg("src/main.rs")
            .assert()
            .success()
            .stdout(predicate::str::contains("main.rs"));
    }

    #[test]
    fn test_cli_with_uri_flag() {
        let mut cmd = Command::cargo_bin("path").unwrap();

        cmd.arg("src/main.rs")
            .arg("--uri")
            .assert()
            .success()
            .stdout(predicate::str::starts_with("file://"));
    }

    #[test]
    fn test_cli_with_nonexistent_file() {
        let mut cmd = Command::cargo_bin("path").unwrap();

        cmd.arg("nonexistent_file.txt")
            .assert()
            .failure()
            .stderr(predicate::str::contains("Error getting full path"));
    }

    #[test]
    fn test_cli_with_output_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let output_path = temp_file.path().to_str().unwrap();

        let mut cmd = Command::cargo_bin("path").unwrap();

        cmd.arg("src/main.rs")
            .arg("--output")
            .arg(output_path)
            .assert()
            .success()
            .stdout(predicate::str::contains("Full path written to"));

        // Verify the file was written
        let content = fs::read_to_string(output_path).unwrap();
        assert!(content.contains("main.rs"));
    }

    #[test]
    fn test_cli_with_output_file_and_uri() {
        let temp_file = NamedTempFile::new().unwrap();
        let output_path = temp_file.path().to_str().unwrap();

        let mut cmd = Command::cargo_bin("path").unwrap();

        cmd.arg("src/main.rs")
            .arg("--output")
            .arg(output_path)
            .arg("--uri")
            .assert()
            .success();

        // Verify the file contains URI format
        let content = fs::read_to_string(output_path).unwrap();
        assert!(content.starts_with("file://"));
    }

    #[test]
    fn test_cli_help() {
        let mut cmd = Command::cargo_bin("path").unwrap();

        cmd.arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "A CLI tool for getting the full path of a file",
            ));
    }

    #[test]
    fn test_cli_version() {
        let mut cmd = Command::cargo_bin("path").unwrap();

        cmd.arg("--version")
            .assert()
            .success()
            .stdout(predicate::str::contains("1.0"));
    }

    #[test]
    fn test_cli_missing_input() {
        let mut cmd = Command::cargo_bin("path").unwrap();

        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("required"));
    }

    #[test]
    fn test_cli_with_clipboard() {
        let mut cmd = Command::cargo_bin("path").unwrap();

        // Test that the command succeeds
        let assert = cmd.arg("src/main.rs")
            .arg("--clipboard")
            .assert()
            .success()
            .stdout(predicate::str::contains("main.rs")); // Always outputs the path

        // Check for either success message in stdout or warning in stderr
        let output = assert.get_output();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        assert!(
            stdout.contains("Full path copied to clipboard") || stderr.contains("Warning: Failed to"),
            "Expected clipboard success or warning message"
        );
    }

    #[test]
    fn test_cli_with_clipboard_and_uri() {
        let mut cmd = Command::cargo_bin("path").unwrap();

        // Test that the command succeeds and outputs URI format
        cmd.arg("src/main.rs")
            .arg("--clipboard")
            .arg("--uri")
            .assert()
            .success()
            .stdout(predicate::str::starts_with("file://"));
    }

    // Manual clipboard verification test (run separately)
    #[test]
    #[ignore] // Ignore by default, run with --ignored flag
    fn test_clipboard_manual_verification() {
        let mut cmd = Command::cargo_bin("path").unwrap();

        cmd.arg("src/main.rs")
            .arg("--clipboard")
            .assert()
            .success();

        // Verify clipboard content using system command
        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = std::process::Command::new("pbpaste").output() {
                let clipboard_content = String::from_utf8(output.stdout).unwrap();
                assert!(clipboard_content.contains("main.rs"));
            }
        }

        #[cfg(target_os = "linux")]
        {
            // Try different clipboard tools on Linux
            if let Ok(output) = std::process::Command::new("xclip")
                .args(["-selection", "clipboard", "-o"])
                .output()
            {
                let clipboard_content = String::from_utf8(output.stdout).unwrap();
                assert!(clipboard_content.contains("main.rs"));
            }
        }
    }
}
