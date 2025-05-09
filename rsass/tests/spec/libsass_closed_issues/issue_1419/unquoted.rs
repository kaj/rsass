//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1419/unquoted.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("unquoted")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \nfoo {\
             \n  foo: string.to-upper-case(ab\\63 d);\
             \n}\n"),
        "foo {\
         \n  foo: ABCD;\
         \n}\n"
    );
}
