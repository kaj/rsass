//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1281.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1281")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$quoted: \"green\";\
             \n$unquoted: green;\n\
             \n.test {\
             \n  string: type-of($quoted);\
             \n  color: type-of($unquoted);\
             \n  string: type-of(\"green\");\
             \n  color: type-of(green);\
             \n}\n"),
        ".test {\
         \n  string: string;\
         \n  color: color;\
         \n  string: string;\
         \n  color: color;\
         \n}\n"
    );
}
