//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklab/lch.hrx"

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
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.4), lch)}\n"),
            "a {\
         \n  b: lch(2.4858192097% 79.7774716477 61.6719233557deg / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.0), lch)}\n"),
            "a {\
         \n  b: lch(2.4858192097% 79.7774716477 61.6719233557deg / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(0% 0 0), lch)}\n"),
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
             \na {b: color.to-space(oklab(50% 0 0), lch)}\n"),
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
             \na {b: color.to-space(oklab(50% 0.2 -0.3), lch)}\n"),
        "a {\
         \n  b: lch(33.0422497851% 152.1726359305 314.7965716122deg);\
         \n}\n"
    );
}
mod missing {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn a() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% none 0.3), lch)}\n"),
            "a {\
         \n  b: lch(2.4372581534% 59.0755529992 98.6375851746deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 none), lch)}\n"),
            "a {\
         \n  b: lch(0.4263319128% 27.5527624534 1.0818401525deg);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(none 0.2 0.3), lch)}\n"),
            "a {\
         \n  b: lch(none 171.7356811932 112.1415379531deg);\
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
             \na {b: color.to-space(oklab(50% -999999 0), lch)}\n"
        ),
        "a {\
         \n  b: color-mix(in lch, color(xyz -76842638588973744 3781392309267311 5284390240038480) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklab(0% -2 2), $lightness: -50%), lch)}\n"
        ),
        "a {\
         \n  b: color-mix(in lch, color(xyz -7.6342507319 1.7017043263 -38.7847424885) 100%, black);\
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
             \na {b: color.to-space(oklab(100% 0 0), lch)}\n"
        ),
        "a {\
         \n  b: color-mix(in lch, color(xyz 0.9504559526 1.0000000182 1.0890580001) 100%, black);\
         \n}\n"
    );
}
