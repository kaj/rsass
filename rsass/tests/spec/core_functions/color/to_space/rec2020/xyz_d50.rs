//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rec2020/xyz_d50.hrx"

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
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.4), xyz-d50)}\n"
        ),
        "a {\
         \n  b: color(xyz-d50 0.0131178529 0.017837619 0.0449403544 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.0), xyz-d50)}\n"
        ),
        "a {\
         \n  b: color(xyz-d50 0.0131178529 0.017837619 0.0449403544 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0 0 0), xyz-d50)}\n"),
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
             \na {b: color.to-space(color(rec2020 0.5 0.5 0.5), xyz-d50)}\n"),
        "a {\
         \n  b: color(xyz-d50 0.1826998665 0.1894645708 0.1563280894);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.2 0.4 0.8), xyz-d50)}\n"),
        "a {\
         \n  b: color(xyz-d50 0.1057457722 0.1074639988 0.4698430078);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 none), xyz-d50)}\n"
        ),
        "a {\
         \n  b: color(xyz-d50 0.0061629811 0.0153008861 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 none 0.3), xyz-d50)}\n"
        ),
        "a {\
         \n  b: color(xyz-d50 0.0096361851 none 0.0443104537);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 none 0.2 0.3), xyz-d50)}\n"
        ),
        "a {\
         \n  b: color(xyz-d50 none 0.0167266651 0.0449480476);\
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
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 -999999 0 0), xyz-d50)}\n"),
            "a {\
         \n  b: color(xyz-d50 -169179029309091.25 -70096284629085.1 485402584810.47015);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 -1 0.4 2), xyz-d50)}\n"),
            "a {\
         \n  b: color(xyz-d50 0.0050526551 0.0366354859 4.2121607574);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 1 1 1), xyz-d50)}\n"),
        "a {\
         \n  b: color(xyz-d50 0.9642956764 1 0.8251046025);\
         \n}\n"
    );
}
