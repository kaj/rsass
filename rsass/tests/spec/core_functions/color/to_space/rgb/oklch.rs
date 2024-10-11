//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rgb/oklch.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklch")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 20 30 / 0.4), oklch)}\n"),
            "a {\
         \n  b: oklch(18.6989145594% 0.0253359348 249.3231163305deg / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 20 30 / 0.0), oklch)}\n"),
            "a {\
         \n  b: oklch(18.6989145594% 0.0253359348 249.3231163305deg / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#000, oklch)}\n"),
        "a {\
         \n  b: oklch(0% 0 none);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn float() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(rgb(50.123456789 100.987654321 200.192837465), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(52.7265726495% 0.1642181438 262.0110608448deg);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#aaa, oklch)}\n"),
        "a {\
         \n  b: oklch(73.8018670949% 0 none);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#28d, oklch)}\n"),
        "a {\
         \n  b: oklch(61.3651182767% 0.1562424067 249.3182371499deg);\
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
             \na {b: color.to-space(rgb(10 20 none), oklch)}\n"),
            "a {\
         \n  b: oklch(17.4737574106% 0.0462108173 128.8017058762deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(10 none 30), oklch)}\n"),
            "a {\
         \n  b: oklch(12.5934962291% 0.0689567196 295.5019774519deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(rgb(none 20 30), oklch)}\n"),
            "a {\
         \n  b: oklch(17.9105840016% 0.0357110855 230.0496886646deg);\
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
             \na {b: color.to-space(color.change(black, $red: -999999), oklch)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklch, color(xyz -152693379.43919486 -78732523.7733348 -7157502.161212236) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(rgb(0, 100, 0), $red: -50, $blue: 400), oklch)}\n"
        ),
        "a {\
         \n  b: oklch(69.4063331508% 0.4056030863 261.9120997817deg);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(#fff, oklch)}\n"),
        "a {\
         \n  b: oklch(100% 0 none);\
         \n}\n"
    );
}
