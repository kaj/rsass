//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_803.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "\
            \n$query-string: \"(min-width: 0) and (max-width: 599px),  (min-width: 600px) and (max-width: 899px)\";\
            \n@media #{$query-string} {\
            \n  .foo {\
            \n    content: bar;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media (min-width: 0) and (max-width: 599px), (min-width: 600px) and (max-width: 899px) {\
        \n  .foo {\
        \n    content: bar;\
        \n  }\
        \n}\
        \n"
    );
}
