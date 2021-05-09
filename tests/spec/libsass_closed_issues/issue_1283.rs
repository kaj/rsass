//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1283.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$map: map-merge((1 2: 3), (2 1: 3));\n\
             \n.test {\
             \n  test: inspect($map);\
             \n}\n"),
        ".test {\
         \n  test: (1 2: 3, 2 1: 3);\
         \n}\n"
    );
}
