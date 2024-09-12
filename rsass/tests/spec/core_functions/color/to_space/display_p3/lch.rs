//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/display_p3/lch.hrx"

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
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 0.1 0.2 0.3 / 0.4), lch)}\n"
        ),
        "a {\
         \n  b: lch(20.127570568% 21.268258991 256.0066071682deg / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 0.1 0.2 0.3 / 0.0), lch)}\n"
        ),
        "a {\
         \n  b: lch(20.127570568% 21.268258991 256.0066071682deg / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 0 0 0), lch)}\n"),
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
             \na {b: color.to-space(color(display-p3 0.5 0.5 0.5), lch)}\n"),
        "a {\
         \n  b: lch(53.3889647411% 0 none);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 0.2 0.4 0.8), lch)}\n"),
        "a {\
         \n  b: lch(44.25592932% 63.6944584797 278.686909273deg);\
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
             \na {b: color.to-space(color(display-p3 0.1 0.2 none), lch)}\n"),
            "a {\
         \n  b: lch(18.0687084518% 36.3592591914 128.2437408357deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 0.1 none 0.3), lch)}\n"),
            "a {\
         \n  b: lch(6.5882355616% 53.8167472672 306.386978589deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 none 0.2 0.3), lch)}\n"),
            "a {\
         \n  b: lch(19.1369530209% 25.1058836166 242.9021173179deg);\
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
             \na {b: color.to-space(color(display-p3 -999999 0 0), lch)}\n"
        ),
        "a {\
         \n  b: color-mix(in lch, color(xyz -107482896009634.61 -50580183886727.57 -376620475.52490234) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 -1 0.4 2), lch)}\n"),
            "a {\
         \n  b: lch(49.5672259701% 229.5452219725 284.3695672545deg);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3 1 1 1), lch)}\n"),
        "a {\
         \n  b: lch(100% 0 none);\
         \n}\n"
    );
}
