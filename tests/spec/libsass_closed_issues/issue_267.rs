//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_267.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$x: foo;\r\
            \n@keyframes $x {\r\
            \n  to {\r\
            \n    blah: blah;\r\
            \n  }\r\
            \n}"
        )
        .unwrap(),
        "@keyframes $x {\
        \n  to {\
        \n    blah: blah;\
        \n  }\
        \n}\
        \n"
    );
}
