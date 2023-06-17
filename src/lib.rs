//! This library can be used to parse Markdown text into HTML.

mod ast;

use ast::{
    Heading,
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
    static ref ATX_HEADING_RE: Regex = Regex::new(
        r"(?x)
        # start of text
        ^
        # leading spaces               
        \ {0,3}
        # delimiters
        (\#{1,6})
        # body
        (?:
            # separating space
            \ +
            # content
            (?:
                ([^\#]*)\s+\#+\s*$  # closing sequence (with text)
                |\#+\s*$            # closing sequence (without text)
                |(.*)$              # no closing sequence
            )
            # empty
            |$
        )
        "
    )
    .expect("heading regex should be valid");
    static ref SETEXT_HEADING_RE: Regex = Regex::new(
        r"(?x)
        # start of text
        ^
        # leading spaces
        \ {0, 3}
        # delimiters
        (?:
            (=+) # level 1
            |(-+) # level 2
        )
        # trailing spaces
        \ *
        # end of text
        $
        "
    )
    .expect("setext heading should be valid");
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

        // ATX heading
        if let Some(cap) = ATX_HEADING_RE.captures(line) {
            end_previous(&mut root, &mut scope);
            let opening = cap.get(1).expect("opening sequence should be captured");
            let content = match (cap.get(2), cap.get(3)) {
                (Some(mat), None) => mat.as_str().to_string(),
                (None, Some(mat)) => mat.as_str().to_string(),
                (None, None) => String::new(),
                _ => unreachable!("cannot match on both"),
            };
            root.children_mut().unwrap().push(Heading(Heading::new(
                opening.len() as u8,
                vec![Text(content.trim().into())],
            )));
            continue;
        }

        // Setext heading
        if let (Some(cap), Some(para)) = (SETEXT_HEADING_RE.captures(line), scope.last()) {
            let level = if cap.get(1).is_some() { 1 } else { 2 };
            root.children_mut().unwrap().push(Heading(Heading::new(
                level,
                para.children().unwrap().clone(),
            )));
            scope.pop();
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
