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
         \n  b: rgba(19.7331492551%, 6.4318602199%, 5.0882302441%, 0.4);\
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
         \n  b: rgba(19.7331492551%, 6.4318602199%, 5.0882302441%, 0);\
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
         \n  b: rgb(20.2112305374%, 6.2293076838%, 4.8044080843%);\
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
         \n  b: rgb(46.6326609284%, 46.6326609284%, 46.6326609284%);\
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
         \n  b: rgb(19.7331492551%, 6.4318602199%, 5.0882302441%);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn all() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(none none none), rgb)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn chroma() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(10% none 30deg), rgb)}\n"),
            "a {\
         \n  b: rgb(10.7703411095%, 10.7703411095%, 10.7703411095%);\
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
         \n  b: rgb(19.9421405152%, 5.5967475731%, 11.0660279238%);\
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
    #[test]
    #[ignore] // unexepected error
    fn non_hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(none none 10deg), rgb)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn non_lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(lch(50% none none), rgb)}\n"),
            "a {\
         \n  b: rgb(46.6326609284%, 46.6326609284%, 46.6326609284%);\
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
