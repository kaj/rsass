//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rec2020/a98_rgb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("a98_rgb")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.4), a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb -0.0313075222 0.1793609893 0.2750277026 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.0), a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb -0.0313075222 0.1793609893 0.2750277026 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0 0 0), a98-rgb)}\n"),
        "a {\
         \n  b: color(a98-rgb 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.5 0.5 0.5), a98-rgb)}\n"),
        "a {\
         \n  b: color(a98-rgb 0.4693393652 0.4693393652 0.4693393652);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.2 0.4 0.8), a98-rgb)}\n"),
        "a {\
         \n  b: color(a98-rgb -0.1629359564 0.3786245371 0.8056352345);\
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
             \na {b: color.to-space(color(rec2020 0.1 0.2 none), a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.0660319295 0.1810092824 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 none 0.3), a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.0528853413 none 0.2772545754);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 none 0.2 0.3), a98-rgb)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb none 0.181120821 0.2752194056);\
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
             \na {b: color.to-space(color(rec2020 -999999 0 0), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb -3764596.565534183 1369072.8495067134 629175.4025038923);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 -1 0.4 2), a98-rgb)}\n"),
            "a {\
         \n  b: color(a98-rgb -1.1841806398 0.4876740568 2.2024057679);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 1 1 1), a98-rgb)}\n"),
        "a {\
         \n  b: color(a98-rgb 1 1 1);\
         \n}\n"
    );
}
