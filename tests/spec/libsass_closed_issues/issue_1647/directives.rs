//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1647/directives.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@foo #{\"directive\"} {\
            \n  .#{\"foo\"} { #{\"foo-prop\"}: #{\"foo-val\"}; }\
            \n}\
            \n"
        )
        .unwrap(),
        "@foo directive {\
        \n  .foo {\
        \n    foo-prop: foo-val;\
        \n  }\
        \n}\
        \n"
    );
}
