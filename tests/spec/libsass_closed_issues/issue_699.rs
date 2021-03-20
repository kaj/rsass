//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_699.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".selector {\
            \n  color: invert(rebeccapurple);\
            \n}"
        )
        .unwrap(),
        ".selector {\
        \n  color: #99cc66;\
        \n}\
        \n"
    );
}
