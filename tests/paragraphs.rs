use markdown::to_html;

// 219
#[test]
fn simple() {
    let markdown = "aaa\n\nbbb\n";
    let html = to_html(markdown);
    assert_eq!(html, "<p>aaa</p>\n<p>bbb</p>\n");
}

// 220
#[test]
fn multiline() {
    let markdown = "aaa\nbbb\n\nccc\nddd\n";
    let html = to_html(markdown);
    assert_eq!(html, "<p>aaa\nbbb</p>\n<p>ccc\nddd</p>\n");
}

// 221
#[test]
fn multiple_blanks() {
    let markdown = "aaa\n\n\nbbb\n";
    let html = to_html(markdown);
    assert_eq!(html, "<p>aaa</p>\n<p>bbb</p>\n");
}

// 222
#[test]
fn skip_leading_spaces() {
    let markdown = "  aaa\n bbb\n";
    let html = to_html(markdown);
    assert_eq!(html, "<p>aaa\nbbb</p>\n");
}

// 223
#[test]
fn skip_many_leading_spaces() {
    let markdown = r"
aaa
            bbb
                                    ccc
";
    let html = to_html(markdown);
    assert_eq!(html, "<p>aaa\nbbb\nccc</p>\n");
}

// 224
#[test]
fn start_three_leading_spaces() {
    let markdown = r"
   aaa
bbb
";
    let html = to_html(markdown);
    assert_eq!(html, "<p>aaa\nbbb</p>\n");
}

// 225
#[test]
#[ignore = "code block not supported"]
fn start_four_leading_spaces() {
    let markdown = r"
     aaa
 bbb
 ";
    let html = to_html(markdown);
    assert_eq!(html, "<p>aaa\nbbb</p>\n");
}

// 226
#[test]
#[ignore = "hard line break not supported"]
fn trailing_spaces() {
    let markdown = r"
aaa     
bbb     
";
    let html = to_html(markdown);
    assert_eq!(html, "<p>aaa<br />\nbbb</p>\n")
}
