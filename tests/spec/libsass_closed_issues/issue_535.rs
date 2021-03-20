//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_535.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$width: 10;\
            \n\
            \n.test {\
            \n  margin-left: - 54 * $width - 1;\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  margin-left: -541;\
        \n}\
        \n"
    );
}
