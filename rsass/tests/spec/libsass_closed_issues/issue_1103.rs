//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1103.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("issue_1103")
        .mock_file("_import.scss", "foo { bar: baz }\nbaz { bar: foo }\n\n@media screen and (max-width: 2) {\n    foo { bar: baz }\n    baz { bar: foo }\n}\n")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@import \"import\";\n\
             \n@media screen and (min-width: 1) {\
             \n    foo { bar: baz }\
             \n    baz { bar: foo }\
             \n}\n\
             \n@media screen and (min-width: 1) {\
             \n    @import \"import\";\
             \n}\n"),
        "foo {\
         \n  bar: baz;\
         \n}\
         \nbaz {\
         \n  bar: foo;\
         \n}\
         \n@media screen and (max-width: 2) {\
         \n  foo {\
         \n    bar: baz;\
         \n  }\
         \n  baz {\
         \n    bar: foo;\
         \n  }\
         \n}\
         \n@media screen and (min-width: 1) {\
         \n  foo {\
         \n    bar: baz;\
         \n  }\
         \n  baz {\
         \n    bar: foo;\
         \n  }\
         \n}\
         \n@media screen and (min-width: 1) {\
         \n  foo {\
         \n    bar: baz;\
         \n  }\
         \n  baz {\
         \n    bar: foo;\
         \n  }\
         \n}\
         \n@media screen and (min-width: 1) and (max-width: 2) {\
         \n  foo {\
         \n    bar: baz;\
         \n  }\
         \n  baz {\
         \n    bar: foo;\
         \n  }\
         \n}\n"
    );
}
