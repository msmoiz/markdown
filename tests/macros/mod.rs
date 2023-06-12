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
            let separator = "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~";
            let components: Vec<&str> = indoc!($test).split(separator).collect();
            let markdown = &components[1][1..]; // skip leading newline
            let html = to_html(markdown);
            let expected = &components[2][1..]; // skip leading newline
            assert_eq!(html, expected);
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
            let separator = "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~";
            let components: Vec<&str> = indoc!($test).split(separator).collect();
            let markdown = &components[1][1..]; // skip leading newline
            let html = to_html(markdown);
            let expected = &components[2][1..]; // skip leading newline
            assert_eq!(html, expected);
        }
    };
}
