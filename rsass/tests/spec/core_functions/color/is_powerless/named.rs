//! Tests auto-converted from "sass-spec/spec/core_functions/color/is_powerless/named.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("named")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.is-powerless($color: red, $channel: \"a\", $space: lab)}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
