//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_823.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "%test {\
            \n  > {\
            \n    .red {\
            \n      color: #F00;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \np {\
            \n  @extend %test;\
            \n\
            \n  > {\
            \n    a {\
            \n      @extend %test;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "p > a > .red, p > .red {\
        \n  color: #F00;\
        \n}\
        \n"
    );
}
