//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rgb/rec2020.hrx"

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
             \na {b: color.to-space(rgb(10 20 30 / 0.4), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.1078255025 0.124912903 0.1599645978 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 20 30 / 0.0), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.1078255025 0.124912903 0.1599645978 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#000, rec2020)}\n"),
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
             \na {b: color.to-space(rgb(50.123456789 100.987654321 200.192837465), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.3632738111 0.4252117758 0.7677900175);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#aaa, rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.6840442338 0.6840442338 0.6840442338);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#28d, rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.4168157247 0.5476373982 0.8460479497);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 20 none), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.1023348096 0.1237748652 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 none 30), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.0819181198 none 0.1565762659);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(none 20 30), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 none 0.1232905084 0.1596945236);\
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
             \na {b: color.to-space(color.change(black, $red: -999999), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 -3060.9602363886 -1220.8297511778 -670.3528590291);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(rgb(0, 100, 0), $red: -50, $blue: 400), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.4457820611 0.4497749835 1.4724705584);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#fff, rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 1 1 1);\
         \n}\n"
    );
}
