//! Tests auto-converted from "sass-spec/spec/core_functions/meta/type_of.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("type_of")
}

#[test]
fn arglist() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@function type-of-arglist($args...) {\
             \n  @return meta.type-of($args);\
             \n}\n\
             \na {b: type-of-arglist()}\n"),
        "a {\
         \n  b: arglist;\
         \n}\n"
    );
}
mod boolean {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn test_false() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \na {b: meta.type-of(false)}\n"),
            "a {\
         \n  b: bool;\
         \n}\n"
        );
    }
    #[test]
    fn test_true() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \na {b: meta.type-of(true)}\n"),
            "a {\
         \n  b: bool;\
         \n}\n"
        );
    }
}
mod calculation {
    #[allow(unused)]
    use super::runner;

    mod preserved {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn calc() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \na {b: meta.type-of(calc(var(--c)))}\n"),
                "a {\
         \n  b: calculation;\
         \n}\n"
            );
        }
        #[test]
        fn clamp() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \na {b: meta.type-of(clamp(1%, 1px, 2px))}\n"),
                "a {\
         \n  b: calculation;\
         \n}\n"
            );
        }
    }
    #[test]
    fn simplified() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \na {b: meta.type-of(calc(1px))}\n"),
            "a {\
         \n  b: number;\
         \n}\n"
        );
    }
}
#[test]
fn color() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.type-of(red)}\n"),
        "a {\
         \n  b: color;\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \na {b: meta.type-of()}\n"
            ),
            "Error: Missing argument $value.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.type-of()}\
         \n  |       ^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function type-of($value) {\
         \n  |           =============== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \na {b: meta.type-of(1, 2)}\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.type-of(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function type-of($value) {\
         \n  |           =============== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn function() {
    assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \na {b: meta.type-of(meta.get-function(\"type-of\", $module: \"meta\"))}\n"
        ),
        "a {\
         \n  b: function;\
         \n}\n"
    );
}
mod list {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn empty() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \na {b: meta.type-of(())}\n"),
            "a {\
         \n  b: list;\
         \n}\n"
        );
    }
    #[test]
    fn non_empty() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \na {b: meta.type-of(1 2 3)}\n"),
            "a {\
         \n  b: list;\
         \n}\n"
        );
    }
}
mod map {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn empty() {
        assert_eq!(
            runner().ok("@use \"sass:map\";\
             \n@use \"sass:meta\";\
             \na {b: meta.type-of(map.remove((c: d), c))}\n"),
            "a {\
         \n  b: map;\
         \n}\n"
        );
    }
    #[test]
    fn non_empty() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \na {b: meta.type-of((c: d))}\n"),
            "a {\
         \n  b: map;\
         \n}\n"
        );
    }
}
mod mixin {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn builtin() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \na {b: meta.type-of(meta.get-mixin(load-css, meta))}\n"),
            "a {\
         \n  b: mixin;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn user_defined() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@mixin a() {}\
             \na {b: meta.type-of(meta.get-mixin(a))}\n"),
            "a {\
         \n  b: mixin;\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.type-of($value: c)}\n"),
        "a {\
         \n  b: string;\
         \n}\n"
    );
}
#[test]
fn null() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \na {b: meta.type-of(null)}\n"),
        "a {\
         \n  b: null;\
         \n}\n"
    );
}
mod number {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn unit() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \na {b: meta.type-of(1.5px * 3.4em)}\n"),
            "a {\
         \n  b: number;\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \na {b: meta.type-of(1)}\n"),
            "a {\
         \n  b: number;\
         \n}\n"
        );
    }
}
mod string {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn quoted() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \na {b: meta.type-of(\"c\")}\n"),
            "a {\
         \n  b: string;\
         \n}\n"
        );
    }
    #[test]
    fn unquoted() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \na {b: meta.type-of(c)}\n"),
            "a {\
         \n  b: string;\
         \n}\n"
        );
    }
}
