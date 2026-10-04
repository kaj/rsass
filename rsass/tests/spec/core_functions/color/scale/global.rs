//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/global.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("global")
}

#[test]
#[ignore] // wrong result
fn legacy() {
    assert_eq!(
        runner().ok("a {b: scale-color(pink, $blue: 20%)}\n"),
        "a {\
         \n  b: rgb(100%, 75.2941176471%, 83.6862745098%);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn non_legacy() {
    assert_eq!(
        runner()
            .ok("a {b: scale-color(pink, $chroma: -10%, $space: oklch)}\n"),
        "a {\
         \n  b: rgb(98.4203940332%, 76.1122323181%, 79.9334812156%);\
         \n}\n"
    );
}
