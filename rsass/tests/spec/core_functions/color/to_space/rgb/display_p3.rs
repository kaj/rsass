//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rgb/display_p3.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("display_p3")
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
             \na {b: color.to-space(rgb(10 20 30 / 0.4), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.0477705982 0.0773808537 0.1142571507 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 20 30 / 0.0), display-p3)}\n"),
            "a {\
         \n  b: color(display-p3 0.0477705982 0.0773808537 0.1142571507 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#000, display-p3)}\n"),
        "a {\
         \n  b: color(display-p3 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(rgb(50.123456789 100.987654321 200.192837465), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.2464159945 0.3912935228 0.7592218197);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#aaa, display-p3)}\n"),
        "a {\
         \n  b: color(display-p3 0.6666666667 0.6666666667 0.6666666667);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#28d, display-p3)}\n"),
        "a {\
         \n  b: color(display-p3 0.2644791221 0.525654809 0.8414340148);\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 20 none), display-p3)}\n"),
            "a {\
         \n  b: color(display-p3 0.0477705982 0.0773808537 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 none 30), display-p3)}\n"),
            "a {\
         \n  b: color(display-p3 0.0322534105 none 0.1113368327);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(none 20 30), display-p3)}\n"),
            "a {\
         \n  b: color(display-p3 none 0.0765677073 0.1139614092);\
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
             \na {b: color.to-space(color.change(black, $red: -999999), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 -3614.8515104566 -948.8907591358 -719.4436054746);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(rgb(0, 100, 0), $red: -50, $blue: 400), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 -0.0462648785 0.3843286419 1.5086360967);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#fff, display-p3)}\n"),
        "a {\
         \n  b: color(display-p3 1 1 1);\
         \n}\n"
    );
}
