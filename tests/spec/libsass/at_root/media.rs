//! Tests auto-converted from "sass-spec/spec/libsass/at-root/media.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  @at-root {\
             \n    @media print {\
             \n      bar {\
             \n        color: red;\
             \n      }\
             \n    }\n\
             \n    baz {\
             \n      @media speech {\
             \n        color: blue;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n"),
        "@media print {\
         \n  bar {\
         \n    color: red;\
         \n  }\
         \n}\
         \n@media speech {\
         \n  baz {\
         \n    color: blue;\
         \n  }\
         \n}\n"
    );
}
