//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1043.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
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
