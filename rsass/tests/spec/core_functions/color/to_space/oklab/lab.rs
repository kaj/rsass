//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/oklab/lab.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lab")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.4), lab)}\n"),
            "a {\
         \n  b: lab(2.4858192097% 37.8559749186 70.2237149791 / 0.4);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 0.3 / 0.0), lab)}\n"),
            "a {\
         \n  b: lab(2.4858192097% 37.8559749186 70.2237149791 / 0);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(0% 0 0), lab)}\n"),
        "a {\
         \n  b: lab(0% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(50% 0 0), lab)}\n"),
        "a {\
         \n  b: lab(42.0000002803% 0.0000000582 -0.0000070926);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(50% 0.2 -0.3), lab)}\n"),
        "a {\
         \n  b: lab(33.0422497851% 107.2195839075 -107.9836652126);\
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
             \na {b: color.to-space(oklab(10% none 0.3), lab)}\n"),
            "a {\
         \n  b: lab(2.4372581534% none 58.4055225293);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn b() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(10% 0.2 none), lab)}\n"),
            "a {\
         \n  b: lab(0.4263319128% 27.5478510774 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(oklab(none 0.2 0.3), lab)}\n"),
            "a {\
         \n  b: lab(none -64.726469561 159.0711423642);\
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
             \na {b: color.to-space(oklab(50% -999999 0), lab)}\n"
        ),
        "a {\
         \n  b: color-mix(in lab, color(xyz -76837326254677664 3783159310641774 5396110649242741) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color.change(oklab(0% -2 2), $lightness: -50%), lab)}\n"
        ),
        "a {\
         \n  b: color-mix(in lab, color(xyz -7.6342507319 1.7017043263 -38.7847424885) 100%, black);\
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
             \na {b: color.to-space(oklab(100% 0 0), lab)}\n"
        ),
        "a {\
         \n  b: color-mix(in lab, color(xyz 0.9504559526 1.0000000182 1.0890580001) 100%, black);\
         \n}\n"
    );
}
