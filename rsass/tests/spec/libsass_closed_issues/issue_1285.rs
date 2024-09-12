//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1285.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1285")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \n.container {\
             \n  @for $i from 1 through 3 {\
             \n    @at-root .box-#{$i} {\
             \n      color: color.adjust(red,$lightness: -($i * 5%));\
             \n    }\
             \n  }\n\
             \n // Control\
             \n @at-root .outside-child {\
             \n   background-color: blue;\
             \n  }\
             \n}\n"),
        ".box-1 {\
         \n  color: rgb(229.5, 0, 0);\
         \n}\
         \n.box-2 {\
         \n  color: #cc0000;\
         \n}\
         \n.box-3 {\
         \n  color: rgb(178.5, 0, 0);\
         \n}\
         \n.outside-child {\
         \n  background-color: blue;\
         \n}\n"
    );
}
