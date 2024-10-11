//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklab/display_p3.hrx"

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
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.4), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.2245541298 -0.0715469579 -0.2325234172 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.0), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.2245541298 -0.0715469579 -0.2325234172 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(0% 0 0), display-p3)}\n"),
        "a {\
         \n  b: color(display-p3 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(50% 0 0), display-p3)}\n"),
        "a {\
         \n  b: color(display-p3 0.388572859 0.388572859 0.388572859);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(50% 0.2 -0.3), display-p3)}\n"),
        "a {\
         \n  b: color(display-p3 0.5192152895 -0.2770640538 0.9741962879);\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn a() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% none 0.3), display-p3)}\n"),
            "a {\
         \n  b: color(display-p3 0.0980260334 0.0512840295 -0.2115516357);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 none), display-p3)}\n"),
            "a {\
         \n  b: color(display-p3 0.1380887336 -0.0636630559 0.0058108369);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(none 0.2 0.3), display-p3)}\n"),
            "a {\
         \n  b: color(display-p3 0.0112774756 0.1430562716 -0.3528893207);\
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
             \na {b: color.to-space(oklab(50% -999999 0), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 -16964711.364205375 11051275.702622797 2566313.2368174535);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklab(0% -2 2), $lightness: -50%), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 -2.0082800862 2.5077632707 -4.7222178016);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(100% 0 0), display-p3)}\n"),
        "a {\
         \n  b: color(display-p3 1 1 1);\
         \n}\n"
    );
}
