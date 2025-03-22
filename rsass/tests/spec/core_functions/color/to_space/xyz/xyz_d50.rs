//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/xyz/xyz_d50.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("xyz_d50")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 0.2 0.3 / 0.4), xyz-d50)}\n"
        ),
        "a {\
         \n  b: color(xyz-d50 0.0943246735 0.1959275265 0.2276490187 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 0.2 0.3 / 0.0), xyz-d50)}\n"
        ),
        "a {\
         \n  b: color(xyz-d50 0.0943246735 0.1959275265 0.2276490187 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0 0 0), xyz-d50)}\n"),
        "a {\
         \n  b: color(xyz-d50 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.5 0.5 0.5), xyz-d50)}\n"),
        "a {\
         \n  b: color(xyz-d50 0.5103421984 0.5014942182 0.3788432161);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.2 0.4 0.8), xyz-d50)}\n"),
        "a {\
         \n  b: color(xyz-d50 0.1786108937 0.3884402932 0.6056728936);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 0.2 none), xyz-d50)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.1093823534 0.2010496662 none);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 none 0.3), xyz-d50)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.0897352994 none 0.2246379804);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz none 0.2 0.3), xyz-d50)}\n"),
            "a {\
         \n  b: color(xyz-d50 none 0.1929647456 0.2285733227);\
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
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz -999999 0 0), xyz-d50)}\n"
        ),
        "a {\
         \n  b: color(xyz-d50 -1047928.744615204 -29627.7791422469 9243.0314031639);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz -1 0.4 2), xyz-d50)}\n"),
            "a {\
         \n  b: color(xyz-d50 -1.1391355769 0.3323983638 1.5190136801);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 1 1 1), xyz-d50)}\n"),
        "a {\
         \n  b: color(xyz-d50 1.0206843969 1.0029884365 0.7576864323);\
         \n}\n"
    );
}
