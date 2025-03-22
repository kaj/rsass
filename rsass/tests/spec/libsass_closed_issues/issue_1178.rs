//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1178.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1178")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\n\
             \n$foo: ((4, 5), 6, (7 8) 9);\n\
             \nbar {\
             \n  a: $foo;\
             \n  f: 1 2 3 + $foo;\
             \n  b: 1, 2, 3 + (2 ($foo));\
             \n  x: meta.inspect($foo);\
             \n}\n"),
        "bar {\
         \n  a: 4, 5, 6, 7 8 9;\
         \n  f: 1 2 34, 5, 6, 7 8 9;\
         \n  b: 1, 2, 32 4, 5, 6, 7 8 9;\
         \n  x: (4, 5), 6, (7 8) 9;\
         \n}\n"
    );
}
