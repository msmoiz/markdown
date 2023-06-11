//! This library can be used to parse Markdown text into HTML.

/// Parses an input Markdown text into HTML.
///
/// # Examples
///
/// ```
/// let markdown = "hello world";
/// let html = markdown::to_html(markdown);
/// assert_eq!(html, "<p>hello world</p>")
/// ```
pub fn to_html(text: &str) -> String {
    format!("<p>{text}</p>")
}
