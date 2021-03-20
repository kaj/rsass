//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/three_args/named.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@use \'sass:color\';\
            \na {b: color.hwb($hue: 0, $whiteness: 30%, $blackness: 40%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #994d4d;\
        \n}\
        \n"
    );
}
