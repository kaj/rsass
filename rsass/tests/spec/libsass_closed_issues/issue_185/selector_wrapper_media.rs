//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_185/selector_wrapper_media.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("selector_wrapper_media")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  @media all {\
             \n    content: baz;\n\
             \n    @media (min-width: 1337px) {\
             \n      content: bar;\
             \n    }\
             \n  }\
             \n}\n\
             \n.foo {\
             \n  @media all {\
             \n    content: baz;\n\
             \n    @media (min-width: 1337px) {\
             \n      content: foo;\n\
             \n      @media (max-width: 42em) {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n"),
        "@media all {\
         \n  .foo {\
         \n    content: baz;\
         \n  }\
         \n}\
         \n@media (min-width: 1337px) {\
         \n  .foo {\
         \n    content: bar;\
         \n  }\
         \n}\
         \n@media all {\
         \n  .foo {\
         \n    content: baz;\
         \n  }\
         \n}\
         \n@media (min-width: 1337px) {\
         \n  .foo {\
         \n    content: foo;\
         \n  }\
         \n}\
         \n@media (min-width: 1337px) and (max-width: 42em) {\
         \n  .foo {\
         \n    content: bar;\
         \n  }\
         \n}\n"
    );
}
