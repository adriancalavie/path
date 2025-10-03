use std::fs::write;
use std::path;

use clap::Parser;

#[derive(Parser)]
#[command(name = "path")]
#[command(about = "A CLI tool for getting the full path of a file")]
#[command(version = "1.0")]
struct Args {
    /// Input file path
    input: String,

    /// Output file path
    #[arg(short, long)]
    output: Option<String>,

    /// Output as URI
    #[arg(short, long, default_value_t = false)]
    uri: bool,
}

fn get_full_path(file: &str) -> std::io::Result<std::path::PathBuf> {
    let path = std::fs::canonicalize(file)?;
    Ok(path)
}

fn as_uri(path: &std::path::Path) -> String {
    let mut uri = String::from("file://");
    if cfg!(windows) {
        // On Windows, we need to add an extra slash and replace backslashes with forward slashes
        uri.push('/');
        uri.push_str(&path.to_string_lossy().replace("\\", "/"));
    } else {
        uri.push_str(&path.to_string_lossy());
    }
    uri
}

fn main() {
    let args = Args::parse();

    if let Some(output) = &args.output {
        println!("Output file: {}", output);
    }

    let format_path = |full_path: path::PathBuf| {
        if args.uri {
            as_uri(&full_path)
        } else {
            full_path.to_string_lossy().to_string()
        }
    };

    match get_full_path(&args.input) {
        Ok(full_path) => match args.output {
            Some(ref output) => {
                if let Err(e) = write(output, format_path(full_path).as_bytes()) {
                    eprintln!("Unable to write to {}: {}", output, e);
                    std::process::exit(1);
                }
                println!("Full path written to {}", output);
            }
            None => {
                println!("{}", format_path(full_path));
            }
        },
        Err(e) => {
            eprintln!("Error getting full path: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_get_full_path_existing_file() {
        // Test with current file
        let result = get_full_path("src/main.rs");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.is_absolute());
        assert!(path.to_string_lossy().contains("main.rs"));
    }

    #[test]
    fn test_get_full_path_nonexistent_file() {
        let result = get_full_path("nonexistent_file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_as_uri_unix() {
        let path = Path::new("/home/user/file.txt");
        let uri = as_uri(path);
        assert_eq!(uri, "file:///home/user/file.txt");
    }

    #[test]
    fn test_as_uri_current_dir() {
        let current_dir = std::env::current_dir().unwrap();
        let uri = as_uri(&current_dir);
        assert!(uri.starts_with("file://"));
        let dir_name = current_dir.file_name().unwrap().to_string_lossy();
        assert!(uri.contains(&*dir_name));
    }

    #[test]
    fn test_as_uri_relative_path() {
        // Test with a file we know exists
        let path_result = get_full_path("src/main.rs");
        assert!(path_result.is_ok());
        let path = path_result.unwrap();
        let uri = as_uri(&path);
        assert!(uri.starts_with("file://"));
        assert!(uri.contains("main.rs"));
    }
}
