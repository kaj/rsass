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
         \n  b: color(srgb 0.2526613275 -0.0872393682 -0.2448119073 / 0.4);\
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
         \n  b: color(srgb 0.2526613275 -0.0872393682 -0.2448119073 / 0);\
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
         \n  b: color(srgb 0.3885728462 0.3885728627 0.3885729073);\
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
         \n  b: color(srgb 0.5825328265 -0.3031010838 1.0153388947);\
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
         \n  b: color(srgb 0.1060888708 0.0486334832 -0.2236185095);\
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
         \n  b: color(srgb 0.1600386946 -0.0720166274 0.0074363895);\
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
         \n  b: color(srgb -0.0386443242 0.1463213645 -0.3713891058);\
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
         \n  b: color(srgb -18956885.930884026 11755005.716275353 1575236.4989373833);\
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
         \n  b: color(srgb -2.4568465628 2.5780112923 -4.9406967346);\
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
         \n  b: color(srgb 0.9999999694 1.0000000087 1.0000001149);\
         \n}\n"
    );
}
