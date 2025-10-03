# Path CLI Tool

Get the full path of files or folders, with optional URI format output.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Get the full path of a file
path myfile.txt

# Get the full path as a URI
path myfile.txt --uri

# Write the result to an output file
path myfile.txt -o result.txt
```

## Examples

```bash
$ path src/main.rs
/Users/<username>/Developer/rust/path/src/main.rs

$ path src/main.rs --uri
file:///Users/<username>/Developer/rust/path/src/main.rs
```

## License

MIT
