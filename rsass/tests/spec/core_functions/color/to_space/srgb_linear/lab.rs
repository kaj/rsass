//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/srgb_linear/lab.hrx"

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
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 0.3 / 0.4), lab)}\n"
        ),
        "a {\
         \n  b: lab(49.9553149355% -7.5157058766 -17.8791167699 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 0.3 / 0.0), lab)}\n"
        ),
        "a {\
         \n  b: lab(49.9553149355% -7.5157058766 -17.8791167699 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0 0 0), lab)}\n"),
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
             \na {b: color.to-space(color(srgb-linear 0.5 0.5 0.5), lab)}\n"),
        "a {\
         \n  b: lab(76.0692610142% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.2 0.4 0.8), lab)}\n"),
        "a {\
         \n  b: lab(68.0021326658% -3.4788867492 -36.298343272);\
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
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 0.2 none), lab)}\n"
            ),
            "a {\
         \n  b: lab(47.7042083773% -24.5180464109 51.183897624);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 0.1 none 0.3), lab)}\n"
            ),
            "a {\
         \n  b: lab(23.8148183096% 52.2336661374 -59.2206654307);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear none 0.2 0.3), lab)}\n"
            ),
            "a {\
         \n  b: lab(47.1789302985% -22.7267758708 -22.4056108481);\
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
             \na {b: color.to-space(color(srgb-linear -999999 0 0), lab)}\n"
        ),
        "a {\
         \n  b: color-mix(in lab, color(xyz -412390.3868751603 -212638.7932325045 -19330.7993847731) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear -1 0.4 2), lab)}\n"),
            "a {\
         \n  b: lab(50.1566645274% -199.4292910489 -127.4603757066);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(srgb-linear 1 1 1), lab)}\n"),
        "a {\
         \n  b: lab(100% 0 0);\
         \n}\n"
    );
}
