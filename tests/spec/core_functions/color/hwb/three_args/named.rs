//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/three_args/named.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \'sass:color\';\
             \na {b: color.hwb($hue: 0, $whiteness: 30%, $blackness: 40%)}\n"
        ),
        "a {\
         \n  b: #994d4d;\
         \n}\n"
    );
}
