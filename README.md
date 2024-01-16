
# Markdown to HTML Converter in Rust 📝➡️🌐

A simple Rust application that converts Markdown text to HTML format.

## Description 📘

This program reads Markdown text from the user, converts it to HTML using the `markdown` crate, and then outputs the HTML. It's a basic example of using external crates and standard I/O in Rust.

## How It Works 🛠️

- The user is prompted to enter Markdown text.
- The program reads the input, converts it to HTML, and displays the HTML output.

## Usage 🚀

1. Run the program.
2. Enter your Markdown text when prompted.
3. The program will display the HTML version of the entered text.

## Code Snippet 📌

```rust
// Importing necessary crates and modules
extern crate markdown;
use std::io;

fn main() {
    // ... [Main Function Implementation]
}

fn text() -> String {
    // ... [Function to Read Markdown Text]
}

fn tohtml(text: String) -> String {
    // ... [Function to Convert Markdown to HTML]
}
```

## Note 🚨

- This application is for demonstration purposes and showcases basic Rust programming with external crates.
- The `markdown` crate is used for conversion, and the standard `io` module is used for input/output operations.

---

Happy Markdown to HTML converting! 🎉
