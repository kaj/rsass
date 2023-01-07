//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1786.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1786")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("$input: \"\\0_\\a_\\A\";\n\
             \ntest {\
             \n    bug1: \"#{\"_\\a\" + b}\";\
             \n    bug2: \"#{a $input}\";\
             \n}\n"),
        "@charset \"UTF-8\";\
         \ntest {\
         \n  bug1: \"_\\a b\";\
         \n  bug2: \"a �_ _ \";\
         \n}\n"
    );
}
