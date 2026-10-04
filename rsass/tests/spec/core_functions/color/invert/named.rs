//! Tests auto-converted from "sass-spec/spec/core_functions/color/invert/named.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("named")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.invert($color: turquoise, $weight: 10%, $space: oklch)}\n"
        ),
        "a {\
         \n  b: rgb(28.355744452%, 79.4315282085%, 62.3391300852%);\
         \n}\n"
    );
}
