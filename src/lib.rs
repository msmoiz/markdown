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
    static ref INDENT_CODE_RE: Regex = Regex::new(
        r"(?x)
        # start of text
        ^
        # leading spaces
        \ {4}
        # content
        (
            \ *
            [^\ ]{1}
            .*
        )
        # end of text
        $
        "
    )
    .expect("indented code regex should be valid");
    static ref FENCED_CODE_RE: Regex = Regex::new(
        r"(?x)
        # start of text
        ^
        # leading spaces
        (\ {0,3})
        # delimiters
        (`{3,}|~{3,})
        # info string
        (.*)
        $
        "
    )
    .expect("fenced code regex should be valid");
}

enum CodeBlockType {
    Fenced,
    _Indented,
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
    use CodeBlockType::*;

    let mut root = Root(Root::new());

    let mut scope = vec![];
    let mut chunk_separators = vec![];
    let mut code_block_type: Option<CodeBlockType> = None;
    let mut fenced_block_delim: Option<String> = None;
    let mut fenced_block_lead: Option<u8> = None;

    for line in text.lines() {
        // Blank line
        if line.trim().is_empty() {
            match scope.last_mut() {
                Some(Paragraph(_)) => end_previous(&mut root, &mut scope),
                Some(Code(code)) => match code_block_type {
                    Some(Fenced) => code.text.push_str(&format!("{line}\n")),
                    _ => {
                        let content = line.chars().skip(4).collect::<String>();
                        chunk_separators.push(format!("{content}\n"));
                    }
                },
                _ => {}
            }
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
        if let (Some(cap), Some(Paragraph(para))) = (SETEXT_HEADING_RE.captures(line), scope.last())
        {
            let level = if cap.get(1).is_some() { 1 } else { 2 };
            root.children_mut()
                .unwrap()
                .push(Heading(Heading::new(level, para.children.clone())));
            scope.pop();
            continue;
        }

        // Thematic break
        if HR_RE.is_match(line) {
            end_previous(&mut root, &mut scope);
            root.children_mut().unwrap().push(ThematicBreak);
            continue;
        }

        // Fenced code
        match (
            FENCED_CODE_RE.captures(line),
            scope.last_mut(),
            &code_block_type,
            &fenced_block_delim,
            &fenced_block_lead,
        ) {
            (Some(cap), _, None, None, None) => {
                end_previous(&mut root, &mut scope);
                let mut code = ast::Code::new();
                if let Some(info) = cap.get(3) {
                    if info.as_str().len() > 0 {
                        code.info = Some(info.as_str().into());
                    }
                }
                scope.push(Code(code));
                code_block_type = Some(Fenced);
                let delim = cap.get(2).unwrap().as_str();
                fenced_block_delim = Some(delim.into());
                let lead = cap.get(1).unwrap().len() as u8;
                fenced_block_lead = Some(lead);
                continue;
            }
            (None, Some(Code(code)), Some(Fenced), _, Some(lead)) => {
                let mut content = format!("{line}\n");
                for _ in 0..*lead {
                    if content.starts_with(' ') {
                        content.remove(0);
                    }
                }
                code.text.push_str(&content);
                continue;
            }
            (Some(cap), Some(Code(code)), Some(Fenced), Some(op_delim), _) => {
                let cl_delim = cap.get(2).unwrap().as_str();
                let same_type =
                    cl_delim.chars().nth(0).unwrap() == op_delim.as_str().chars().nth(0).unwrap();
                let long_enough = cl_delim.len() >= op_delim.len();
                let has_info = !cap.get(3).unwrap().is_empty();
                if same_type && long_enough && !has_info {
                    end_previous(&mut root, &mut scope);
                    code_block_type = None;
                    fenced_block_delim = None;
                    fenced_block_lead = None;
                } else {
                    code.text.push_str(&format!("{line}\n"));
                }
                continue;
            }
            _ => {}
        }

        // Paragraph
        if let Some(Paragraph(para)) = scope.last_mut() {
            para.children.push(Text(format!("\n{}", line.trim_start())));
            continue;
        }

        // Indented code
        match (INDENT_CODE_RE.captures(line), scope.last_mut()) {
            (Some(cap), Some(Code(code))) => {
                while let Some(sep) = chunk_separators.pop() {
                    code.text.push_str(&sep);
                }
                let content = cap.get(1).unwrap().as_str();
                code.text.push_str(&format!("{content}\n"));
                continue;
            }
            (Some(cap), None) => {
                chunk_separators.clear();
                let content = cap.get(1).unwrap().as_str();
                let mut code = ast::Code::new();
                code.text = format!("{content}\n");
                scope.push(Code(code));
                continue;
            }
            _ => {}
        }

        end_previous(&mut root, &mut scope);
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
