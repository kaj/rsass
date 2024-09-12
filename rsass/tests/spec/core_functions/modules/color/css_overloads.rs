//! Tests auto-converted from "sass-spec/spec/core_functions/modules/color/css_overloads.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("css_overloads")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn multi_arg() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.alpha(c=d, e=f, g=h)}\n"),
            "a {\
         \n  b: alpha(c=d, e=f, g=h);\
         \n}\n"
        );
    }
    #[test]
    fn one_arg() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.alpha(c=d)}\n"),
            "a {\
         \n  b: alpha(c=d);\
         \n}\n"
        );
    }
}
#[test]
fn grayscale() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.grayscale(1)}\n"),
        "a {\
         \n  b: grayscale(1);\
         \n}\n"
    );
}
#[test]
fn invert() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.invert(1)}\n"),
        "a {\
         \n  b: invert(1);\
         \n}\n"
    );
}
#[test]
fn opacity() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.opacity(1)}\n"),
        "a {\
         \n  b: opacity(1);\
         \n}\n"
    );
}
