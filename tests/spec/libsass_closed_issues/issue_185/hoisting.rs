//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_185/hoisting.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@media only screen {\
            \n  .foo {\
            \n    content: bar;\
            \n\
            \n    @media (min-width: 1337px) {\
            \n      content: baz;\
            \n    }\
            \n\
            \n    content: foo;\
            \n  }\
            \n}\
            \n\
            \n$foo: \"(min-width: 0) and (max-width: 599px),  (min-width: 600px) and (max-width: 899px)\";\
            \n@media #{$foo} {\
            \n  $bar: \"(min-width: 0) and (max-width: 599px)\";\
            \n  @media #{$bar} {\
            \n    .foo {\
            \n      content: bar;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media only screen {\
        \n  .foo {\
        \n    content: bar;\
        \n    content: foo;\
        \n  }\
        \n}\
        \n@media only screen and (min-width: 1337px) {\
        \n  .foo {\
        \n    content: baz;\
        \n  }\
        \n}\
        \n@media (min-width: 0) and (max-width: 599px) and (min-width: 0) and (max-width: 599px), (min-width: 600px) and (max-width: 899px) and (min-width: 0) and (max-width: 599px) {\
        \n  .foo {\
        \n    content: bar;\
        \n  }\
        \n}\
        \n"
    );
}
