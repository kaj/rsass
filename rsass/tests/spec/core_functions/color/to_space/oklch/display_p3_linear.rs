//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklch/display_p3_linear.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("display_p3_linear")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.4), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.008039958 -0.0017148347 -0.0003892457 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.0), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.008039958 -0.0017148347 -0.0003892457 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(0% 0 0deg), display-p3-linear)}\n"),
        "a {\
         \n  b: color(display-p3-linear 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(10.123456789% 0.198765432 30.192837465deg), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.020729313 -0.0064741161 -0.0005343322);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(50% 0 0deg), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.125 0.125 0.125);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.008039958 -0.0017148347 -0.0003892457);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn chroma() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% none 30deg), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.001 0.001 0.001);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 none), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0069979371 -0.0014866968 0.0007236304);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(none 0.1 30deg), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0002424859 0.000017232 -0.0005817775);\
         \n}\n"
    );
    }
}
mod out_of_range {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn far() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 999999 0deg), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 197291098750348672 -70531890535195928 -2120979045523177);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklch(0% 1 0deg), $lightness: -10%), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.1001201909 -0.0471611332 -0.0067005902);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(100% 0 0deg), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 1 1 1);\
         \n}\n"
    );
}
