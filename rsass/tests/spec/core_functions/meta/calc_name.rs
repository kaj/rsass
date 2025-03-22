//! Tests auto-converted from "sass-spec/spec/core_functions/meta/calc_name.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("calc_name")
}

#[test]
fn calc() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.calc-name(calc(var(--c)))}\n"),
        "a {\
         \n  b: \"calc\";\
         \n}\n"
    );
}
#[test]
fn clamp() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.calc-name(clamp(1%, 2px, 3px))}\n"),
        "a {\
         \n  b: \"clamp\";\
         \n}\n"
    );
}
mod error {
    use super::runner;

    #[test]
    fn invalid_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \na {b: meta.calc-name(1)}\n"
            ),
            "Error: $calc: 1 is not a calculation.\
         \n  ,\
         \n2 | a {b: meta.calc-name(1)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \na {b: meta.calc-name()}\n"
            ),
            "Error: Missing argument $calc.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.calc-name()}\
         \n  |       ^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function calc-name($calc) {\
         \n  |           ================ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
        runner().err(
            "@use \"sass:meta\";\
             \na {b: meta.calc-name(calc(var(--c)), calc(var(--d)))}\n"
        ),
        "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.calc-name(calc(var(--c)), calc(var(--d)))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function calc-name($calc) {\
         \n  |           ================ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
}
#[test]
fn max() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.calc-name(max(var(--c)))}\n"),
        "a {\
         \n  b: \"max\";\
         \n}\n"
    );
}
#[test]
fn min() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.calc-name(min(var(--c)))}\n"),
        "a {\
         \n  b: \"min\";\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.calc-name($calc: calc(var(--c)))}\n"),
        "a {\
         \n  b: \"calc\";\
         \n}\n"
    );
}
