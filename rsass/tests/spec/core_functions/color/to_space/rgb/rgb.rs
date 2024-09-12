//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rgb/rgb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rgb")
}

#[test]
#[ignore] // unexepected error
fn in_range() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#28d, rgb)}\n"),
        "a {\
         \n  b: #28d;\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 20 none), rgb)}\n"),
            "a {\
         \n  b: rgb(10 20 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 none 30), rgb)}\n"),
            "a {\
         \n  b: rgb(10 none 30);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(none 20 30), rgb)}\n"),
            "a {\
         \n  b: rgb(none 20 30);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn out_of_range() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(rgb(0, 100, 0), $red: -50, $blue: 400), rgb)}\n"
        ),
        "a {\
         \n  b: hsl(220, 281.25%, 68.6274509804%);\
         \n}\n"
    );
}
