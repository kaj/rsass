//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/xyz/xyz.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("xyz")
}

#[test]
#[ignore] // unexepected error
fn in_range() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.2 0.4 0.8), xyz)}\n"),
        "a {\
         \n  b: color(xyz 0.2 0.4 0.8);\
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
             \na {b: color.to-space(color(xyz 0.1 0.2 none), xyz)}\n"),
            "a {\
         \n  b: color(xyz 0.1 0.2 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 none 0.3), xyz)}\n"),
            "a {\
         \n  b: color(xyz 0.1 none 0.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz none 0.2 0.3), xyz)}\n"),
            "a {\
         \n  b: color(xyz none 0.2 0.3);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn out_of_range() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz -1 0.4 2), xyz)}\n"),
        "a {\
         \n  b: color(xyz -1 0.4 2);\
         \n}\n"
    );
}
