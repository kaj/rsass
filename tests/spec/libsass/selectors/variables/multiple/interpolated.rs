//! Tests auto-converted from "sass-spec/spec/libsass/selectors/variables/multiple/interpolated.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo a,\
            \n.bar p {\
            \n  $bar: &;\
            \n  content: #{$bar};\
            \n}"
        )
        .unwrap(),
        ".foo a,\
        \n.bar p {\
        \n  content: .foo a, .bar p;\
        \n}\
        \n"
    );
}
