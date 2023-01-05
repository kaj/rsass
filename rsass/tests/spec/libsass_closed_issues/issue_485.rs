//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_485.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_485")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(
            "@media not all and (monochrome) { a {foo: bar} }\
             \n@media not screen and (color), print and (color) { a {foo: bar} }\
             \n@media (not (screen and (color))), print and (color) { a {foo: bar} }\n"
        ),
        "@media not all and (monochrome) {\
         \n  a {\
         \n    foo: bar;\
         \n  }\
         \n}\
         \n@media not screen and (color), print and (color) {\
         \n  a {\
         \n    foo: bar;\
         \n  }\
         \n}\
         \n@media not (color), print and (color) {\
         \n  a {\
         \n    foo: bar;\
         \n  }\
         \n}\n"
    );
}
