//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1207.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function test($pos) {\
             \n  @return test-#{$pos};\
             \n}\n\
             \n.foo {\
             \n  content: test(str-slice(\'scale-0\', 7));   // Nope\
             \n  content: test-#{str-slice(\'scale-0\', 7)}; // Yep\
             \n}"),
        ".foo {\
         \n  content: test-0;\
         \n  content: test-0;\
         \n}\n"
    );
}
