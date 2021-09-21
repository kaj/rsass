//! Tests auto-converted from "sass-spec/spec/css/plain/error/expression/silent_comment.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().mock_file("plain.css", "a {\n  b: c // d\n     e;\n}\n")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err("@import \'plain\'\n"),
        "Error: Silent comments aren\'t allowed in plain CSS.\
         \n  ,\
         \n2 |   b: c // d\
         \n  |        ^^^^\
         \n  \'\
         \n  plain.css 2:8   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
