//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/xyz/srgb_linear.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("srgb_linear")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 0.2 0.3 / 0.4), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear -0.1329628694 0.2907356539 0.2818590705 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 0.2 0.3 / 0.0), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear -0.1329628694 0.2907356539 0.2818590705 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0 0 0), srgb-linear)}\n"),
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
             \na {b: color.to-space(color(xyz 0.5 0.5 0.5), srgb-linear)}\n"),
        "a {\
         \n  b: color(srgb-linear 0.602488002 0.4741394613 0.4543123175);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.2 0.4 0.8), srgb-linear)}\n"),
        "a {\
         \n  b: color(srgb-linear -0.3656478909 0.5897823193 0.7751124438);\
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
             \na {b: color.to-space(color(xyz 0.1 0.2 none), srgb-linear)}\n"
            ),
            "a {\
         \n  b: color(srgb-linear 0.0166203587 0.2782691367 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 none 0.3), srgb-linear)}\n"
            ),
            "a {\
         \n  b: color(srgb-linear 0.1745137661 none 0.3226544622);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz none 0.2 0.3), srgb-linear)}\n"
            ),
            "a {\
         \n  b: color(srgb-linear none 0.3876600175 0.2762960625);\
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
             \na {b: color.to-space(color(xyz -999999 0 0), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear -3240966.700934579 969242.6670372436 -55630.0240669139);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz -1 0.4 2), srgb-linear)}\n"),
            "a {\
         \n  b: color(srgb-linear -4.8531447335 1.8027407517 1.9767221652);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 1 1 1), srgb-linear)}\n"),
        "a {\
         \n  b: color(srgb-linear 1.204976004 0.9482789226 0.9086246351);\
         \n}\n"
    );
}
