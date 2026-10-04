//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rec2020/oklch.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklch")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.4), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(26.0943666826% 0.0867230722 229.6913990521deg / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.0), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(26.0943666826% 0.0867230722 229.6913990521deg / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0 0 0), oklch)}\n"),
        "a {\
         \n  b: oklch(0% 0 none);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.5 0.5 0.5), oklch)}\n"),
        "a {\
         \n  b: oklch(57.4349177499% 0 none);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.2 0.4 0.8), oklch)}\n"),
        "a {\
         \n  b: oklch(48.8307497788% 0.2244413094 243.1644318247deg);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn all() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 none none none), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(none none none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 none), oklch)}\n"),
            "a {\
         \n  b: oklch(23.7844679127% 0.1079431282 145.7192580256deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 none 0.3), oklch)}\n"),
            "a {\
         \n  b: oklch(17.9852042001% 0.1240104273 272.4074791641deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 none 0.2 0.3), oklch)}\n"),
            "a {\
         \n  b: oklch(25.3377299665% 0.1056493997 221.157564445deg);\
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
             \na {b: color.to-space(color(rec2020 -999999 0 0), oklch)}\n"),
            "a {\
         \n  b: color-mix(in oklch, color(xyz -159996243905245.12 -65987151441146.27 -0.0078125) 100%, black);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 -1 0.4 2), oklch)}\n"),
            "a {\
         \n  b: oklch(37.9216267643% 2.6072646459 196.0076138199deg);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 1 1 1), oklch)}\n"),
        "a {\
         \n  b: oklch(100% 0 none);\
         \n}\n"
    );
}
