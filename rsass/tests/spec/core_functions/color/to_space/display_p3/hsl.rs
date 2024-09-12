//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/display_p3/hsl.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hsl")
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
             \na {b: color.to-space(color(display-p3 0.1 0.2 0.3 / 0.4), hsl)}\n"
        ),
        "a {\
         \n  b: hsla(205.417452683, 67.7488400949%, 18.4043245596%, 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 0.1 0.2 0.3 / 0.0), hsl)}\n"
        ),
        "a {\
         \n  b: hsla(205.417452683, 67.7488400949%, 18.4043245596%, 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 0 0 0), hsl)}\n"),
        "a {\
         \n  b: hsl(0, 0%, 0%);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 0.5 0.5 0.5), hsl)}\n"),
        "a {\
         \n  b: hsl(0, 0%, 50%);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 0.2 0.4 0.8), hsl)}\n"),
        "a {\
         \n  b: hsl(214.974668252, 77.667451385%, 46.5940983001%);\
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
             \na {b: color.to-space(color(display-p3 0.1 0.2 none), hsl)}\n"),
            "a {\
         \n  b: hsl(96.0435532608, 143.3480015017%, 8.3456369204%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 0.1 none 0.3), hsl)}\n"),
            "a {\
         \n  b: hsl(262.3902012265, 103.5328302701%, 15.4158592357%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 none 0.2 0.3), hsl)}\n"),
            "a {\
         \n  b: hsl(196.0636856013, 172.1527011916%, 11.3581215767%);\
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
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 -999999 0 0), hsl)}\n"),
            "a {\
         \n  b: hsl(356.7852726724, 165.043052964%, -41057989.10847678%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 -1 0.4 2), hsl)}\n"),
            "a {\
         \n  b: hsl(210.5153182162, 325.7395089334%, 48.9349840904%);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 1 1 1), hsl)}\n"),
        "a {\
         \n  b: hsl(0, 0%, 100%);\
         \n}\n"
    );
}
