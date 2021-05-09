//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1285.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".container {\
             \n  @for $i from 1 through 3 {\
             \n    @at-root .box-#{$i} {\
             \n      color: darken(red,($i * 5));\
             \n    }\
             \n  }\n\
             \n // Control\
             \n @at-root .outside-child {\
             \n   background-color: blue;\
             \n  }\
             \n}\n"),
        ".box-1 {\
         \n  color: #e60000;\
         \n}\
         \n.box-2 {\
         \n  color: #cc0000;\
         \n}\
         \n.box-3 {\
         \n  color: #b30000;\
         \n}\
         \n.outside-child {\
         \n  background-color: blue;\
         \n}\n"
    );
}
