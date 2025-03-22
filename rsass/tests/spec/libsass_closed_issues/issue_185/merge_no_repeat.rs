//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_185/merge_no_repeat.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("merge_no_repeat")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  content: foo;\n\
             \n  @media only screen and (min-width: 1337px) {\
             \n    content: bar;\n\
             \n    @media only screen and (max-width: 42em) {\
             \n      content: baz;\
             \n    }\
             \n  }\
             \n}\n"),
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
         \n}\n"
    );
}
