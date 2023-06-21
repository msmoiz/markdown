use std::fmt::Display;

#[derive(Clone)]
pub enum ListType {
    Unordered(char),
    Ordered(char, usize),
}

#[derive(Clone)]
pub enum ListProximity {
    Tight,
    Loose,
}

#[derive(Clone)]
pub enum HtmlType {
    Literal,
    Comment,
    Processing,
    Declaration,
    Cdata,
    Simple,
    Custom,
}

/// Node.
#[derive(Clone)]
pub enum Node {
    Root(Root),

    BlockQuote(BlockQuote),
    List(List),
    ListItem(ListItem),

    ThematicBreak,
    Heading(Heading),
    Paragraph(Paragraph),
    Code(Code),
    Html(Html),
    Text(String),
}

impl Node {
    pub fn children_mut(&mut self) -> Option<&mut Vec<Node>> {
        match self {
            Node::Root(x) => Some(&mut x.children),
            Node::BlockQuote(x) => Some(&mut x.children),
            Node::List(x) => Some(&mut x.children),
            Node::ListItem(x) => Some(&mut x.children),
            Node::Paragraph(x) => Some(&mut x.children),
            _ => None,
        }
    }
}

lazy_static::lazy_static! {
    static ref TIGHT: bool = false;
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Root(x) => x.children.iter().for_each(|c| write!(f, "{c}").unwrap()),
            Node::BlockQuote(x) => {
                write!(f, "<blockquote>\n").unwrap();
                x.children.iter().for_each(|c| write!(f, "{c}").unwrap());
                write!(f, "</blockquote>\n").unwrap();
            }
            Node::ThematicBreak => {
                write!(f, "<hr />\n").unwrap();
            }
            Node::List(x) => {
                let tag = match x.list_type {
                    ListType::Unordered(_) => "ul",
                    ListType::Ordered(_, _) => "ol",
                };
                let start = match x.list_type {
                    ListType::Ordered(_, 1) => "".to_string(),
                    ListType::Ordered(_, start) => format!(r#" start="{start}""#),
                    _ => "".into(),
                };
                write!(f, "<{tag}{start}>\n").unwrap();
                x.children.iter().for_each(|c| write!(f, "{c}").unwrap());
                write!(f, "</{tag}>\n").unwrap();
            }
            Node::ListItem(x) => {
                write!(f, "<li>").unwrap();
                let mut skip = false;
                x.children.iter().for_each(|c| {
                    let newline = if matches!(c, Node::Text(_)) || skip {
                        "".to_string()
                    } else {
                        "\n".to_string()
                    };
                    let buf = format!("{newline}{c}");
                    if buf.ends_with("\n") {
                        skip = true;
                    } else {
                        skip = false;
                    }
                    write!(f, "{buf}").unwrap();
                });
                write!(f, "</li>\n").unwrap();
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
            Node::Code(x) => {
                let info = match &x.info {
                    Some(info) => {
                        let i = info.trim().split(" ").next().unwrap();
                        format!(r#" class="language-{i}""#)
                    }
                    None => "".to_string(),
                };
                write!(f, "<pre><code{}>{}</code></pre>\n", info, encode(&x.text)).unwrap();
            }
            Node::Html(x) => write!(f, "{}", x.text).unwrap(),
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

const ENCODINGS: [(&str, &str); 2] = [("<", "&lt;"), (">", "&gt;")];

fn encode(input: &str) -> String {
    let mut output = input.to_string();
    for (from, to) in ENCODINGS {
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

/// Block quote.
#[derive(Clone)]
pub struct BlockQuote {
    children: Vec<Node>,
}

impl BlockQuote {
    pub fn new() -> Self {
        Self { children: vec![] }
    }
}

/// List.
#[derive(Clone)]
pub struct List {
    pub list_type: ListType,
    pub proximity: ListProximity,
    pub children: Vec<Node>,
}

impl List {
    pub fn new(list_type: ListType) -> Self {
        Self {
            list_type,
            proximity: ListProximity::Tight,
            children: vec![],
        }
    }
}

/// List item.
#[derive(Clone)]
pub struct ListItem {
    pub indent: usize,
    pub children: Vec<Node>,
}

impl ListItem {
    pub fn new(indent: usize) -> Self {
        Self {
            indent,
            children: vec![],
        }
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

/// Code.
#[derive(Clone, Default)]
pub struct Code {
    pub text: String,
    pub info: Option<String>,
}

impl Code {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

/// HTML.
#[derive(Clone)]
pub struct Html {
    pub text: String,
    pub html_type: HtmlType,
}

impl Html {
    pub fn new(text: String, html_type: HtmlType) -> Self {
        Self { text, html_type }
    }
}
