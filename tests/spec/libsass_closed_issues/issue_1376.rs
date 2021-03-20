//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1376.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".div{\
            \n  $foo: 1, null, 2, null, 3;\
            \n\
            \n  content: \"#{$foo}\";\
            \n}"
        )
        .unwrap(),
        ".div {\
        \n  content: \"1, 2, 3\";\
        \n}\
        \n"
    );
}
