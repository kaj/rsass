//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/xyz/rec2020.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rec2020")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 0.2 0.3 / 0.4), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.2132921212 0.5717179697 0.584710702 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 0.2 0.3 / 0.0), rec2020)}\n"
        ),
        "a {\
         \n  b: color(rec2020 0.2132921212 0.5717179697 0.584710702 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0 0 0), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.5 0.5 0.5), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 0.7817466648 0.7382949204 0.7225800026);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.2 0.4 0.8), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 -0.068949908 0.765067036 0.8821414151);\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn x() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz none 0.2 0.3), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 none 0.6284859295 0.5831497901);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn y() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 none 0.3), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.3760930504 none 0.5921986764);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn z() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 0.1 0.2 none), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 0.3839649655 0.5673833022 none);\
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
             \na {b: color.to-space(color(xyz -999999 0 0), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 -396.0810002167 267.0750149624 -58.7993825157);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz -1 0.4 2), rec2020)}\n"),
            "a {\
         \n  b: color(rec2020 -1.4315709021 1.1313819628 1.2920179406);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz 1 1 1), rec2020)}\n"),
        "a {\
         \n  b: color(rec2020 1.0435066041 0.9855054839 0.9645285853);\
         \n}\n"
    );
}
