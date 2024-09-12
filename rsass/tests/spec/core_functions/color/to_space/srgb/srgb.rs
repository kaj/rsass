//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/srgb/srgb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("srgb")
}

#[test]
#[ignore] // unexepected error
fn in_range() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.2 0.4 0.8), srgb)}\n"),
        "a {\
         \n  b: color(srgb 0.2 0.4 0.8);\
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
             \na {b: color.to-space(color(srgb 0.1 0.2 none), srgb)}\n"),
            "a {\
         \n  b: color(srgb 0.1 0.2 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.1 none 0.3), srgb)}\n"),
            "a {\
         \n  b: color(srgb 0.1 none 0.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb none 0.2 0.3), srgb)}\n"),
            "a {\
         \n  b: color(srgb none 0.2 0.3);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn out_of_range() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb -1 0.4 2), srgb)}\n"),
        "a {\
         \n  b: color(srgb -1 0.4 2);\
         \n}\n"
    );
}
