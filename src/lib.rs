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
    static ref BLOCKQUOTE_RE: Regex = Regex::new(
        r"(?x)
        # start of text
        ^
        # leading spaces
        \ {0,3}
        # delim
        >
        # trailing spaces
        \ {0,1}
        "
    )
    .expect("blockquote regex should be valid");
}

enum CodeBlockType {
    Fenced,
    _Indented,
}

/// Parse tree.
///
/// Used to track parsing state.
struct Tree {
    /// The root node of the tree.
    root: Node,
    /// The current position in the tree.
    stack: Vec<usize>,
}

impl Tree {
    /// Create a new tree.
    pub fn new() -> Self {
        Self {
            root: Node::Root(Root::new()),
            stack: vec![],
        }
    }

    /// Get the current focused node.
    fn cur_mut(&mut self) -> &mut Node {
        let mut node = &mut self.root;
        for i in &self.stack {
            node = &mut node.children_mut().expect("should be parent")[*i];
        }
        node
    }

    /// Get the parent of the current focused node.
    fn parent_mut(&mut self) -> &mut Node {
        let mut node = &mut self.root;
        for i in &self.stack[..self.stack.len() - 1] {
            node = &mut node.children_mut().expect("should be parent")[*i];
        }
        node
    }

    /// Push a node onto the tree and focus it.
    fn push(&mut self, node: Node) {
        let children = self
            .cur_mut()
            .children_mut()
            .expect("push target should support children");
        children.push(node);
        let index = children.len() - 1;
        self.stack.push(index);
    }

    /// Pop the current node from focus.
    fn pop(&mut self) {
        self.stack.pop().expect("pop target is valid");
    }

    /// Focus the last child of the current node.
    fn advance(&mut self) {
        let children = self
            .cur_mut()
            .children_mut()
            .expect("push target should support children");
        let index = children.len() - 1;
        self.stack.push(index);
    }

    /// Remove the current node from the tree.
    fn remove(&mut self) {
        self.parent_mut()
            .children_mut()
            .expect("should be parent")
            .pop()
            .expect("child should exist");
        self.stack.pop().expect("child should exist");
    }
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

    let mut tree = Tree::new();

    let mut chunk_separators = vec![];
    let mut code_block_type: Option<CodeBlockType> = None;
    let mut fenced_block_delim: Option<String> = None;
    let mut fenced_block_lead: Option<u8> = None;

    for line in text.lines() {
        // Close unmatched containers
        let len = tree.stack.len();
        let (matched, line) = matched_containers(&mut tree, line);
        for _ in matched..tree.stack.len() {
            if let (Code(_), Some(Fenced)) = (tree.cur_mut(), &code_block_type) {
                code_block_type = None;
                fenced_block_delim = None;
                fenced_block_lead = None;
            }
            tree.pop();
        }

        let dropped = len - matched;

        // Check for new containers
        let mut line = line;
        loop {
            // on a loop, check if we match a block quote
            // if we do, advance line, loop again, otherwise break
            if let Some(cap) = BLOCKQUOTE_RE.captures(line) {
                // ignore in fenced code block
                if let (Code(_), Some(Fenced)) = (tree.cur_mut(), &code_block_type) {
                    break;
                } else {
                    if let Paragraph(_) = tree.cur_mut() {
                        tree.pop();
                    }
                    tree.push(BlockQuote(ast::BlockQuote::new()));
                    let delim = cap.get(0).unwrap().len();
                    line = &line[delim..]
                }
            } else {
                break;
            };
        }

        // Blank line
        if line.trim().is_empty() {
            match tree.cur_mut() {
                Paragraph(_) => tree.pop(),
                Code(code) => match code_block_type {
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
            if let Paragraph(_) = tree.cur_mut() {
                tree.pop();
            }
            let opening = cap.get(1).expect("opening sequence should be captured");
            let content = match (cap.get(2), cap.get(3)) {
                (Some(mat), None) => mat.as_str().to_string(),
                (None, Some(mat)) => mat.as_str().to_string(),
                (None, None) => String::new(),
                _ => unreachable!("cannot match on both"),
            };
            tree.push(Heading(Heading::new(
                opening.len() as u8,
                vec![Text(content.trim().into())],
            )));
            tree.pop();
            continue;
        }

        // Setext heading
        if let (Some(cap), Paragraph(para)) = (SETEXT_HEADING_RE.captures(line), tree.cur_mut()) {
            let children = para.children.clone();
            let level = if cap.get(1).is_some() { 1 } else { 2 };
            tree.remove();
            tree.push(Heading(Heading::new(level, children)));
            tree.pop();
            continue;
        }

        // Thematic break
        if HR_RE.is_match(line) {
            if let Paragraph(_) = tree.cur_mut() {
                tree.pop();
            }
            if let (Code(_), None) = (tree.cur_mut(), &code_block_type) {
                tree.pop();
            }
            tree.push(ThematicBreak);
            tree.pop();
            continue;
        }

        // Fenced code
        match (
            FENCED_CODE_RE.captures(line),
            tree.cur_mut(),
            &code_block_type,
            &fenced_block_delim,
            &fenced_block_lead,
        ) {
            (Some(cap), _, None, None, None) => {
                if let Paragraph(_) = tree.cur_mut() {
                    tree.pop();
                }
                let mut code = ast::Code::new();
                if let Some(info) = cap.get(3) {
                    if info.as_str().len() > 0 {
                        code.info = Some(info.as_str().into());
                    }
                }
                tree.push(Code(code));
                code_block_type = Some(Fenced);
                let delim = cap.get(2).unwrap().as_str();
                fenced_block_delim = Some(delim.into());
                let lead = cap.get(1).unwrap().len() as u8;
                fenced_block_lead = Some(lead);
                continue;
            }
            (None, Code(code), Some(Fenced), _, Some(lead)) => {
                let mut content = format!("{line}\n");
                for _ in 0..*lead {
                    if content.starts_with(' ') {
                        content.remove(0);
                    }
                }
                code.text.push_str(&content);
                continue;
            }
            (Some(cap), Code(code), Some(Fenced), Some(op_delim), _) => {
                let cl_delim = cap.get(2).unwrap().as_str();
                let same_type =
                    cl_delim.chars().nth(0).unwrap() == op_delim.as_str().chars().nth(0).unwrap();
                let long_enough = cl_delim.len() >= op_delim.len();
                let has_info = !cap.get(3).unwrap().is_empty();
                if same_type && long_enough && !has_info {
                    tree.pop();
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
        if let Paragraph(para) = tree.cur_mut() {
            para.children.push(Text(format!("\n{}", line.trim_start())));
            continue;
        }

        // nothing else matched, which means this is a paragraph
        // if we dropped on this iteration, add the scopes back
        // and treat it as a continuation line
        if dropped > 0 && BLOCKQUOTE_RE.is_match(&format!("> {line}")) {
            for _ in 0..dropped {
                tree.advance();
            }
            if let Paragraph(para) = tree.cur_mut() {
                para.children.push(Text(format!("\n{}", line.trim_start())));
                continue;
            } else {
                // unless the top of the stack was not a paragraph
                // in which case, we revert these changes
                for _ in 0..dropped {
                    tree.pop();
                }
            }
        }

        // Indented code
        match (INDENT_CODE_RE.captures(line), tree.cur_mut()) {
            (Some(cap), Code(code)) => {
                while let Some(sep) = chunk_separators.pop() {
                    code.text.push_str(&sep);
                }
                let content = cap.get(1).unwrap().as_str();
                code.text.push_str(&format!("{content}\n"));
                continue;
            }
            (Some(cap), _) => {
                chunk_separators.clear();
                let content = cap.get(1).unwrap().as_str();
                let mut code = ast::Code::new();
                code.text = format!("{content}\n");
                tree.push(Code(code));
                continue;
            }
            _ => {}
        }

        if let (Code(_), None) = (tree.cur_mut(), &code_block_type) {
            tree.pop();
        }

        let mut para = Paragraph(Paragraph::new());
        para.children_mut()
            .unwrap()
            .push(Text(line.trim_start().into()));
        tree.push(para);
    }

    format!("{}", tree.root)
}

fn matched_containers<'a>(state: &mut Tree, line: &'a str) -> (usize, &'a str) {
    let mut i = 0;
    let mut node = &mut state.root;
    let mut line = line;
    for s in &state.stack {
        node = &mut (node.children_mut().expect("should be parent")[*s]);
        match node {
            BlockQuote(_) => match BLOCKQUOTE_RE.captures(line) {
                Some(cap) => {
                    let len = cap.get(0).unwrap().len();
                    line = &line[len..];
                }
                None => break,
            },
            _ => {}
        }
        i += 1;
    }
    (i, line)
}
