//! Tests auto-converted from "sass-spec/spec/core_functions/color/change/global.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("global")
}

#[test]
fn legacy() {
    assert_eq!(
        runner().ok("a {b: change-color(red, $blue: 50)}\n"),
        "a {\
         \n  b: #ff0032;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn non_legacy() {
    assert_eq!(
        runner()
            .ok("a {b: change-color(pink, $chroma: 0.06, $space: oklch)}\n"),
        "a {\
         \n  b: rgb(247.5333922506, 195.8115232441, 204.5286945902);\
         \n}\n"
    );
}
