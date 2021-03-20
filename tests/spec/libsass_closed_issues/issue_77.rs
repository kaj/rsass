//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_77.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin m {\r\
            \n  .m {\r\
            \n    color: red;\r\
            \n    @content;\r\
            \n  }\r\
            \n}\r\
            \ndiv.a {\r\
            \n  @include m;\r\
            \n}"
        )
        .unwrap(),
        "div.a .m {\
        \n  color: red;\
        \n}\
        \n"
    );
}
