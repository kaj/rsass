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
         \n  b: oklch(18.6989144442% 0.0253359283 249.3231108283deg / 0.4);\
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
         \n  b: oklch(18.6989144442% 0.0253359283 249.3231108283deg / 0);\
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
         \n  b: oklch(52.7265723906% 0.1642181246 262.0110592016deg);\
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
         \n  b: oklch(73.8018666132% 0.0000000275 89.8755625379deg);\
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
         \n  b: oklch(61.3651179384% 0.1562423854 249.3182340149deg);\
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
         \n  b: oklch(17.4737572915% 0.0462108221 128.8016996944deg);\
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
         \n  b: oklch(12.5934961979% 0.0689567144 295.5019774012deg);\
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
         \n  b: oklch(17.9105838927% 0.0357110801 230.049682151deg);\
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
         \n  b: color-mix(in oklch, color(xyz -152693379.43919486 -78732523.77333483 -7157502.16121231) 100%, black);\
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
         \n  b: oklch(69.4063330454% 0.405603061 261.9120979439deg);\
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
         \n  b: oklch(99.9999993474% 0.0000000373 89.8755630959deg);\
         \n}\n"
    );
}
