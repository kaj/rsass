//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/three_args/named.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a {b: hsl($hue: 0, $saturation: 100%, $lightness: 50%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: red;\
        \n}\
        \n"
    );
}
