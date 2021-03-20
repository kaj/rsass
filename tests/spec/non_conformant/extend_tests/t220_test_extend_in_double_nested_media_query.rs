//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/220_test_extend_in_double_nested_media_query.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "@media all {\
            \n@media (orientation: landscape) {\
            \n  %foo {color: blue}\
            \n  .bar {@extend %foo}\
            \n}\
            \n}\
            \n"
        )
        .unwrap(),
        "@media (orientation: landscape) {\
        \n  .bar {\
        \n    color: blue;\
        \n  }\
        \n}\
        \n"
    );
}
