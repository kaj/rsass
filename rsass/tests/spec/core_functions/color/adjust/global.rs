//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust/global.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("global")
}

#[test]
fn legacy() {
    assert_eq!(
        runner().ok("a {b: adjust-color(red, $red: -50)}\n"),
        "a {\
         \n  b: #cd0000;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn non_legacy() {
    assert_eq!(
        runner()
            .ok("a {b: change-color(pink, $chroma: 0.01, $space: oklch)}\n"),
        "a {\
         \n  b: rgb(217.7587670223, 208.8497864408, 210.1600909257);\
         \n}\n"
    );
}
