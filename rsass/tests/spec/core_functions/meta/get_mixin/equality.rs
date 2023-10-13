//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_mixin/equality.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("equality")
}

mod built_in {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn different() {
        assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \na {b: meta.get-mixin(load-css, meta) == meta.get-mixin(apply, meta)}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn same() {
        assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \na {b: meta.get-mixin(load-css, meta) == meta.get-mixin(load-css, meta)}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn same_value() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@mixin a() {}\
             \n$a: meta.get-mixin(a);\
             \na {b: $a == $a}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
mod user_defined {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn different() {
        assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@mixin user-defined-1() {}\
             \n@mixin user-defined-2() {}\
             \na {b: meta.get-mixin(user-defined-1) == meta.get-mixin(user-defined-2)}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn redefined() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@mixin user-defined() {}\
             \n$first-reference: meta.get-mixin(user-defined);\n\
             \n@mixin user-defined() {}\
             \n$second-reference: meta.get-mixin(user-defined);\
             \na {b: $first-reference == $second-reference}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn same() {
        assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@mixin user-defined() {}\
             \na {b: meta.get-mixin(user-defined) == meta.get-mixin(user-defined)}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
}
