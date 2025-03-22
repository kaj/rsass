//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_185/media_level_4.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("media_level_4")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  @media (pointer: none) {\
             \n    content: foo;\n\
             \n    @media (scripting) {\
             \n      content: baz;\n\
             \n      @media (light-level: dim) {\
             \n        content: bar;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n"),
        "@media (pointer: none) {\
         \n  .foo {\
         \n    content: foo;\
         \n  }\
         \n}\
         \n@media (pointer: none) and (scripting) {\
         \n  .foo {\
         \n    content: baz;\
         \n  }\
         \n}\
         \n@media (pointer: none) and (scripting) and (light-level: dim) {\
         \n  .foo {\
         \n    content: bar;\
         \n  }\
         \n}\n"
    );
}
