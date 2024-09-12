//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklch/lab.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lab")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.4), lab)}\n"),
            "a {\
         \n  b: lab(0.6560445641% 11.6264793014 1.8588166268 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.0), lab)}\n"),
            "a {\
         \n  b: lab(0.6560445641% 11.6264793014 1.8588166268 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(0% 0 0deg), lab)}\n"),
        "a {\
         \n  b: lab(0% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(10.123456789% 0.198765432 30.192837465deg), lab)}\n"
        ),
        "a {\
         \n  b: lab(0.4361236096% 33.2605943231 2.095776706);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(50% 0 0deg), lab)}\n"),
        "a {\
         \n  b: lab(42.0000002803% 0.0000000582 -0.0000070926);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg), lab)}\n"),
        "a {\
         \n  b: lab(0.6560445641% 11.6264793014 1.8588166268);\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn chroma() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% none 30deg), lab)}\n"),
            "a {\
         \n  b: lab(0.9032963094% 0.0000000027 -0.0000003314);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 none), lab)}\n"),
            "a {\
         \n  b: lab(0.6385915087% 10.5093876004 0.1611873791);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(none 0.1 30deg), lab)}\n"),
            "a {\
         \n  b: lab(none 0.0322037834 0.9096922188);\
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
             \na {b: color.to-space(oklch(10% 999999 0deg), lab)}\n"
        ),
        "a {\
         \n  b: color-mix(in lab, color(xyz 76838084844227696 -3783161942592645 -5396112427214629) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklch(0% 1 0deg), $lightness: -10%), lab)}\n"
        ),
        "a {\
         \n  b: color-mix(in lab, color(xyz 0.0348582183 -0.010229465 -0.0091226442) 100%, black);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(100% 0 0deg), lab)}\n"
        ),
        "a {\
         \n  b: color-mix(in lab, color(xyz 0.9504559526 1.0000000182 1.0890580001) 100%, black);\
         \n}\n"
    );
}
