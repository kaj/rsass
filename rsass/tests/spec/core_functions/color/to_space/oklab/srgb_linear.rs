//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklab/srgb_linear.hrx"

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
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.4), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.0519480274 -0.0081553309 -0.0488237835 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.0), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.0519480274 -0.0081553309 -0.0488237835 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(0% 0 0), srgb-linear)}\n"),
        "a {\
         \n  b: color(srgb-linear 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(50% 0 0), srgb-linear)}\n"),
        "a {\
         \n  b: color(srgb-linear 0.125 0.125 0.125);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(50% 0.2 -0.3), srgb-linear)}\n"),
        "a {\
         \n  b: color(srgb-linear 0.298538395 -0.0747837958 1.035249898);\
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
             \na {b: color.to-space(oklab(10% none 0.3), srgb-linear)}\n"),
            "a {\
         \n  b: color(srgb-linear 0.01099389 0.0038141201 -0.040946619);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 none), srgb-linear)}\n"),
            "a {\
         \n  b: color(srgb-linear 0.0219904416 -0.0062152621 0.0005755719);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(none 0.2 0.3), srgb-linear)}\n"),
            "a {\
         \n  b: color(srgb-linear -0.0029910462 0.0187728305 -0.1136916951);\
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
             \na {b: color.to-space(oklab(50% -999999 0), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear -257534159491122656 81795398641748704 657390374199289.2);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklab(0% -2 2), $lightness: -50%), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear -8.0200579685 8.9800882009 -41.7661703829);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(100% 0 0), srgb-linear)}\n"),
        "a {\
         \n  b: color(srgb-linear 1 1 1);\
         \n}\n"
    );
}
