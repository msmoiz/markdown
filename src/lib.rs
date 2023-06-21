//! This library can be used to parse Markdown text into HTML.

mod ast;

use std::cmp::max;

use ast::{
    Heading, Html, HtmlType, ListProximity, ListType,
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
            [\ \t]+
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
        [\ \t]?
        "
    )
    .expect("blockquote regex should be valid");
    static ref LIST_ITEM_RE: Regex = Regex::new(
        r"(?x)
        # start of text
        ^
        # leading space
        \ {0,3}
        # delim
        (
            [\-+*]
            |[0-9]{1,9}[.)]
        )
        # trailing space
        ([\ \t]+|$)
        "
    )
    .expect("list item regex should be valid");
    static ref HTML_1_5_RE: Regex = Regex::new(
        r"(?x)
        ^
        # leading space
        \ {0,3}
        ((?:(?:<pre|<script|<style|<textarea)(?:[\ >]|$))|<!--|<\?|<![[:alpha:]]|<!\[CDATA\[)
        "
    )
    .expect("html 1-5 regex should be valid");
    static ref HTML_6_RE: Regex = Regex::new(
        r"(?xi)
        ^ # start
        # leading space
        \ {0,3}
        </? # delim
        (?: # tag name
            address|article|aside|base|basefont|blockquote|body|caption
            |center|col|colgroup|dd|details|dialog|dir|div|dl|dt|fieldset
            |figcaption|figure|footer|form|frame|frameset|h1|h2|h3|h4|h5
            |h6|head|header|hr|html|iframe|legend|li|link|main|menu|menuitem
            |nav|noframes|ol|optgroup|option|p|param|section|source|summary
            |table|tbody|td|tfoot|th|thead|title|tr|track|ul
        )
        (?: # close
            [\ ]|>|/>|$
        )
        "
    )
    .expect("html 6 regex should be valid");
    // line begins with a complete open tag (with any tag name other than pre,
    // script, style, or textarea) or a complete closing tag, followed by zero or
    // more spaces and tabs, followed by the end of the line.
    static ref HTML_7_RE: Regex = Regex::new(
        r###"(?xi)
        # start
        ^
        # tag
        (?:
            # open
            (?:
                # start delim and tag name
                <[[:alpha:]][[:alnum:]\-]*
                # attributes
                (?:
                    # leading space
                    \ *
                    # tag name
                    [_:[:alpha:]][_.:\-[:alnum:]]*
                    # optional value spec
                    (?:
                        \ *
                        =
                        \ *
                        # value
                        (?:
                            '[^']*'
                            |"[^"]*"
                            |[^\ "'=<>`]+
                        )
                    )?
                )*
                # trailing space
                \ *
                # optional close
                /?
                # end delim
                >
            )
            # close
            |(?:
                # start delim and tag name
                </[[:alpha:]][[:alnum:]\-]*
                # trailing space
                \ *
                # end delim
                >
            )
        )
        # end
        $
        "###
    ).expect("html 7 regex should be valid");
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

    let mut last_line_blank = false;

    for line in text.lines() {
        // Close unmatched containers
        let len = tree.stack.len();
        let (matched, line, mut _remaining_space) =
            matched_containers(&mut tree, line, last_line_blank);
        for _ in matched..tree.stack.len() {
            if let (Code(_), Some(Fenced)) = (tree.cur_mut(), &code_block_type) {
                code_block_type = None;
                fenced_block_delim = None;
                fenced_block_lead = None;
            }
            tree.pop();
        }

        let dropped = len - matched;
        let mut could_be_lazy = true;

        // Check for new containers
        let mut line = line;
        loop {
            // ignore in fenced code block
            if let (Code(_), Some(Fenced)) = (tree.cur_mut(), &code_block_type) {
                break;
            }

            // on a loop, check if we match a block quote
            // if we do, advance line, loop again, otherwise break
            if let Some(cap) = BLOCKQUOTE_RE.captures(line) {
                if let Paragraph(_) = tree.cur_mut() {
                    tree.pop();
                }
                if let (Code(_), None) = (tree.cur_mut(), &code_block_type) {
                    tree.pop();
                }
                tree.push(BlockQuote(ast::BlockQuote::new()));
                let delim = cap.get(0).unwrap().as_str();
                _remaining_space = if delim.ends_with("\t") { 2 } else { 0 };
                line = &line[delim.len()..];
                continue;
            };

            if let Some(cap) = LIST_ITEM_RE.captures(line) {
                // if it could be a thematic break, that interpretation takes
                // precedence
                if HR_RE.is_match(line) {
                    if let List(_) = tree.cur_mut() {
                        tree.pop();
                    }
                    break;
                }

                let delim = cap.get(1).expect("delim should exist").as_str();
                let trail_len = cap.get(2).unwrap().len();

                if let Paragraph(_) = tree.cur_mut() {
                    // empty list cannot interrupt paragraph
                    if trail_len == 0 {
                        break;
                    } else if !delim.ends_with(")") && !delim.ends_with(".") {
                        tree.pop();
                    } else if delim == "1)" || delim == "1." {
                        tree.pop();
                    } else {
                        break;
                    }
                }

                if !matches!(tree.cur_mut(), List(_)) {
                    let list_type = match delim.chars().last().expect("last char should exist") {
                        c @ ')' | c @ '.' => {
                            ast::ListType::Ordered(c, delim[..delim.len() - 1].parse().unwrap())
                        }
                        c @ '-' | c @ '+' | c @ '*' => ast::ListType::Unordered(c),
                        _ => unreachable!(),
                    };
                    tree.push(List(ast::List::new(list_type)));
                }

                let delim_ = cap.get(0).unwrap().as_str();
                let mut indent = delim_.len();
                if cap.get(2).unwrap().len() == 0 {
                    indent = delim_.len();
                } else if scan_indented_code(&line.trim_start()[delim.len() + 1..]) {
                    indent = delim_.trim_end().len() + 1;
                } else if line.trim_end().len() == delim_.trim_end().len() {
                    // blank starting line
                    indent = delim_.trim_end().len() + 1;
                }
                if cap.get(2).unwrap().as_str().ends_with("\t") {
                    _remaining_space = 2;
                } else {
                    _remaining_space = 0;
                }
                tree.push(ListItem(ast::ListItem::new(max(indent, 2))));
                line = &line[indent..];
                could_be_lazy = false;
                continue;
            }

            break;
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
                Html(html) => match html.html_type {
                    HtmlType::Simple => tree.pop(),
                    _ => html.text.push_str(&format!("{line}\n")),
                },
                _ => {}
            }
            if !matches!(tree.cur_mut(), Node::BlockQuote(_) | Node::Code(_)) {
                last_line_blank = true;
            }
            if let ListItem(list_item) = tree.cur_mut() {
                if list_item.children.is_empty() {
                    last_line_blank = false;
                }
            }
            continue;
        }

        last_line_blank = false;

        // HTML
        if let Html(html) = tree.cur_mut() {
            let content = format!("{line}\n");
            html.text.push_str(&content);
            if end_condition_met(html, line) {
                tree.pop();
            }
            continue;
        }

        if let Some(cap) = HTML_1_5_RE.captures(line) {
            let content = format!("{line}\n");
            let delim = cap.get(1).unwrap().as_str();
            let html_type = match delim {
                "<!--" => HtmlType::Comment,
                "<?" => HtmlType::Processing,
                "<![CDATA[" => HtmlType::Cdata,
                _ => {
                    if delim.starts_with("<!") {
                        HtmlType::Declaration
                    } else {
                        HtmlType::Literal
                    }
                }
            };

            if matches!(tree.cur_mut(), Paragraph(_)) {
                tree.pop();
            }

            if matches!((tree.cur_mut(), &code_block_type), (Code(_), None)) {
                tree.pop();
            }

            tree.push(Html(ast::Html::new(content, html_type)));
            if let Html(html) = tree.cur_mut() {
                if end_condition_met(html, line) {
                    tree.pop();
                }
            }
            continue;
        }

        if HTML_6_RE.is_match(line) {
            let content = format!("{line}\n");

            if matches!(tree.cur_mut(), Paragraph(_)) {
                tree.pop();
            }

            if matches!((tree.cur_mut(), &code_block_type), (Code(_), None)) {
                tree.pop();
            }

            tree.push(Html(ast::Html::new(content, HtmlType::Simple)));
            if let Html(html) = tree.cur_mut() {
                if end_condition_met(html, line) {
                    tree.pop();
                }
            }
            continue;
        }

        if HTML_7_RE.is_match(line) {
            // paragraph takes precedence over html 7 block
            if !matches!(tree.cur_mut(), Paragraph(_)) {
                let content = format!("{line}\n");
                tree.push(Html(ast::Html::new(content, HtmlType::Custom)));
                continue;
            }
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
        if dropped > 0 && could_be_lazy {
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
        match (scan_indented_code(line), tree.cur_mut()) {
            (true, Code(code)) => {
                while let Some(sep) = chunk_separators.pop() {
                    code.text.push_str(&sep);
                }
                let mut line = Line::new(line);
                line.scan_space_upto(4);
                code.text.push_str(&format!("{}\n", line.remainder()));
                continue;
            }
            (true, _) => {
                chunk_separators.clear();
                let mut line = Line::new(line);
                line.scan_space_upto(4);
                let content = line.remainder();
                let mut code = ast::Code::new();
                code.text = format!("{}{content}\n", &"    "[.._remaining_space]);
                tree.push(Code(code));
                continue;
            }
            _ => {}
        }

        if let (Code(_), None) = (tree.cur_mut(), &code_block_type) {
            tree.pop();
        }

        if let List(_) = tree.cur_mut() {
            tree.pop();
        }

        let mut para = Paragraph(Paragraph::new());
        para.children_mut()
            .unwrap()
            .push(Text(line.trim_start().into()));
        tree.push(para);
    }

    tighten(&mut tree.root);
    format!("{}", tree.root)
}

fn matched_containers<'a>(
    state: &mut Tree,
    line: &'a str,
    last_line_blank: bool,
) -> (usize, &'a str, usize) {
    let mut i = 0;
    let mut node = &mut state.root;
    let mut line = line;
    let mut loosen = None;
    let mut maybe_new_item = false;
    let mut remaining_spaces = 0;
    for s in &state.stack {
        node = &mut (node.children_mut().expect("should be parent")[*s]);
        match node {
            BlockQuote(_) => match BLOCKQUOTE_RE.captures(line) {
                Some(cap) => {
                    let cap = cap.get(0).unwrap().as_str();
                    line = &line[cap.len()..];
                    if cap.ends_with("\t") {
                        remaining_spaces = 2;
                    } else {
                        remaining_spaces = 0;
                    }
                }
                None => break,
            },
            List(list) => match (&list.list_type, LIST_ITEM_RE.captures(line)) {
                (ListType::Unordered(delim), Some(cap)) => {
                    let new_delim = cap.get(1).unwrap().as_str();
                    let new_delim = new_delim.chars().next().unwrap();
                    if &new_delim != delim {
                        // I am not a match and neither are my children
                        break;
                    } else {
                        // I am a match but my children cannot be because
                        // there is a new item on the list
                        if i + 1 == state.stack.len() && last_line_blank {
                            // sometimes the list has no child, in which case
                            // we mark loose on our own
                            list.proximity = ListProximity::Loose;
                        } else {
                            maybe_new_item = true;
                        }
                    }
                }
                (ListType::Ordered(delim, _), Some(cap)) => {
                    let new_delim = cap.get(1).unwrap().as_str();
                    let new_delim = new_delim.chars().last().unwrap();
                    if &new_delim != delim {
                        // I am not a match and neither are my children
                        break;
                    } else {
                        // I am a match but my children cannot be because
                        // there is a new item on the list
                        if i + 1 == state.stack.len() && last_line_blank {
                            // sometimes the list has no child, in which case
                            // we mark loose on our own
                            list.proximity = ListProximity::Loose;
                        } else {
                            maybe_new_item = true;
                        }
                    }
                }
                (_, None) => {
                    // not sure, defer to list item
                }
            },
            ListItem(list_item) => {
                if line.trim().is_empty() {
                    if list_item.children.is_empty() {
                        // too many blank lines
                        break;
                    }
                    i += 1;
                    continue;
                }

                let indent = list_item.indent;
                let mut chars = line.chars();
                let mut count = 0;
                let mut used = 0;

                while remaining_spaces > 0 && count < indent {
                    count += 1;
                    remaining_spaces -= 1;
                }

                loop {
                    match chars.next() {
                        Some(ch) => {
                            if ch == ' ' {
                                count += 1;
                                used += 1;
                                if count >= indent {
                                    break;
                                }
                            } else if ch == '\t' {
                                count = (count / 4) + 4;
                                used += 1;
                                if count >= indent {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                        None => break,
                    }
                }

                if count < indent {
                    if maybe_new_item {
                        if last_line_blank {
                            loosen = Some(i);
                        }
                        break;
                    }
                    i -= 1;
                    break;
                }

                let mut lin = Line::new(line);
                lin.scan_space_upto(indent);
                remaining_spaces = lin.remaining_spaces;

                maybe_new_item = false;

                line = &line[used..];

                if last_line_blank && !list_item.children.is_empty() {
                    // grab the parent
                    loosen = Some(i);
                }
            }
            _ => {}
        }

        i += 1;
    }

    if let Some(parent) = loosen {
        node = &mut state.root;
        for j in 0..parent {
            let s = state.stack[j];
            node = &mut (node.children_mut().expect("should be parent")[s]);
        }
        if let List(list) = node {
            list.proximity = ListProximity::Loose;
        }
    }

    (i, line, remaining_spaces)
}

fn tighten(node: &mut Node) {
    if let List(ast::List {
        proximity: ListProximity::Tight,
        children,
        ..
    }) = node
    {
        for item in children {
            match item {
                ListItem(list_item) => {
                    let mut ix = 0;
                    loop {
                        if ix == list_item.children.len() {
                            break;
                        }
                        // if the child at this index is a paragraph, steal
                        // its children and discard it
                        if let Paragraph(_) = &list_item.children[ix] {
                            let mut p = list_item.children.remove(ix);
                            while let Some(c) = p.children_mut().unwrap().pop() {
                                list_item.children.insert(ix, c);
                            }
                        }
                        ix += 1;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    if let Some(children) = node.children_mut() {
        for child in children {
            tighten(child);
        }
    }
}

fn end_condition_met(html: &Html, line: &str) -> bool {
    match html.html_type {
        HtmlType::Literal => {
            line.contains("</pre>")
                || line.contains("</script>")
                || line.contains("</style>")
                || line.contains("</textarea>")
        }
        HtmlType::Comment => line.contains("-->"),
        HtmlType::Processing => line.contains("?>"),
        HtmlType::Declaration => line.contains(">"),
        HtmlType::Cdata => line.contains("]]>"),
        HtmlType::Simple | HtmlType::Custom => line.is_empty(),
    }
}

fn scan_indented_code(line: &str) -> bool {
    let mut chars = line.chars();
    let mut spaces = 0;
    while let Some(ch) = chars.next() {
        match ch {
            ' ' => spaces += 1,
            '\t' => spaces += 4 - (spaces % 4),
            _ => break,
        }
    }
    spaces >= 4
}

/// Abstraction over a line of text.
struct Line<'a> {
    /// Input text.
    input: &'a str,
    /// Current position in the input.
    position: usize,
    /// Remaining spaces in the last consumed tab.
    remaining_spaces: usize,
}

impl<'a> Line<'a> {
    /// Creates a new Line object.
    fn new(input: &'a str) -> Self {
        Line {
            input,
            position: 0,
            remaining_spaces: 0,
        }
    }

    /// Scan up to the specified number of spaces.
    fn scan_space_upto(&mut self, n: usize) {
        let mut chars = self.input.chars();
        let mut spaces = 0;
        while let Some(ch) = chars.next() {
            if spaces >= n {
                break;
            }
            match ch {
                ' ' => {
                    self.position += 1;
                    spaces += 1;
                }
                '\t' => {
                    self.position += 1;
                    spaces += 4 - (spaces % 4);
                    if spaces > n {
                        self.remaining_spaces = spaces - n;
                    }
                }
                _ => break,
            }
        }
    }

    /// Returns the remaining text.
    fn remainder(&self) -> &'a str {
        &self.input[self.position..]
    }
}
