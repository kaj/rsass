//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_803.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(
            "\
             \n$query-string: \"(min-width: 0) and (max-width: 599px),  (min-width: 600px) and (max-width: 899px)\";\
             \n@media #{$query-string} {\
             \n  .foo {\
             \n    content: bar;\
             \n  }\
             \n}\n"
        ),
        "@media (min-width: 0) and (max-width: 599px), (min-width: 600px) and (max-width: 899px) {\
         \n  .foo {\
         \n    content: bar;\
         \n  }\
         \n}\n"
    );
}
