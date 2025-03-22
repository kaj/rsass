//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/lch/rgb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rgb")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 20 30deg / 0.4), rgb)}\n"),
            "a {\
         \n  b: rgba(50.3195306005, 16.4012435608, 12.9749871225, 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 20 30deg / 0.0), rgb)}\n"),
            "a {\
         \n  b: rgba(50.3195306005, 16.4012435608, 12.9749871225, 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(0% 0 0deg), rgb)}\n"),
        "a {\
         \n  b: black;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(lch(10.123456789% 20.987654321 30.192837465deg), rgb)}\n"
        ),
        "a {\
         \n  b: rgb(51.5386378704, 15.8847345936, 12.2512406149);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(50% 0 0deg), rgb)}\n"),
        "a {\
         \n  b: rgb(118.9132853673, 118.9132853673, 118.9132853673);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 20 30deg), rgb)}\n"),
        "a {\
         \n  b: rgb(50.3195306005, 16.4012435608, 12.9749871225);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn chroma() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% none 30deg), rgb)}\n"),
            "a {\
         \n  b: rgb(27.4643698292, 27.4643698292, 27.4643698292);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% 20 none), rgb)}\n"),
            "a {\
         \n  b: rgb(50.8524583137, 14.2717063113, 28.2183712056);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(none 20 30deg), rgb)}\n"),
            "a {\
         \n  b: hsl(6.9848409854, 394.5339053958%, 2.7008748146%);\
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
             \na {b: color.to-space(lch(10% 999999 0deg), rgb)}\n"),
            "a {\
         \n  b: hsl(149.4283545837, 420.5938588221%, 429851.5077692641%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(lch(0% 200 0deg), $lightness: -10%), rgb)}\n"
        ),
        "a {\
         \n  b: hsl(340.1543058221, 427.9584468502%, 11.074503568%);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(100% 0 0deg), rgb)}\n"),
        "a {\
         \n  b: white;\
         \n}\n"
    );
}
