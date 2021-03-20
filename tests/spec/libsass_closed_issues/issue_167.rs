//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_167.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "%l-cell, .l-cell {\r\
            \n  margin: 0 auto;\r\
            \n  max-width: 1000px;\r\
            \n}"
        )
        .unwrap(),
        ".l-cell {\
        \n  margin: 0 auto;\
        \n  max-width: 1000px;\
        \n}\
        \n"
    );
}
