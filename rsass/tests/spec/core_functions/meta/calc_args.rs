//! Tests auto-converted from "sass-spec/spec/core_functions/meta/calc_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("calc_args")
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn invalid_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \na {b: meta.calc-args(1)}\n"
            ),
            "Error: $calc: 1 is not a calculation.\
         \n  ,\
         \n2 | a {b: meta.calc-args(1)}\
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
             \na {b: meta.calc-args()}\n"
            ),
            "Error: Missing argument $calc.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.calc-args()}\
         \n  |       ^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function calc-args($calc) {\
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
             \na {b: meta.calc-args(calc(var(--c)), calc(var(--d)))}\n"
        ),
        "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.calc-args(calc(var(--c)), calc(var(--d)))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function calc-args($calc) {\
         \n  |           ================ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
}
mod multi_args {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn first() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \na {b: list.nth(meta.calc-args(clamp(1%, 2px, 3px)), 1)}\n"),
            "a {\
         \n  b: 1%;\
         \n}\n"
        );
    }
    #[test]
    fn length() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \na {b: list.length(meta.calc-args(clamp(1%, 2px, 3px)))}\n"),
            "a {\
         \n  b: 3;\
         \n}\n"
        );
    }
    #[test]
    fn second() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \na {b: list.nth(meta.calc-args(clamp(1%, 2px, 3px)), 2)}\n"),
            "a {\
         \n  b: 2px;\
         \n}\n"
        );
    }
    #[test]
    fn third() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \na {b: list.nth(meta.calc-args(clamp(1%, 2px, 3px)), 3)}\n"),
            "a {\
         \n  b: 3px;\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.calc-args($calc: calc(var(--c)))}\n"),
        "a {\
         \n  b: var(--c);\
         \n}\n"
    );
}
mod one_arg {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn first() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \na {b: list.nth(meta.calc-args(calc(var(--c))), 1)}\n"),
            "a {\
         \n  b: var(--c);\
         \n}\n"
        );
    }
    #[test]
    fn length() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \na {b: list.length(meta.calc-args(calc(var(--c))))}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
}
mod test_type {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn calculation() {
        assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \na {b: meta.type-of(list.nth(meta.calc-args(min(max(1%, 1px), 2px)), 1))}\n"
        ),
        "a {\
         \n  b: calculation;\
         \n}\n"
    );
    }
    #[test]
    fn css_function() {
        assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \na {b: meta.type-of(list.nth(meta.calc-args(calc(var(--c))), 1))}\n"
        ),
        "a {\
         \n  b: string;\
         \n}\n"
    );
    }
    #[test]
    fn interpolation() {
        assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \na {b: meta.type-of(list.nth(meta.calc-args(calc(#{1px})), 1))}\n"
        ),
        "a {\
         \n  b: string;\
         \n}\n"
    );
    }
    #[test]
    fn math() {
        assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \na {b: meta.type-of(list.nth(meta.calc-args(calc(1% + 1px)), 1))}\n"
        ),
        "a {\
         \n  b: string;\
         \n}\n"
    );
    }
    #[test]
    fn number() {
        assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \na {b: meta.type-of(list.nth(meta.calc-args(min(1%, 2px)), 1))}\n"
        ),
        "a {\
         \n  b: number;\
         \n}\n"
    );
    }
    #[test]
    fn string_variable() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n$a: b;\
             \nc {d: meta.type-of(list.nth(meta.calc-args(calc($a)), 1))}\n"),
            "c {\
         \n  d: string;\
         \n}\n"
        );
    }
}
