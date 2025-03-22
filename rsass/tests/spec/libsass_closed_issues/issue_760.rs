//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_760.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_760")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \nfoo {\
             \n  quoted: string.slice(\"abcd\", 1, 0);\
             \n  unquoted: string.slice(abcd, 1, 0);\
             \n}\n"),
        "foo {\
         \n  quoted: \"\";\
         \n}\n"
    );
}
