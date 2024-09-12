//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/xyz/lch.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lch")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 0.2 0.3 / 0.4), lch)}\n"),
            "a {\
         \n  b: lch(51.373608379% 61.6452395223 193.1649108151deg / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 0.2 0.3 / 0.0), lch)}\n"),
            "a {\
         \n  b: lch(51.373608379% 61.6452395223 193.1649108151deg / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0 0 0), lch)}\n"),
        "a {\
         \n  b: lch(0% 0 none);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.5 0.5 0.5), lch)}\n"),
        "a {\
         \n  b: lch(76.1608841835% 8.541979634 32.621430668deg);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.2 0.4 0.8), lch)}\n"),
        "a {\
         \n  b: lch(68.6381340629% 86.9366558179 203.3717362956deg);\
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
             \na {b: color.to-space(color(xyz 0.1 0.2 none), lch)}\n"),
            "a {\
         \n  b: lch(51.9556818936% 99.6116642671 120.7127528375deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 none 0.3), lch)}\n"),
            "a {\
         \n  b: color-mix(in lch, color(xyz 0.1 0 0.3) 100%, black);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz none 0.2 0.3), lch)}\n"),
            "a {\
         \n  b: lch(51.0322781723% 262.6519587272 183.231207866deg);\
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
             \na {b: color.to-space(color(xyz -999999 0 0), lch)}\n"
        ),
        "a {\
         \n  b: color-mix(in lch, color(xyz -999998.9999993658 0.000000211 0.0000133413) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz -1 0.4 2), lch)}\n"),
            "a {\
         \n  b: lch(64.3546378926% 4878.0214227861 181.2519414622deg);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 1 1 1), lch)}\n"),
        "a {\
         \n  b: color-mix(in lch, color(xyz 1 1 1) 100%, black);\
         \n}\n"
    );
}
