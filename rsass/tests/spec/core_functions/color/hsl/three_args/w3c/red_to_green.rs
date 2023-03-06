//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/three_args/w3c/red_to_green.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("red_to_green")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {\
             \n  step-1: hsl(0, 100%, 50%);\
             \n  step-2: hsl(12, 100%, 50%);\
             \n  step-3: hsl(24, 100%, 50%);\
             \n  step-4: hsl(36, 100%, 50%);\
             \n  step-5: hsl(48, 100%, 50%);\
             \n  step-6: hsl(60, 100%, 50%);\
             \n  step-7: hsl(72, 100%, 50%);\
             \n  step-8: hsl(84, 100%, 50%);\
             \n  step-9: hsl(96, 100%, 50%);\
             \n  step-10: hsl(108, 100%, 50%);\
             \n  step-11: hsl(120, 100%, 50%);\
             \n}\n"),
        "a {\
         \n  step-1: hsl(0, 100%, 50%);\
         \n  step-2: hsl(12, 100%, 50%);\
         \n  step-3: hsl(24, 100%, 50%);\
         \n  step-4: hsl(36, 100%, 50%);\
         \n  step-5: hsl(48, 100%, 50%);\
         \n  step-6: hsl(60, 100%, 50%);\
         \n  step-7: hsl(72, 100%, 50%);\
         \n  step-8: hsl(84, 100%, 50%);\
         \n  step-9: hsl(96, 100%, 50%);\
         \n  step-10: hsl(108, 100%, 50%);\
         \n  step-11: hsl(120, 100%, 50%);\
         \n}\n"
    );
}
