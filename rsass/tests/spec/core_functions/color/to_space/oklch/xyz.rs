//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklch/xyz.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("xyz")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.4), xyz)}\n"),
            "a {\
         \n  b: color(xyz 0.0033792787 0.0006238666 -0.0004837129 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg / 0.0), xyz)}\n"),
            "a {\
         \n  b: color(xyz 0.0033792787 0.0006238666 -0.0004837129 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(0% 0 0deg), xyz)}\n"),
        "a {\
         \n  b: color(xyz 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(oklch(10.123456789% 0.198765432 30.192837465deg), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz 0.0082604044 0.0002257242 -0.0008498824);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(50% 0 0deg), xyz)}\n"),
        "a {\
         \n  b: color(xyz 0.1188069941 0.1250000023 0.13613225);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 30deg), xyz)}\n"),
        "a {\
         \n  b: color(xyz 0.0033792787 0.0006238666 -0.0004837129);\
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
             \na {b: color.to-space(oklch(10% none 30deg), xyz)}\n"),
            "a {\
         \n  b: color(xyz 0.000950456 0.001 0.001089058);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(10% 0.1 none), xyz)}\n"),
            "a {\
         \n  b: color(xyz 0.0031534617 0.0006313185 0.0006883601);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(none 0.1 30deg), xyz)}\n"),
            "a {\
         \n  b: color(xyz 0.0000072462 0.0000213158 -0.000606566);\
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
             \na {b: color.to-space(oklch(10% 999999 0deg), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz 76837666021869456 -3783149507010139 -5396117824941693);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklch(0% 1 0deg), $lightness: -10%), xyz)}\n"
        ),
        "a {\
         \n  b: color(xyz 0.0348582183 -0.010229465 -0.0091226442);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklch(100% 0 0deg), xyz)}\n"),
        "a {\
         \n  b: color(xyz 0.9504559526 1.0000000182 1.0890580001);\
         \n}\n"
    );
}
