use markdown::to_html;

// 43
#[test]
fn simple() {
    let markdown = r"
***
---
___
";
    let html = to_html(markdown);
    assert_eq!(html, "<hr />\n".repeat(3));
}

// 44
#[test]
fn wrong_chars() {
    let markdown = "+++";
    let html = to_html(markdown);
    assert_eq!(html, "<p>+++</p>\n");
}

// 45
#[test]
fn wrong_chars_2() {
    let markdown = "===";
    let html = to_html(markdown);
    assert_eq!(html, "<p>===</p>\n");
}

// 46
#[test]
fn not_enough_chars() {
    let markdown = r"
--
**
__
";
    let html = to_html(markdown);
    assert_eq!(html, "<p>--\n**\n__</p>\n");
}

// 47
#[test]
fn three_leading_spaces() {
    let markdown = r"
 ***
  ***
   ***
";
    let html = to_html(markdown);
    assert_eq!(html, "<hr />\n".repeat(3));
}

// 48
#[test]
#[ignore = "code block not supported"]
fn four_leading_spaces() {
    let markdown = r"
    ***
";
    let html = to_html(markdown);
    assert_eq!(html, "<pre><code>***\n</code></pre>");
}

// 49
#[test]
fn four_leading_spaces_2() {
    let markdown = r"
Foo
    ***
";
    let html = to_html(markdown);
    assert_eq!(html, "<p>Foo\n***</p>\n");
}

// 50
#[test]
fn many_chars() {
    let markdown = "_____________________________________";
    let html = to_html(markdown);
    assert_eq!(html, "<hr />\n");
}

// 51
#[test]
fn interleaved_spaces() {
    let markdown = " - - -";
    let html = to_html(markdown);
    assert_eq!(html, "<hr />\n");
}

// 52
#[test]
fn interleaved_spaces_2() {
    let markdown = " **  * ** * ** * **";
    let html = to_html(markdown);
    assert_eq!(html, "<hr />\n");
}

// 53
#[test]
fn interleaved_spaces_3() {
    let markdown = "-     -      -      -";
    let html = to_html(markdown);
    assert_eq!(html, "<hr />\n");
}

// 54
#[test]
fn trailing_spaces() {
    let markdown = "- - - -    ";
    let html = to_html(markdown);
    assert_eq!(html, "<hr />\n");
}

// 55
#[test]
fn interleaved_alnum() {
    let markdown = r"
_ _ _ _ a

a------

---a---
";
    let html = to_html(markdown);
    assert_eq!(html, "<p>_ _ _ _ a</p>\n<p>a------</p>\n<p>---a---</p>\n");
}

// 56
#[test]
fn mixed_delimiters() {
    let markdown = " *-*";
    let html = to_html(markdown);
    assert_eq!(html, "<p>*-*</p>\n");
}

// 57
#[test]
#[ignore = "list not supported"]
fn no_blank_lines() {
    let markdown = r"
- foo
***
- bar
";
    let html = to_html(markdown);
    assert_eq!(
        html,
        r"<ul>\n<li>foo</li>\n</ul>\n<hr />\n<ul>\n<li>bar</li>\n</ul>\n"
    )
}

// 58
#[test]
fn interrupt_paragraph() {
    let markdown = "
Foo
***
bar
";
    let html = to_html(markdown);
    assert_eq!(html, "<p>Foo</p>\n<hr />\n<p>bar</p>\n");
}

// 59
#[test]
#[ignore = "header not supported"]
fn setext_header_precedence() {
    let markdown = r"
Foo
---
bar
";
    let html = to_html(markdown);
    assert_eq!(html, "<h2>Foo</h2>\n<p>bar</p>\n");
}

// 60
#[test]
#[ignore = "list not supported"]
fn list_precedence() {
    let markdown = r"
* Foo
* * *
* Bar
";
    let html = to_html(markdown);
    assert_eq!(
        html,
        "<ul>\n<li>Foo</li>\n</ul>\n<hr />\n<ul>\n<li>Bar</li>\n</ul>\n"
    );
}

// 61
#[test]
#[ignore = "list not supported"]
fn in_list() {
    let markdown = r"
- Foo
- * * *
";
    let html = to_html(markdown);
    assert_eq!(html, "<ul>\n<li>Foo</li>\n<li>\n<hr />\n</li>\n</ul>\n");
}
