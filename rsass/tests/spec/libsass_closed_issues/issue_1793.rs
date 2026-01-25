//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1793.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1793")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@media (max-width: (2px*5in)) {\
             \n  foo { bar: baz; }\
             \n}\n"),
        "@media (max-width: calc(10px * 1in)) {\
         \n  foo {\
         \n    bar: baz;\
         \n  }\
         \n}\n"
    );
}
