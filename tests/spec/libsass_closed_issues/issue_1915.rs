//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1915.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin wrapper() {\
            \n  .wrapped {\
            \n    @content;\
            \n  }\
            \n}\
            \n\
            \n%ext {\
            \n  background: #000;\
            \n}\
            \n\
            \n@include wrapper() {\
            \n  @extend %ext;\
            \n}"
        )
        .unwrap(),
        ".wrapped {\
        \n  background: #000;\
        \n}\
        \n"
    );
}
