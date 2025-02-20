# Mimee

Mimee is a Rust library that provides a mapping between file extensions and their corresponding MIME types. It allows you to easily determine the MIME type of a file based on its extension.

## Features

- Predefined set of file extensions and their corresponding MIME types.
- Retrieve the MIME type for a given file path.
- Supports both forward and backward slashes in file paths.
- Handles non-ASCII characters in file paths.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mimee = "0.1.0"
```

## Usage
```rust
use mimee::MimeDict;

fn main() {
    let mime_dict = MimeDict::new();
    let content_type = mime_dict.get_content_type("example.txt".to_string());
    println!("MIME type: {:?}", content_type);
}
```

## License

This project is licensed under the MIT License