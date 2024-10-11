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
         \n  b: hsla(349.9222385729, 174.308760087%, 3.6948783411%, 0.4);\
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
         \n  b: hsla(349.9222385729, 174.308760087%, 3.6948783411%, 0);\
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
         \n  b: hsl(342.3640348742, 274.4922123902%, 4.7714615118%);\
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
         \n  b: hsl(0, 0%, 38.8572859046%);\
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
         \n  b: hsl(349.9222385729, 174.308760087%, 3.6948783411%);\
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
         \n  b: hsl(0, 0%, 1.292%);\
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
         \n  b: hsl(0, 169.3004993061%, 3.4369836159%);\
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
         \n  b: hsl(221.7487198664, 266.6061126985%, 0%);\
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
         \n  b: hsl(160.1123665311, 426.4426501978%, 360094735.8725038%);\
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
         \n  b: hsl(342.6995872052, 454.6290807287%, 7.2218298303%);\
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
         \n  b: hsl(161.8181818182, 266.6666666667%, 100%);\
         \n}\n"
    );
}
