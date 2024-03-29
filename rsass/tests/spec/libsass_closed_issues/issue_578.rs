//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_578.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_578")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$list: one foo three bar six seven;\
             \n$pos: set-nth($list, 2, two);\
             \n$neg: set-nth($pos, -3, four five);\n\
             \n.test {\
             \n  -positive: $pos;\
             \n  -negative: $neg;\
             \n}\n"),
        ".test {\
         \n  -positive: one two three bar six seven;\
         \n  -negative: one two three four five six seven;\
         \n}\n"
    );
}
