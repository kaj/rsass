//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklch/lch.hrx"

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
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.4), lch)}\n"),
            "a {\
         \n  b: lch(0.6560445641% 11.7741335222 9.0834533485deg / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.0), lch)}\n"),
            "a {\
         \n  b: lch(0.6560445641% 11.7741335222 9.0834533485deg / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(0% 0 0deg), lch)}\n"),
        "a {\
         \n  b: lch(0% 0 none);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(10.123456789% 0.198765432 30.192837465deg), lch)}\n"
        ),
        "a {\
         \n  b: lch(0.4361236096% 33.3265571988 3.6054863359deg);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(50% 0 0deg), lch)}\n"),
        "a {\
         \n  b: lch(42.0000002803% 0.0000070929 270.4699000403deg);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg), lch)}\n"),
        "a {\
         \n  b: lch(0.6560445641% 11.7741335222 9.0834533485deg);\
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
             \na {b: color.to-space(oklch(10% none 30deg), lch)}\n"),
            "a {\
         \n  b: lch(0.9032963094% none 270.4698988243deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 none), lch)}\n"),
            "a {\
         \n  b: lch(0.6385915087% 10.5106236307 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(none 0.1 30deg), lch)}\n"),
            "a {\
         \n  b: lch(none 0.9102620593 87.9725333167deg);\
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
             \na {b: color.to-space(oklch(10% 999999 0deg), lch)}\n"
        ),
        "a {\
         \n  b: color-mix(in lch, color(xyz 76837027122572336 -3783130536950956 -5396126058991186) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklch(0% 1 0deg), $lightness: -10%), lch)}\n"
        ),
        "a {\
         \n  b: color-mix(in lch, color(xyz 0.0348582183 -0.010229465 -0.0091226442) 100%, black);\
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
             \na {b: color.to-space(oklch(100% 0 0deg), lch)}\n"
        ),
        "a {\
         \n  b: color-mix(in lch, color(xyz 0.9504559526 1.0000000182 1.0890580001) 100%, black);\
         \n}\n"
    );
}
