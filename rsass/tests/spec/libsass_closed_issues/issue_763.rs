//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_763.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_763")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  a: str-slice(\"abcd\", 1, 1);\
             \n  b: str-slice(\'abcd\', 1, 1);\
             \n  c: str-slice(abcd, 1, 1);\n\
             \n  d: str-insert(\"abcd\", \"X\", 1);\
             \n  e: str-insert(\"abcd\", \'X\', 1);\
             \n  f: str-insert(\'abcd\', \"X\", 1);\
             \n  g: str-insert(\'abcd\', \'X\', 1);\
             \n}\n"),
        "foo {\
         \n  a: \"a\";\
         \n  b: \"a\";\
         \n  c: a;\
         \n  d: \"Xabcd\";\
         \n  e: \"Xabcd\";\
         \n  f: \"Xabcd\";\
         \n  g: \"Xabcd\";\
         \n}\n"
    );
}
