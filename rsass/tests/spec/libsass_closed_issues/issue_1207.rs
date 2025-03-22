//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1207.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1207")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \n@function test($pos) {\
             \n  @return test-#{$pos};\
             \n}\n\
             \n.foo {\
             \n  content: test(string.slice(\'scale-0\', 7));   // Nope\
             \n  content: test-#{string.slice(\'scale-0\', 7)}; // Yep\
             \n}"),
        ".foo {\
         \n  content: test-0;\
         \n  content: test-0;\
         \n}\n"
    );
}
