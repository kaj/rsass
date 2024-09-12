//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklch/hsl.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hsl")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.4), hsl)}\n"),
            "a {\
         \n  b: hsla(349.9222383971, 174.3087628098%, 3.6948783654%, 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.0), hsl)}\n"),
            "a {\
         \n  b: hsla(349.9222383971, 174.3087628098%, 3.6948783654%, 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(0% 0 0deg), hsl)}\n"),
        "a {\
         \n  b: hsl(0, 0%, 0%);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(10.123456789% 0.198765432 30.192837465deg), hsl)}\n"
        ),
        "a {\
         \n  b: hsl(342.3640346721, 274.4922188663%, 4.7714615434%);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(50% 0 0deg), hsl)}\n"),
        "a {\
         \n  b: hsl(223.8135972091, 0.0000078676%, 38.8572876766%);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg), hsl)}\n"),
        "a {\
         \n  b: hsl(349.9222383971, 174.3087628098%, 3.6948783654%);\
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
             \na {b: color.to-space(oklch(10% none 30deg), hsl)}\n"),
            "a {\
         \n  b: hsl(223.8135982915, 0%, 1.2920001239%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 none), hsl)}\n"),
            "a {\
         \n  b: hsl(0, 169.3005035983%, 3.4369836375%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(none 0.1 30deg), hsl)}\n"),
            "a {\
         \n  b: hsl(221.7487183642, 266.6061391496%, 0%);\
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
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 999999 0deg), hsl)}\n"),
            "a {\
         \n  b: hsl(160.1123681644, 426.4426548179%, 360094749.97600085%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklch(0% 1 0deg), $lightness: -10%), hsl)}\n"
        ),
        "a {\
         \n  b: hsl(342.6995890373, 454.6290714992%, 7.2218303523%);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(100% 0 0deg), hsl)}\n"),
        "a {\
         \n  b: hsl(43.8135971986, 172.5242105081%, 100.0000042145%);\
         \n}\n"
    );
}
