//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/srgb/display_p3.hrx"

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
             \na {b: color.to-space(color(srgb 0.1 0.2 0.3 / 0.4), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.1237968384 0.1975241128 0.2918191239 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.1 0.2 0.3 / 0.0), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.1237968384 0.1975241128 0.2918191239 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0 0 0), display-p3)}\n"),
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
             \na {b: color.to-space(color(srgb 0.5 0.5 0.5), display-p3)}\n"),
        "a {\
         \n  b: color(display-p3 0.5 0.5 0.5);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.2 0.4 0.8), display-p3)}\n"),
        "a {\
         \n  b: color(display-p3 0.2498513331 0.3952400722 0.77356175);\
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
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.1 0.2 none), display-p3)}\n"
            ),
            "a {\
         \n  b: color(display-p3 0.1237968384 0.1975241128 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.1 none 0.3), display-p3)}\n"
            ),
            "a {\
         \n  b: color(display-p3 0.0878773188 none 0.2867666162);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb none 0.2 0.3), display-p3)}\n"
            ),
            "a {\
         \n  b: color(display-p3 none 0.196438359 0.2914615969);\
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
             \na {b: color.to-space(color(srgb -999999 0 0), display-p3)}\n"
        ),
        "a {\
         \n  b: color(display-p3 -921788.227771966 -241977.733146743 -183469.5263235596);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb -1 0.4 2), display-p3)}\n"),
            "a {\
         \n  b: color(display-p3 -0.9057671336 0.3411005959 1.9199196788);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 1 1 1), display-p3)}\n"),
        "a {\
         \n  b: color(display-p3 1 1 1);\
         \n}\n"
    );
}
