//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1168.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1168")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$namespace: \'test-\';\
             \n$column: 1;\n\
             \n.#{$namespace}#{$column}\\/#{$column} {\
             \n  width: 100% !important;\
             \n}"),
        ".test-1\\/1 {\
         \n  width: 100% !important;\
         \n}\n"
    );
}
