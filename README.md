# Path CLI Tool

Get the full path of files or folders, with optional URI format output.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Get the full path of a file
path -i myfile.txt

# Get the full path as a URI
path -i myfile.txt --uri

# Write the result to an output file
path -i myfile.txt -o result.txt
```

## Examples

```bash
$ path -i src/main.rs
/Users/<username>/Developer/rust/path/src/main.rs

$ path -i src/main.rs --uri
file:///Users/<username>/Developer/rust/path/src/main.rs
```

## License

MIT
