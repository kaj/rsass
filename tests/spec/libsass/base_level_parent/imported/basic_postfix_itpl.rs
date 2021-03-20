//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/imported/basic-postfix-itpl.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass("@import \"include.scss\";").unwrap(),
        "post foo {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}
