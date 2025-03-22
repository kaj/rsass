//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1283.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1283")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\
             \n@use \"sass:meta\";\n\
             \n$map: map.merge((1 2: 3), (2 1: 3));\n\
             \n.test {\
             \n  test: meta.inspect($map);\
             \n}\n"),
        ".test {\
         \n  test: (1 2: 3, 2 1: 3);\
         \n}\n"
    );
}
