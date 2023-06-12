mod macros;

use indoc::indoc;
use markdown::to_html;

// 227
mdtest_ignore!(
    simple,
    "headings not supported",
    "
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~



    aaa



    # aaa

        


    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    <p>aaa</p>
    <h1>aaa</h1>
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    "
);
