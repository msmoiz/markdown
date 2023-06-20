mod macros;

// 228
mdtest!(
    simple,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    > # Foo
    > bar
    > baz
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <h1>Foo</h1>
    <p>bar
    baz</p>
    </blockquote>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 229
mdtest!(
    optional_trailing_space,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ># Foo
    >bar
    > baz
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <h1>Foo</h1>
    <p>bar
    baz</p>
    </blockquote>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 230
mdtest!(
    up_to_three_leading_spaces,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
       > # Foo
       > bar
     > baz
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <h1>Foo</h1>
    <p>bar
    baz</p>
    </blockquote>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 231
mdtest!(
    four_leading_spaces,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        > # Foo
        > bar
        > baz
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <pre><code>&gt; # Foo
    &gt; bar
    &gt; baz
    </code></pre>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 232
mdtest!(
    lazy,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    > # Foo
    > bar
    baz
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <h1>Foo</h1>
    <p>bar
    baz</p>
    </blockquote>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 233
mdtest!(
    mixed_lazy,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    > bar
    baz
    > foo
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <p>bar
    baz
    foo</p>
    </blockquote>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 234
mdtest!(
    lazy_thematic_break,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    > foo
    ---
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <p>foo</p>
    </blockquote>
    <hr />
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 235
mdtest_ignore!(
    lazy_list,
    "list not supported",
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    > - foo
    - bar
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <ul>
    <li>foo</li>
    </ul>
    </blockquote>
    <ul>
    <li>bar</li>
    </ul>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 236
mdtest!(
    lazy_indent_code,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    >     foo
        bar
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <pre><code>foo
    </code></pre>
    </blockquote>
    <pre><code>bar
    </code></pre>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 237
mdtest!(
    lazy_fenced_code,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    > ```
    foo
    ```
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <pre><code></code></pre>
    </blockquote>
    <p>foo</p>
    <pre><code></code></pre>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 238
mdtest!(
    lazy_deep_list,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    > foo
        - bar
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <p>foo
    - bar</p>
    </blockquote>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 239
mdtest!(
    empty,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    >
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    </blockquote>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 240
mdtest!(
    empty_2,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    >
    >  
    > 
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    </blockquote>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 241
mdtest!(
    initial_final_blank,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    >
    > foo
    >  
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <p>foo</p>
    </blockquote>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 242
mdtest!(
    blank_between,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    > foo

    > bar
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <p>foo</p>
    </blockquote>
    <blockquote>
    <p>bar</p>
    </blockquote>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 243
mdtest!(
    consecutive_para,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    > foo
    > bar
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <p>foo
    bar</p>
    </blockquote>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 244
mdtest!(
    consecutive_para_2,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    > foo
    >
    > bar
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <p>foo</p>
    <p>bar</p>
    </blockquote>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 245
mdtest!(
    interrupt_para,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    foo
    > bar
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <p>foo</p>
    <blockquote>
    <p>bar</p>
    </blockquote>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 246
mdtest!(
    blank_line_not_needed,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    > aaa
    ***
    > bbb
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <p>aaa</p>
    </blockquote>
    <hr />
    <blockquote>
    <p>bbb</p>
    </blockquote>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 247
mdtest!(
    blank_line_needed,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    > bar
    baz
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <p>bar
    baz</p>
    </blockquote>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 248
mdtest!(
    blank_line_needed_2,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    > bar

    baz
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <p>bar</p>
    </blockquote>
    <p>baz</p>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 249
mdtest!(
    blank_line_needed_3,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    > bar
    >
    baz
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <p>bar</p>
    </blockquote>
    <p>baz</p>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 250
mdtest!(
    lazy_multiple,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    > > > foo
    bar
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <blockquote>
    <blockquote>
    <p>foo
    bar</p>
    </blockquote>
    </blockquote>
    </blockquote>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 251
mdtest!(
    lazy_multiple_2,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    >>> foo
    > bar
    >>baz
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <blockquote>
    <blockquote>
    <p>foo
    bar
    baz</p>
    </blockquote>
    </blockquote>
    </blockquote>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);

// 252
mdtest!(
    indent_code_spaces,
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    >     code

    >    not code
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <blockquote>
    <pre><code>code
    </code></pre>
    </blockquote>
    <blockquote>
    <p>not code</p>
    </blockquote>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);
