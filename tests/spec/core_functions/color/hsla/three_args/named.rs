//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla/three_args/named.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner()
            .ok("a {b: hsla($hue: 0, $saturation: 100%, $lightness: 50%)}\n"),
        "a {\
         \n  b: red;\
         \n}\n"
    );
}
