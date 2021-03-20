//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_73.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin box-shadow($shadow...) { \r\
            \n  -webkit-box-shadow: $shadow;\r\
            \n     -moz-box-shadow: $shadow;\r\
            \n          box-shadow: $shadow;\r\
            \n}"
        )
        .unwrap(),
        ""
    );
}
