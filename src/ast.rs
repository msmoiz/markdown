use std::fmt::Display;

/// Node.
#[derive(Clone)]
pub enum Node {
    Root(Root),
    ThematicBreak,
    Heading(Heading),
    Paragraph(Paragraph),
    Text(String),
}

impl Node {
    pub fn children(&self) -> Option<&Vec<Node>> {
        match self {
            Node::Root(x) => Some(&x.children),
            Node::Paragraph(x) => Some(&x.children),
            _ => None,
        }
    }

    pub fn children_mut(&mut self) -> Option<&mut Vec<Node>> {
        match self {
            Node::Root(x) => Some(&mut x.children),
            Node::Paragraph(x) => Some(&mut x.children),
            _ => None,
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Root(x) => x.children.iter().for_each(|c| write!(f, "{c}").unwrap()),
            Node::ThematicBreak => {
                write!(f, "<hr />\n").unwrap();
            }
            Node::Heading(x) => {
                let level = x.level;
                write!(f, "<h{level}>").unwrap();
                x.children.iter().for_each(|c| write!(f, "{c}").unwrap());
                write!(f, "</h{level}>\n").unwrap();
            }
            Node::Paragraph(x) => {
                write!(f, "<p>").unwrap();
                x.children.iter().for_each(|c| write!(f, "{c}").unwrap());
                write!(f, "</p>\n").unwrap();
            }
            Node::Text(x) => {
                write!(f, "{}", escape(x.trim_end())).unwrap();
            }
        };
        Ok(())
    }
}
const ESCAPES: [(&str, &str); 3] = [(r"\#", "#"), (r"\>", "&gt;"), (r"\-", "-")];

fn escape(input: &str) -> String {
    let mut output = input.to_string();
    for (from, to) in ESCAPES {
        output = output.replace(from, to);
    }
    output
}

/// Root.
#[derive(Clone)]
pub struct Root {
    pub children: Vec<Node>,
}

impl Root {
    pub fn new() -> Self {
        Self { children: vec![] }
    }
}

/// Heading.
#[derive(Clone)]
pub struct Heading {
    pub level: u8,
    pub children: Vec<Node>,
}

impl Heading {
    pub fn new(level: u8, children: Vec<Node>) -> Self {
        Self { level, children }
    }
}

/// Paragraph.
#[derive(Clone)]
pub struct Paragraph {
    pub children: Vec<Node>,
}

impl Paragraph {
    pub fn new() -> Self {
        Self { children: vec![] }
    }
}
