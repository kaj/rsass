//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2000.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".m__exhibit-header--medium {\
            \n    @extend #{&}--plain;\
            \n    &--plain {\
            \n        font-size: --&;\
            \n    }\
            \n}"
        )
        .unwrap(),
        ".m__exhibit-header--medium--plain, .m__exhibit-header--medium {\
        \n  font-size: -- .m__exhibit-header--medium--plain;\
        \n}\
        \n"
    );
}
