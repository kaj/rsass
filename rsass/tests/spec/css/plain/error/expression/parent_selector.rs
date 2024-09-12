//! Tests auto-converted from "sass-spec/spec/css/plain/error/expression/parent_selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("parent_selector")
        .mock_file("plain.css", "a {\n  x: &;\n}\n")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("@use \'plain\'"),
        "Error: The parent selector isn\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: &;\
         \n  |      ^\
         \n  \'\
         \n  plain.css 2:6   @use\
         \n  input.scss 1:1  root stylesheet",
    );
}
