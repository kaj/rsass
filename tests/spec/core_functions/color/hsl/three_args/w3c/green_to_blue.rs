//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/three_args/w3c/green_to_blue.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("green_to_blue")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {\
             \n  step-1: hsl(120, 100%, 50%);\
             \n  step-2: hsl(132, 100%, 50%);\
             \n  step-3: hsl(144, 100%, 50%);\
             \n  step-4: hsl(156, 100%, 50%);\
             \n  step-5: hsl(168, 100%, 50%);\
             \n  step-6: hsl(180, 100%, 50%);\
             \n  step-7: hsl(192, 100%, 50%);\
             \n  step-8: hsl(204, 100%, 50%);\
             \n  step-9: hsl(216, 100%, 50%);\
             \n  step-10: hsl(228, 100%, 50%);\
             \n  step-11: hsl(240, 100%, 50%);\
             \n}\n"),
        "a {\
         \n  step-1: hsl(120deg, 100%, 50%);\
         \n  step-2: hsl(132deg, 100%, 50%);\
         \n  step-3: hsl(144deg, 100%, 50%);\
         \n  step-4: hsl(156deg, 100%, 50%);\
         \n  step-5: hsl(168deg, 100%, 50%);\
         \n  step-6: hsl(180deg, 100%, 50%);\
         \n  step-7: hsl(192deg, 100%, 50%);\
         \n  step-8: hsl(204deg, 100%, 50%);\
         \n  step-9: hsl(216deg, 100%, 50%);\
         \n  step-10: hsl(228deg, 100%, 50%);\
         \n  step-11: hsl(240deg, 100%, 50%);\
         \n}\n"
    );
}
