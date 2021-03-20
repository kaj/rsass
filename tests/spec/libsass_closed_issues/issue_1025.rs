//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1025.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin m() {\
            \n  .a & {\
            \n    @content;\
            \n  }\
            \n}\
            \n\
            \n:not(:last-of-type) {\
            \n  top: 10px;\
            \n  @include m {\
            \n    top: 10px;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ":not(:last-of-type) {\
        \n  top: 10px;\
        \n}\
        \n.a :not(:last-of-type) {\
        \n  top: 10px;\
        \n}\
        \n"
    );
}
