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
         \n  b: color(display-p3 0.2245541345 -0.0715469676 -0.2325234092 / 0.4);\
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
         \n  b: color(display-p3 0.2245541345 -0.0715469676 -0.2325234092 / 0);\
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
         \n  b: color(display-p3 0.3885728491 0.3885728621 0.3885729031);\
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
         \n  b: color(display-p3 0.5192153014 -0.2770640957 0.9741963252);\
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
         \n  b: color(display-p3 0.0980260328 0.0512840259 -0.2115516265);\
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
         \n  b: color(display-p3 0.1380887377 -0.0636630597 0.0058108376);\
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
         \n  b: color(display-p3 0.011277471 0.1430562743 -0.3528893187);\
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
         \n  b: color(display-p3 -16964712.17081906 11051276.299069608 2566313.802188239);\
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
         \n  b: color(display-p3 -2.0082801891 2.5077633347 -4.7222178033);\
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
         \n  b: color(display-p3 0.9999999764 1.0000000074 1.0000001047);\
         \n}\n"
    );
}
