//! Tests auto-converted from "sass-spec/spec/css/plain/error/expression/map.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().mock_file("plain.css", "a {\n  x: (y: z);\n}\n")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err("@import \'plain\'"),
        "Error: Parentheses aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   x: (y: z);\
         \n  |      ^\
         \n  \'\
         \n  plain.css 2:6   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
