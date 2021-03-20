//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_893.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$gutter: 20px;\
            \n\
            \n.row {\
            \n  margin: 0 -$gutter;\
            \n}"
        )
        .unwrap(),
        ".row {\
        \n  margin: -20px;\
        \n}\
        \n"
    );
}
