//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1043.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1043")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".component{\
             \n    color: red;\
             \n    @at-root{\
             \n        #{&}--foo#{&}--bar {\
             \n            color: blue;\
             \n        }\
             \n    }\
             \n}\n\
             \n.test{\
             \n        .selector#{&} {\
             \n            color: blue;\
             \n        }\
             \n}"),
        ".component {\
         \n  color: red;\
         \n}\
         \n.component--foo.component--bar {\
         \n  color: blue;\
         \n}\
         \n.test .selector.test {\
         \n  color: blue;\
         \n}\n"
    );
}
