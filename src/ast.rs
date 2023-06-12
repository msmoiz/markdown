use std::fmt::Display;

/// Node.
pub enum Node {
    Root(Root),
    ThematicBreak,
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
            Node::Paragraph(x) => {
                write!(f, "<p>").unwrap();
                x.children.iter().for_each(|c| write!(f, "{c}").unwrap());
                write!(f, "</p>\n").unwrap();
            }
            Node::Text(x) => {
                write!(f, "{x}").unwrap();
            }
        };
        Ok(())
    }
}

/// Root.
pub struct Root {
    pub children: Vec<Node>,
}

impl Root {
    pub fn new() -> Self {
        Self { children: vec![] }
    }
}

/// Paragraph.
pub struct Paragraph {
    pub children: Vec<Node>,
}

impl Paragraph {
    pub fn new() -> Self {
        Self { children: vec![] }
    }
}
