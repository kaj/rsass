//! Tests auto-converted from "sass-spec/spec/css/plain/import/scss_takes_precedence.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \"other\";\
            \n"
        )
        .unwrap(),
        "other {\
        \n  syntax: scss;\
        \n}\
        \n"
    );
}
