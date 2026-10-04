//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/lab/rec2020.hrx"

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
             \na {b: color.to-space(lab(10% 20 30 / 0.4), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.2200718657 0.1213607074 -0.1382041281 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(10% 20 30 / 0.0), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.2200718657 0.1213607074 -0.1382041281 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(0% 0 0), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(50% 0 0), rec2020)}\n"),
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
             \na {b: color.to-space(lab(50% 50 -75), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.5510378055 0.4065454276 0.942259267);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn a() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(10% none 30), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.174024002 0.1530409515 -0.1395593739);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn all() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(none none none), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 none none none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(10% 20 none), rec2020)}\n"),
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
             \na {b: color.to-space(lab(none 20 30), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.1578653677 -0.0957295804 -0.1950138561);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn non_lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(10% none none), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.1542212427 0.1542212427 0.1542212427);\
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
             \na {b: color.to-space(lab(50% -999999 0), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 -12.2394863737 8.4850048383 -2.2719720142);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(lab(0% -150 150), $lightness: -50%), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 -0.3795747647 -0.2352130085 -0.4611984299);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(100% 0 0), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 1 1 1);\
         \n}\n"
    );
}
