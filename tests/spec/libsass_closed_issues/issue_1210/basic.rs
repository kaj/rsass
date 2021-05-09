//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1210/basic.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  color: blue;\n\
             \n  @at-root {\
             \n    bar {\
             \n      color: red;\
             \n    }\
             \n  }\
             \n}\n\
             \nfoo {\
             \n  color: blue;\n\
             \n  @at-root bar {\
             \n    color: red;\
             \n  }\
             \n}\n\
             \nfoo {\
             \n  color: blue;\n\
             \n  @at-root bar {\
             \n    baz {\
             \n      color: red;\
             \n    }\
             \n  }\
             \n}\n\
             \nfoo {\
             \n  color: blue;\n\
             \n  @at-root {\
             \n    bar {\
             \n      baz {\
             \n        color: red;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\n"),
        "foo {\
         \n  color: blue;\
         \n}\
         \nbar {\
         \n  color: red;\
         \n}\
         \nfoo {\
         \n  color: blue;\
         \n}\
         \nbar {\
         \n  color: red;\
         \n}\
         \nfoo {\
         \n  color: blue;\
         \n}\
         \nbar baz {\
         \n  color: red;\
         \n}\
         \nfoo {\
         \n  color: blue;\
         \n}\
         \nbar baz {\
         \n  color: red;\
         \n}\n"
    );
}
