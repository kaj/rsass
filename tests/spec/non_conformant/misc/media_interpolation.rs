//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/media_interpolation.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$baz: 12;\
            \n@media bar#{$baz} {a {b: c}}\
            \n"
        )
        .unwrap(),
        "@media bar12 {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n"
    );
}
