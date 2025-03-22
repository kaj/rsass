//! Tests auto-converted from "sass-spec/spec/core_functions/color/channel/named.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("named")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.channel($color: pink, $channel: \"hue\", $space: hsl)}\n"
        ),
        "a {\
         \n  b: 349.5238095238deg;\
         \n}\n"
    );
}
