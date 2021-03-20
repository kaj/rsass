//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1291.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin spec1($decimal) {\
            \n  $decimal: unquote($decimal) * -1;\
            \n  value: $decimal;\
            \n}\
            \n\
            \n@mixin spec2($decimal) {\
            \n  $decimal: -1 * unquote($decimal);\
            \n  value: $decimal;\
            \n}\
            \n\
            \n@mixin spec3($decimal) {\
            \n  value: #{$decimal * -1};\
            \n}\
            \n\
            \n.my-element {\
            \n  @include spec1(3);\
            \n  @include spec1(-3);\
            \n  @include spec2(5);\
            \n  @include spec2(-5);\
            \n  @include spec3(7);\
            \n  @include spec3(-7);\
            \n}"
        )
        .unwrap(),
        ".my-element {\
        \n  value: -3;\
        \n  value: 3;\
        \n  value: -5;\
        \n  value: 5;\
        \n  value: -7;\
        \n  value: 7;\
        \n}\
        \n"
    );
}
