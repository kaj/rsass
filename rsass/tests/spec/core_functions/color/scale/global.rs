//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/global.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("global")
}

#[test]
fn legacy() {
    assert_eq!(
        runner().ok("a {b: scale-color(pink, $blue: 20%)}\n"),
        "a {\
         \n  b: rgb(255, 192, 213.4);\
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
         \n  b: rgb(250.9720040643, 194.0861924561, 203.8303793932);\
         \n}\n"
    );
}
