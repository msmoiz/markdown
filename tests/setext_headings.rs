mod macros;

// 80
mdtest_ignore!(
    simple,
    "emphasis not supported",
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Foo *bar*
    =========
    
    Foo *bar*
    ---------
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <h1>Foo <em>bar</em></h1>
    <h2>Foo <em>bar</em></h2>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 81
mdtest_ignore!(
    multiline,
    "emphasis not supported",
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Foo *bar
    baz*
    ====
    ---------
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <h1>Foo <em>bar
    baz</em></h1>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 82
mdtest_ignore!(
    content,
    "emphasis not supported",
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Foo *bar
    baz*→
    ====
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <h1>Foo <em>bar
    baz</em></h1>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 83
mdtest!(
    any_length,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Foo
    -------------------------
    
    Foo
    =
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <h2>Foo</h2>
    <h1>Foo</h1>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 84
mdtest!(
    three_leading_spaces,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Foo
    ---
    
      Foo
    -----
    
      Foo
      ===
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <h2>Foo</h2>
    <h2>Foo</h2>
    <h1>Foo</h1>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 85
mdtest!(
    four_leading_spaces,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        Foo
        ---

        Foo
    ---
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <pre><code>Foo
    ---
    
    Foo
    </code></pre>
    <hr />
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 86
mdtest!(
    leading_trailing_spaces,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Foo
    ----      
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <h2>Foo</h2>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 87
mdtest!(
    four_leading_spaces_2,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Foo
        ---
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <p>Foo
    ---</p>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 88
mdtest!(
    internal_spaces,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Foo
    = =
    
    Foo
    --- -
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <p>Foo
    = =</p>
    <p>Foo</p>
    <hr />
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 89
mdtest!(
    trailing_spaces_no_break,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Foo  
    -----
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <h2>Foo</h2>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 90
mdtest!(
    backslack_no_break,
    r"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Foo\
    ----
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <h2>Foo\</h2>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 91
mdtest_ignore!(
    block_precedes_inline,
    "inline not supported",
    r#"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    `Foo
    ----
    `
    
    <a title="a lot
    ---
    of dashes"/>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <h2>`Foo</h2>
    <p>`</p>
    <h2>&lt;a title=&quot;a lot</h2>
    <p>of dashes&quot;/&gt;</p>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "#
);

// 92
mdtest!(
    blockquote_continuation,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    > Foo
    ---
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <p>Foo</p>
    </blockquote>
    <hr />
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 93
mdtest!(
    blockquote_continuation_2,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    > foo
    bar
    ===
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <p>foo
    bar
    ===</p>
    </blockquote>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 94
mdtest_ignore!(
    list_continuation,
    "list not supported",
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    - Foo
    ---
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <ul>
    <li>Foo</li>
    </ul>
    <hr />
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 95
mdtest!(
    merged_paragraph,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Foo
    Bar
    ---
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <h2>Foo
    Bar</h2>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 96
mdtest!(
    consecutive_headings,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ---
    Foo
    ---
    Bar
    ---
    Baz
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <hr />
    <h2>Foo</h2>
    <h2>Bar</h2>
    <p>Baz</p>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 97
mdtest!(
    empty_heading,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    ====
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <p>====</p>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 98
mdtest!(
    paragraph_content_only,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ---
    ---
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <hr />
    <hr />
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 99
mdtest_ignore!(
    paragraph_content_only_2,
    "list not supported",
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    - foo
    -----
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <ul>
    <li>foo</li>
    </ul>
    <hr />
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 100
mdtest!(
    paragraph_content_only_3,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        foo
    ---
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <pre><code>foo
    </code></pre>
    <hr />
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 101
mdtest!(
    paragraph_content_only_4,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    > foo
    -----
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <p>foo</p>
    </blockquote>
    <hr />
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 102
mdtest!(
    literal_block_quote_heading,
    r"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    \> foo
    ------
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <h2>&gt; foo</h2>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 103
mdtest!(
    blank_line_to_separate,
    r"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Foo

    bar
    ---
    baz
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <p>Foo</p>
    <h2>bar</h2>
    <p>baz</p>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 104
mdtest!(
    blank_line_for_thematic_break,
    r"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Foo
    bar
    
    ---
    
    baz
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <p>Foo
    bar</p>
    <hr />
    <p>baz</p>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 105
mdtest!(
    unsupported_chars_for_thematic_break,
    r"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Foo
    bar
    * * *
    baz
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <p>Foo
    bar</p>
    <hr />
    <p>baz</p>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 106
mdtest!(
    escaped_delim,
    r"
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    Foo
    bar
    \---
    baz
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <p>Foo
    bar
    ---
    baz</p>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);
