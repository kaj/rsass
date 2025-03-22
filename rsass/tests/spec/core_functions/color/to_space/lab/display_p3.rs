//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/lab/display_p3.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("display_p3")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(lab(10% 20 30 / 0.4), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.2036678394 0.0627355423 -0.092163728 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(10% 20 30 / 0.0), display-p3)}\n"),
            "a {\
         \n  b: color(display-p3 0.2036678394 0.0627355423 -0.092163728 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(0% 0 0), display-p3)}\n"),
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
             \na {b: color.to-space(lab(50% 0 0), display-p3)}\n"),
        "a {\
         \n  b: color(display-p3 0.4663266093 0.4663266093 0.4663266093);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(50% 50 -75), display-p3)}\n"),
        "a {\
         \n  b: color(display-p3 0.5227133162 0.348060966 0.9452485417);\
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
             \na {b: color.to-space(lab(10% none 30), display-p3)}\n"),
            "a {\
         \n  b: color(display-p3 0.1411767229 0.1061424169 -0.0944472015);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(10% 20 none), display-p3)}\n"),
            "a {\
         \n  b: color(display-p3 0.1823778479 0.0651398083 0.1096920557);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(none 20 30), display-p3)}\n"),
            "a {\
         \n  b: color(display-p3 0.1438618115 -0.0552726461 -0.1517242634);\
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
             \na {b: color.to-space(lab(50% -999999 0), display-p3)}\n"),
            "a {\
         \n  b: color(display-p3 -15.0614491889 9.7148006078 -2.8909250239);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(lab(0% -150 150), $lightness: -50%), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 -0.3704020725 -0.1731154433 -0.4345219765);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lab(100% 0 0), display-p3)}\n"),
        "a {\
         \n  b: color(display-p3 1 1 1);\
         \n}\n"
    );
}
