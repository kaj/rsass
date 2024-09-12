//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/hsl/srgb_linear.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("srgb_linear")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg 20% 30% / 0.4), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.106539224 0.0549717327 0.0469642007 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg 20% 30% / 0.0), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.106539224 0.0549717327 0.0469642007 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(0deg 0% 0%), srgb-linear)}\n"),
        "a {\
         \n  b: color(srgb-linear 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(hsl(20.123456789deg 30.987654321% 60.192837465%), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.4848385603 0.2752493311 0.1947438934);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(0deg 0% 50%), srgb-linear)}\n"),
        "a {\
         \n  b: color(srgb-linear 0.2140411405 0.2140411405 0.2140411405);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(80deg 30% 60%), srgb-linear)}\n"),
        "a {\
         \n  b: color(srgb-linear 0.3672464682 0.4769997846 0.1959941793);\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(none 20% 30%), srgb-linear)}\n"),
            "a {\
         \n  b: color(srgb-linear 0.106539224 0.0469642007 0.0469642007);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg 20% none), srgb-linear)}\n"),
            "a {\
         \n  b: color(srgb-linear 0 0 0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn saturation() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(10deg none 30%), srgb-linear)}\n"),
            "a {\
         \n  b: color(srgb-linear 0.0732389559 0.0732389559 0.0732389559);\
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
             \na {b: color.to-space(hsl(20deg 999999% 50%), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 663493625.4651376 -47462621.32578329 -663175228.1293004);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(20deg 200% -50%), srgb-linear)}\n"),
            "a {\
         \n  b: color(srgb-linear -2.5371552394 -0.0236523902 0.2140411405);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hsl(0deg 0% 100%), srgb-linear)}\n"),
        "a {\
         \n  b: color(srgb-linear 1 1 1);\
         \n}\n"
    );
}
