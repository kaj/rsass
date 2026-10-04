//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/lch/rec2020.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rec2020")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 20 30deg / 0.4), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.2045875369 0.1279657857 0.1061716391 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 20 30deg / 0.0), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.2045875369 0.1279657857 0.1061716391 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(0% 0 0deg), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(lch(10.123456789% 20.987654321 30.192837465deg), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.2078221881 0.1272355998 0.1040078427);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(50% 0 0deg), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.4941484448 0.4941484448 0.4941484448);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 20 30deg), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.2045875369 0.1279657857 0.1061716391);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn all() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(none none none), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 none none none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn chroma() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% none 30deg), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.1542212427 0.1542212427 0.1542212427);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 20 none), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.2063393882 0.1229873357 0.1553854471);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(none 20 30deg), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.1359248656 -0.0888340784 -0.122773095);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn non_hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(none none 10deg), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0 0 0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn non_lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(50% none none), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.4941484448 0.4941484448 0.4941484448);\
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
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 999999 0deg), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 16221.0149979506 -11237.9817920794 3042.4784078444);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(lch(0% 200 0deg), $lightness: -10%), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.4572970831 -0.3473463713 -0.1345576522);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(100% 0 0deg), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 1 1 1);\
         \n}\n"
    );
}
