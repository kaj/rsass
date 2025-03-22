//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_578.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_578")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n$list: one foo three bar six seven;\
             \n$pos: list.set-nth($list, 2, two);\
             \n$neg: list.set-nth($pos, -3, four five);\n\
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
