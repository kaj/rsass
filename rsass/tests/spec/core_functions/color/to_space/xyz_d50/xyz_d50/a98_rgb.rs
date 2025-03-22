//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/xyz_d50/xyz-d50/a98_rgb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("a98_rgb")
}

#[test]
#[ignore] // unexepected error
fn in_range() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.2 0.4 0.8), xyz-d50)}\n"),
        "a {\
         \n  b: color(xyz-d50 0.2 0.4 0.8);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.1 0.2 none), xyz-d50)}\n"
            ),
            "a {\
         \n  b: color(xyz-d50 0.1 0.2 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.1 none 0.3), xyz-d50)}\n"
            ),
            "a {\
         \n  b: color(xyz-d50 0.1 none 0.3);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 none 0.2 0.3), xyz-d50)}\n"
            ),
            "a {\
         \n  b: color(xyz-d50 none 0.2 0.3);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn out_of_range() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 -1 0.4 2), xyz-d50)}\n"),
        "a {\
         \n  b: color(xyz-d50 -1 0.4 2);\
         \n}\n"
    );
}
