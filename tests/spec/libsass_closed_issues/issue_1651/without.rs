//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1651/without.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".a {\
            \n  display: block;\
            \n}\
            \n\
            \n.b {\
            \n  @at-root (without: media) {\
            \n    @extend .a;\
            \n  }\
            \n} \
            \n"
        )
        .unwrap(),
        ".a, .b {\
        \n  display: block;\
        \n}\
        \n"
    );
}
