//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1168.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$namespace: \'test-\';\
            \n$column: 1;\
            \n\
            \n.#{$namespace}#{$column}\\/#{$column} {\
            \n  width: 100% !important;\
            \n}"
        )
        .unwrap(),
        ".test-1\\/1 {\
        \n  width: 100% !important;\
        \n}\
        \n"
    );
}
