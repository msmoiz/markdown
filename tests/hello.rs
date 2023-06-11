use markdown::to_html;

#[test]
fn parse_hello() {
    let markdown = "hello";
    let html = to_html(markdown);
    assert_eq!(html, "<p>hello</p>")
}
