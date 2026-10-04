//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/hwb/rec2020.hrx"

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
             \na {b: color.to-space(hwb(10deg 20% 30% / 0.4), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.608871264 0.3689227048 0.2688397341 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg 20% 30% / 0.0), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.608871264 0.3689227048 0.2688397341 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(0deg 0% 100%), rec2020)}\n"),
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
             \na {b: color.to-space(hwb(20.123456789deg 30.987654321% 40.192837465%), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.5585194342 0.4525927717 0.3621149514);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(0deg 50% 50%), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.5260663507 0.5260663507 0.5260663507);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(80deg 20% 40%), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.5341965345 0.6105772488 0.3111867074);\
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
             \na {b: color.to-space(hwb(none none none), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 none none none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn blackness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg 20% none), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.8403632233 0.4573990543 0.296448064);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(none 20% 30%), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.5999445076 0.313446548 0.261273946);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn non_hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg none none), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.8277080064 0.368096751 0.1895507624);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn whiteness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(10deg none 30%), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.5929717335 0.2688294895 0.1369699874);\
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
             \na {b: color.to-space(hwb(20deg 999999% -999950%), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 9479.0719964917 9478.8972151984 9478.7377462887);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(20deg 200% -125%), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 2.1247745453 2.0374617526 1.9591919276);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(hwb(0deg 100% 0%), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 1 1 1);\
         \n}\n"
    );
}
