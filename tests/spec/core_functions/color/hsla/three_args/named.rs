//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla/three_args/named.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a {b: hsla($hue: 0, $saturation: 100%, $lightness: 50%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: red;\
        \n}\
        \n"
    );
}
