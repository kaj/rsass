//! Tests auto-converted from "sass-spec/spec/css/plain/error/expression/parentheses.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("parentheses")
        .mock_file("plain.css", "a {\n  x: (y);\n}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().err("@use \'plain\'"),
        "Error: Parentheses aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: (y);\
         \n  |      ^^^\
         \n  \'\
         \n  plain.css 2:6   @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
