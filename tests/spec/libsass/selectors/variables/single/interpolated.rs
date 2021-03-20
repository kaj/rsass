//! Tests auto-converted from "sass-spec/spec/libsass/selectors/variables/single/interpolated.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  $bar: &;\
            \n  content: #{$bar};\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  content: .foo;\
        \n}\
        \n"
    );
}
