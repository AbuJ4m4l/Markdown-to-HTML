// This program converts Markdown text to HTML format. It uses the `markdown` external crate
// and standard `io` library for input and output operations.

// Importing the `markdown` crate to enable Markdown to HTML conversion
extern crate markdown;
// Importing the `io` module from the standard library for input/output operations
use std::io;

/// Main function: Entry point of the program.
/// 
/// It reads Markdown text from the user, converts it to HTML, and then prints the HTML.
fn main() {
    // Calling `text` function to read Markdown text from the user
    let md_text = text();
    // Converting the Markdown text to HTML using `tohtml` function
    let html = tohtml(md_text);
    // Printing the HTML version of the text
    println!("{}", html);
}

/// Reads Markdown text from standard input.
/// 
/// # Returns
/// 
/// A `String` containing the trimmed Markdown text.
fn text() -> String {
    println!("Enter Markdown text: ");
    let mut t = String::new(); // Creating a new, empty String to store the input
    // Reading a line from stdin and storing it in the String `t`
    io::stdin().read_line(&mut t).expect("Failed to read line");
    t.trim().to_string() // Trimming white spaces and new lines from the text
}

/// Converts Markdown text to HTML format.
/// 
/// # Arguments
/// 
/// * `text` - A `String` containing Markdown text.
/// 
/// # Returns
/// 
/// A `String` containing the HTML representation of the Markdown text.
fn tohtml(text: String) -> String {
    println!("Converting Markdown to HTML...");

    // Converting the input text to HTML and printing a confirmation message
    println!("Converted to HTML: ");
    markdown::to_html(&text) // Using `to_html` function from `markdown` crate
}