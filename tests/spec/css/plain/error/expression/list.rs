//! Tests auto-converted from "sass-spec/spec/css/plain/error/expression/list.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("empty/plain.css", "a {\n  x: ();\n}\n")
        .mock_file("empty_comma/plain.css", "a {\n  x: (,);\n}\n")
}

#[test]
#[ignore] // missing error
fn empty() {
    let runner = runner().with_cwd("empty");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: Parentheses aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: ();\
         \n  |      ^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn empty_comma() {
    let runner = runner().with_cwd("empty_comma");
    assert_eq!(
        runner.err("@import \'plain\'"),
        "Error: Parentheses aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: (,);\
         \n  |      ^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
