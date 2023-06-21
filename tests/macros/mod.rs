/// Generates a test that compares the parsed Markdown input against the
/// expected HTML output. Use the following separator to frame the input and
/// expected output: `~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~`.
///
/// # Examples
///
/// ```
/// mdtest!(
///     simple,
///     "
///     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///     aaa
///
///     bbb
///     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///     <p>aaa</p>
///     <p>bbb</p>
///     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///     "
/// );
/// ```
#[macro_export]
macro_rules! mdtest {
    ($name:ident, $test:expr) => {
        #[test]
        fn $name() {
            use indoc::indoc;
            use markdown::to_html;

            let separator = "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~";
            let components: Vec<&str> = indoc!($test).split(separator).collect();
            let markdown = &components[1][1..]; // skip leading newline
            let html = to_html(markdown);
            let expected = &components[2][1..]; // skip leading newline
            if html != expected {
                panic!(
                    "\nFailed to parse markdown.\n{separator}Markdown\n{markdown}{separator}Expected\n{expected}{separator}Actual\n{html}"
                );
            }
        }
    };
}

/// Generates a test that compares the parsed Markdown input against the
/// expected HTML output. Use the following separator to frame the input and
/// expected output: `~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~`. Also ignores
/// the test.
///
/// # Examples
///
/// ```
/// mdtest_ignore!(
///     simple,
///     "paragraph not supported"
///     "
///     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///     aaa
///
///     bbb
///     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///     <p>aaa</p>
///     <p>bbb</p>
///     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///     "
/// );
/// ```
#[macro_export]
macro_rules! mdtest_ignore {
    ($name:ident, $reason:expr, $test:expr) => {
        #[test]
        #[ignore = $reason]
        fn $name() {
            use indoc::indoc;
            use markdown::to_html;

            let separator = "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~";
            let components: Vec<&str> = indoc!($test).split(separator).collect();
            let markdown = &components[1][1..]; // skip leading newline
            let html = to_html(markdown);
            let expected = &components[2][1..]; // skip leading newline
            assert_eq!(html, expected);
        }
    };
}
