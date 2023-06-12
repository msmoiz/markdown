//! This library can be used to parse Markdown text into HTML.

mod ast;

use ast::{
    Node::{self, *},
    Paragraph, Root,
};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref HR_RE: Regex = Regex::new(
        r"(?x)
        # start of text
        ^
        # leading spaces               
        \ {0,3}
        # delimiters 
        (
            (\*\s*){3,}
            |(-\s*){3,}
            |(_\s*){3,}
        )
        # end of text
        $
    ",
    )
    .expect("hr regex should be valid");
}

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
        // Blank line
        if line.trim().is_empty() {
            end_previous(&mut root, &mut scope);
            continue;
        }

        // Thematic break
        if HR_RE.is_match(line) {
            end_previous(&mut root, &mut scope);
            root.children_mut().unwrap().push(ThematicBreak);
            continue;
        }

        // Paragraph
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

fn end_previous(root: &mut Node, scope: &mut Vec<Node>) {
    if let Some(node) = scope.pop() {
        root.children_mut().unwrap().push(node);
    }
}
