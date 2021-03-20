//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_185/merge_no_repeat.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  content: foo;\
            \n\
            \n  @media only screen and (min-width: 1337px) {\
            \n    content: bar;\
            \n\
            \n    @media only screen and (max-width: 42em) {\
            \n      content: baz;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  content: foo;\
        \n}\
        \n@media only screen and (min-width: 1337px) {\
        \n  .foo {\
        \n    content: bar;\
        \n  }\
        \n}\
        \n@media only screen and (min-width: 1337px) and (max-width: 42em) {\
        \n  .foo {\
        \n    content: baz;\
        \n  }\
        \n}\
        \n"
    );
}
