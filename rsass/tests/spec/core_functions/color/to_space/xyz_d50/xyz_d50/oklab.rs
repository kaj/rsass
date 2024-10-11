//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/xyz_d50/xyz-d50/oklab.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("oklab")
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
             \na {b: color.to-space(color(xyz-d50 0.1 0.2 0.3 / 0.4), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(56.9970930189% -0.1786682066 -0.0706832815 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.1 0.2 0.3 / 0.0), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(56.9970930189% -0.1786682066 -0.0706832815 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0 0 0), oklab)}\n"),
        "a {\
         \n  b: oklab(0% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.5 0.5 0.5), oklab)}\n"),
        "a {\
         \n  b: oklab(79.6577663953% 0.0133226729 -0.0281100683);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.2 0.4 0.8), oklab)}\n"),
        "a {\
         \n  b: oklab(71.9979204728% -0.2538804808 -0.1411846361);\
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
             \na {b: color.to-space(color(xyz-d50 0.1 0.2 none), oklab)}\n"),
            "a {\
         \n  b: oklab(56.4114763984% -0.1759135199 0.154183989);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.1 none 0.3), oklab)}\n"),
            "a {\
         \n  b: oklab(29.1994680372% 0.3079984413 -0.28949216);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 none 0.2 0.3), oklab)}\n"),
            "a {\
         \n  b: oklab(53.7370326457% -0.4814824036 -0.0723240854);\
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
             \na {b: color.to-space(color(xyz-d50 -999999 0 0), oklab)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklab, color(xyz -955472.4660146532 28369.6809641542 -12314.0025504671) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 -1 0.4 2), oklab)}\n"),
            "a {\
         \n  b: oklab(42.8855762363% -3.2722753539 -0.3756610884);\
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
             \na {b: color.to-space(color(xyz-d50 1 1 1), oklab)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklab, color(xyz 0.9956342097 1.0026671299 1.3221722918) 100%, black);\
         \n}\n"
    );
}
