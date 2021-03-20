//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1557.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$xs-break: 30em;@media ALL AND (max-width: $xs-break) {header {display: none;}}\
            \n"
        )
        .unwrap(),
        "@media ALL and (max-width: 30em) {\
        \n  header {\
        \n    display: none;\
        \n  }\
        \n}\
        \n"
    );
}
