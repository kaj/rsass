//! Tests auto-converted from "sass-spec/spec/core_functions/color/lab/special_functions/slash_list.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("slash_list")
}

#[test]
#[ignore] // unexepected error
fn alpha() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(list.slash(1% 2 3, var(--c))));\n"),
        "a {\
         \n  value: lab(1% 2 3 / var(--c));\
         \n  type: string;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn channels() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(list.slash(var(--foo), 0.4)));\n"),
        "a {\
         \n  value: lab(var(--foo) / 0.4);\
         \n  type: string;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn some_channels() {
    assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(list.slash(1% var(--foo), 0.4)));\n"
        ),
        "a {\
         \n  value: lab(1% var(--foo) / 0.4);\
         \n  type: string;\
         \n}\n"
    );
}
