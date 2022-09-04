//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/three_args/w3c/blue_to_red.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("blue_to_red")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {\
             \n  step-1: hsl(240, 100%, 50%);\
             \n  step-2: hsl(252, 100%, 50%);\
             \n  step-3: hsl(264, 100%, 50%);\
             \n  step-4: hsl(276, 100%, 50%);\
             \n  step-5: hsl(288, 100%, 50%);\
             \n  step-6: hsl(300, 100%, 50%);\
             \n  step-7: hsl(312, 100%, 50%);\
             \n  step-8: hsl(324, 100%, 50%);\
             \n  step-9: hsl(336, 100%, 50%);\
             \n  step-10: hsl(348, 100%, 50%);\
             \n  step-11: hsl(360, 100%, 50%);\
             \n}\n"),
        "a {\
         \n  step-1: hsl(240deg, 100%, 50%);\
         \n  step-2: hsl(252deg, 100%, 50%);\
         \n  step-3: hsl(264deg, 100%, 50%);\
         \n  step-4: hsl(276deg, 100%, 50%);\
         \n  step-5: hsl(288deg, 100%, 50%);\
         \n  step-6: hsl(300deg, 100%, 50%);\
         \n  step-7: hsl(312deg, 100%, 50%);\
         \n  step-8: hsl(324deg, 100%, 50%);\
         \n  step-9: hsl(336deg, 100%, 50%);\
         \n  step-10: hsl(348deg, 100%, 50%);\
         \n  step-11: hsl(0deg, 100%, 50%);\
         \n}\n"
    );
}
