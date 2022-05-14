//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2169.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2169")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function test($a, $b) {\
             \n  @return \"#{$a}\" + \"#{$b}\" + \"\" + \"\";\
             \n}\n\
             \nfoo {\
             \n  content: test(\'bim\', \'baz\');\
             \n}"),
        "foo {\
         \n  content: \"bimbaz\";\
         \n}\n"
    );
}
