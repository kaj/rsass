//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1210/media.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  @at-root {\
            \n    @media print {\
            \n      bar {\
            \n        color: red;\
            \n      }\
            \n    }\
            \n\
            \n    baz {\
            \n      @media speech {\
            \n        color: blue;\
            \n      }\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media print {\
        \n  bar {\
        \n    color: red;\
        \n  }\
        \n}\
        \n@media speech {\
        \n  baz {\
        \n    color: blue;\
        \n  }\
        \n}\
        \n"
    );
}
