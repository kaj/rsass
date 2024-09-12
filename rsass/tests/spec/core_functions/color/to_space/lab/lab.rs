//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/lab/lab.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lab")
}

#[test]
#[ignore] // unexepected error
fn in_range() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(50% 50 -75), lab)}\n"),
        "a {\
         \n  b: lab(50% 50 -75);\
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
             \na {b: color.to-space(lab(10% none 30), lab)}\n"),
            "a {\
         \n  b: lab(10% none 30);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(10% 20 none), lab)}\n"),
            "a {\
         \n  b: lab(10% 20 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(none 20 30), lab)}\n"),
            "a {\
         \n  b: lab(none 20 30);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn out_of_range() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(50% -150 150), lab)}\n"),
        "a {\
         \n  b: lab(50% -150 150);\
         \n}\n"
    );
}
