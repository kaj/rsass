//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklab/oklab.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklab")
}

#[test]
#[ignore] // unexepected error
fn in_range() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(50% 0.2 -0.3), oklab)}\n"),
        "a {\
         \n  b: oklab(50% 0.2 -0.3);\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn a() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% none 0.3), oklab)}\n"),
            "a {\
         \n  b: oklab(10% none 0.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 none), oklab)}\n"),
            "a {\
         \n  b: oklab(10% 0.2 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(none 0.2 0.3), oklab)}\n"),
            "a {\
         \n  b: oklab(none 0.2 0.3);\
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
             \na {b: color.to-space(color.change(oklab(0% -2 2), $lightness: -50%), oklab)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklab, color(xyz -7.6342505681 1.7017041167 -38.7847424763) 100%, black);\
         \n}\n"
    );
}
