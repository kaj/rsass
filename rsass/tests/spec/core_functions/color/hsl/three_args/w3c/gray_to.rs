//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/three_args/w3c/gray_to.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("gray_to")
}

#[test]
fn blue() {
    assert_eq!(
        runner().ok("a {\
             \n  step-1: hsl(240, 20%, 50%);\
             \n  step-2: hsl(240, 60%, 50%);\
             \n  step-3: hsl(240, 100%, 50%);\
             \n}\n"),
        "a {\
         \n  step-1: hsl(240, 20%, 50%);\
         \n  step-2: hsl(240, 60%, 50%);\
         \n  step-3: hsl(240, 100%, 50%);\
         \n}\n"
    );
}
#[test]
fn cyan() {
    assert_eq!(
        runner().ok("a {\
             \n  step-1: hsl(180, 20%, 50%);\
             \n  step-2: hsl(180, 60%, 50%);\
             \n  step-3: hsl(180, 100%, 50%);\
             \n}\n"),
        "a {\
         \n  step-1: hsl(180, 20%, 50%);\
         \n  step-2: hsl(180, 60%, 50%);\
         \n  step-3: hsl(180, 100%, 50%);\
         \n}\n"
    );
}
#[test]
fn green() {
    assert_eq!(
        runner().ok("a {\
             \n  step-1: hsl(120, 20%, 50%);\
             \n  step-2: hsl(120, 60%, 50%);\
             \n  step-3: hsl(120, 100%, 50%);\
             \n}\n"),
        "a {\
         \n  step-1: hsl(120, 20%, 50%);\
         \n  step-2: hsl(120, 60%, 50%);\
         \n  step-3: hsl(120, 100%, 50%);\
         \n}\n"
    );
}
#[test]
fn purple() {
    assert_eq!(
        runner().ok("a {\
             \n  step-1: hsl(300, 20%, 50%);\
             \n  step-2: hsl(300, 60%, 50%);\
             \n  step-3: hsl(300, 100%, 50%);\
             \n}\n"),
        "a {\
         \n  step-1: hsl(300, 20%, 50%);\
         \n  step-2: hsl(300, 60%, 50%);\
         \n  step-3: hsl(300, 100%, 50%);\
         \n}\n"
    );
}
#[test]
fn red() {
    assert_eq!(
        runner().ok("a {\
             \n  step-1: hsl(0, 20%, 50%);\
             \n  step-2: hsl(0, 60%, 50%);\
             \n  step-3: hsl(0, 100%, 50%);\
             \n}\n"),
        "a {\
         \n  step-1: hsl(0, 20%, 50%);\
         \n  step-2: hsl(0, 60%, 50%);\
         \n  step-3: hsl(0, 100%, 50%);\
         \n}\n"
    );
}
#[test]
fn yellow() {
    assert_eq!(
        runner().ok("a {\
             \n  step-1: hsl(60, 20%, 50%);\
             \n  step-2: hsl(60, 60%, 50%);\
             \n  step-3: hsl(60, 100%, 50%);\
             \n}\n"),
        "a {\
         \n  step-1: hsl(60, 20%, 50%);\
         \n  step-2: hsl(60, 60%, 50%);\
         \n  step-3: hsl(60, 100%, 50%);\
         \n}\n"
    );
}
