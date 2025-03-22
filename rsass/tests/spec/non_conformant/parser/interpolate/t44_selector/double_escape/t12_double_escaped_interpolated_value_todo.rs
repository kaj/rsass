//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/44_selector/double_escape/12_double_escaped_interpolated_value_todo.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("12_double_escaped_interpolated_value_todo")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$key: \'bar\';\
             \n.test12#{\'\\\\@#{$key}\'} { content: \'1.2\'; }\n"),
        ".test12\\@bar {\
         \n  content: \"1.2\";\
         \n}\n"
    );
}
