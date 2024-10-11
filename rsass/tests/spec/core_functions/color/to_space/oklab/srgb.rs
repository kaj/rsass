//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklab/srgb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("srgb")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.4), srgb)}\n"),
            "a {\
         \n  b: color(srgb 0.2526613219 -0.0872393592 -0.2448119157 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.0), srgb)}\n"),
            "a {\
         \n  b: color(srgb 0.2526613219 -0.0872393592 -0.2448119157 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(0% 0 0), srgb)}\n"),
        "a {\
         \n  b: color(srgb 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(50% 0 0), srgb)}\n"),
        "a {\
         \n  b: color(srgb 0.388572859 0.388572859 0.388572859);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(50% 0.2 -0.3), srgb)}\n"),
        "a {\
         \n  b: color(srgb 0.5825328101 -0.3031010436 1.0153388554);\
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
             \na {b: color.to-space(oklab(10% none 0.3), srgb)}\n"),
            "a {\
         \n  b: color(srgb 0.106088871 0.0486334871 -0.2236185191);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 none), srgb)}\n"),
            "a {\
         \n  b: color(srgb 0.1600386899 -0.0720166235 0.0074363885);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(none 0.2 0.3), srgb)}\n"),
            "a {\
         \n  b: color(srgb -0.0386443168 0.1463213616 -0.3713891078);\
         \n}\n"
        );
    }
}
mod out_of_range {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn far() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklab(50% -999999 0), srgb)}\n"
        ),
        "a {\
         \n  b: color(srgb -18956885.022046793 11755005.08950827 1575235.545128226);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklab(0% -2 2), $lightness: -50%), srgb)}\n"
        ),
        "a {\
         \n  b: color(srgb -2.4568464522 2.5780112249 -4.9406967313);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(100% 0 0), srgb)}\n"),
        "a {\
         \n  b: color(srgb 1 1 1);\
         \n}\n"
    );
}
