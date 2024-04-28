//! Tests auto-converted from "sass-spec/spec/css/plain/error/statement/silent_comment.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("silent_comment")
        .mock_file("plain.css", "// silent\n")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err("@import \'plain\'\n"),
        "Error: Silent comments aren\'t allowed in plain CSS.\
         \n  ,\
         \n1 | // silent\
         \n  | ^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:1   @import\
         \n  input.scss 1:9  root stylesheet",
    );
}
