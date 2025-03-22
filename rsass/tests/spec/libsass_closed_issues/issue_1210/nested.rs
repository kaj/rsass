//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1210/nested.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("nested")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  color: blue;\n\
             \n  baz {\
             \n    color: purple;\n\
             \n    @at-root {\
             \n      bar {\
             \n        color: red;\
             \n      }\
             \n    }\
             \n  }\
             \n}\n\
             \nfoo {\
             \n  color: blue;\n\
             \n  baz {\
             \n    color: purple;\n\
             \n    @at-root bar {\
             \n      color: red;\
             \n    }\
             \n  }\
             \n}\n"),
        "foo {\
         \n  color: blue;\
         \n}\
         \nfoo baz {\
         \n  color: purple;\
         \n}\
         \nbar {\
         \n  color: red;\
         \n}\
         \nfoo {\
         \n  color: blue;\
         \n}\
         \nfoo baz {\
         \n  color: purple;\
         \n}\
         \nbar {\
         \n  color: red;\
         \n}\n"
    );
}
