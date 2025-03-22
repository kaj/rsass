//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1281.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1281")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n$quoted: \"green\";\
             \n$unquoted: green;\n\
             \n.test {\
             \n  string: meta.type-of($quoted);\
             \n  color: meta.type-of($unquoted);\
             \n  string: meta.type-of(\"green\");\
             \n  color: meta.type-of(green);\
             \n}\n"),
        ".test {\
         \n  string: string;\
         \n  color: color;\
         \n  string: string;\
         \n  color: color;\
         \n}\n"
    );
}
