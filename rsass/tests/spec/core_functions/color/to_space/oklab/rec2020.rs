//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklab/rec2020.hrx"

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
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.4), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.2247165405 -0.1048905172 -0.2710745546 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.0), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.2247165405 -0.1048905172 -0.2710745546 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(0% 0 0), rec2020)}\n"),
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
             \na {b: color.to-space(oklab(50% 0 0), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.4204482076 0.4204482076 0.4204482076);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(50% 0.2 -0.3), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.5193268126 -0.251384226 0.9682456661);\
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
             \na {b: color.to-space(oklab(10% none 0.3), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.1217144828 0.0980967097 -0.2507496386);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn all() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(none none none), rec2020)}\n"),
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
             \na {b: color.to-space(oklab(10% 0.2 none), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.1571221358 -0.1021456253 0.0353819172);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(none 0.2 0.3), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 -0.0460583372 0.1774297951 -0.3834672783);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn non_lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% none none), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.0562341325 0.0562341325 0.0562341325);\
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
             \na {b: color.to-space(oklab(50% -999999 0), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 -13712704.330516009 9615330.225098789 3020647.9352328237);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklab(0% -2 2), $lightness: -50%), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 -1.760053313 2.2800532201 -4.489254188);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(100% 0 0), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 1 1 1);\
         \n}\n"
    );
}
