//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1092.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1092")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$bar: \"\";\
             \n$baz: \" \";\
             \na { a: foo #{\"\"}; }\
             \nb { b: foo #{\" \"}; }\
             \nc { c: foo #{$bar}; }\
             \nd { d: foo #{$baz}; }\n"),
        "a {\
         \n  a: foo;\
         \n}\
         \nb {\
         \n  b: foo  ;\
         \n}\
         \nc {\
         \n  c: foo;\
         \n}\
         \nd {\
         \n  d: foo  ;\
         \n}\n"
    );
}
