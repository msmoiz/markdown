//! This library can be used to parse Markdown text into HTML.

mod ast;

use ast::{Node::*, Paragraph, Root};

/// Parses an input Markdown text into HTML.
///
/// # Examples
///
/// ```
/// let markdown = "hello world";
/// let html = markdown::to_html(markdown);
/// assert_eq!(html, "<p>hello world</p>\n")
/// ```
pub fn to_html(text: &str) -> String {
    let mut root = Root(Root::new());

    let mut scope = vec![];

    for line in text.lines() {
        if line.trim().is_empty() {
            if let Some(node) = scope.pop() {
                root.children_mut().unwrap().push(node);
            }
            continue;
        }

        if let Some(Paragraph(para)) = scope.last_mut() {
            para.children.push(Text(format!("\n{}", line.trim_start())));
            continue;
        }

        let mut para = Paragraph(Paragraph::new());
        para.children_mut()
            .unwrap()
            .push(Text(line.trim_start().into()));
        scope.push(para);
    }

    while let Some(node) = scope.pop() {
        root.children_mut().unwrap().push(node);
    }

    format!("{root}")
}
