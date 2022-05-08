//! Tests auto-converted from "sass-spec/spec/css/plain/error/expression/parent_selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().mock_file("plain.css", "a {\n  x: &;\n}\n")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("@import \'plain\'"),
        "Error: The parent selector isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: &;\
         \n  |      ^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
