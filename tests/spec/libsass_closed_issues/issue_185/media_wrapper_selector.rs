//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_185/media_wrapper_selector.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@media all {\
            \n  .bar { content: baz; }\
            \n\
            \n  @media (min-width: 1337px) {\
            \n    .foo { content: bar; }\
            \n  }\
            \n}\
            \n\
            \n@media all {\
            \n  .bar { content: baz; }\
            \n\
            \n  @media (min-width: 1337px) {\
            \n    .baz { content: foo; }\
            \n\
            \n    @media (max-width: 42em) {\
            \n      .foo { content: bar; }\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media all {\
        \n  .bar {\
        \n    content: baz;\
        \n  }\
        \n}\
        \n@media (min-width: 1337px) {\
        \n  .foo {\
        \n    content: bar;\
        \n  }\
        \n}\
        \n@media all {\
        \n  .bar {\
        \n    content: baz;\
        \n  }\
        \n}\
        \n@media (min-width: 1337px) {\
        \n  .baz {\
        \n    content: foo;\
        \n  }\
        \n}\
        \n@media (min-width: 1337px) and (max-width: 42em) {\
        \n  .foo {\
        \n    content: bar;\
        \n  }\
        \n}\
        \n"
    );
}
