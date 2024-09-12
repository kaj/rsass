//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/rec2020/oklab.hrx"

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
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.4), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(36.3512304678% -0.050217455 -0.0580072349 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 0.2 0.3 / 0.0), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(36.3512304678% -0.050217455 -0.0580072349 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0 0 0), oklab)}\n"),
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
             \na {b: color.to-space(color(rec2020 0.5 0.5 0.5), oklab)}\n"),
        "a {\
         \n  b: oklab(63.8020766953% 0.0000000001 0.0000000238);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.2 0.4 0.8), oklab)}\n"),
        "a {\
         \n  b: oklab(55.2815992148% -0.0857471824 -0.1749134266);\
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
             \na {b: color.to-space(color(rec2020 0.1 0.2 none), oklab)}\n"),
            "a {\
         \n  b: oklab(34.13560351% -0.0917726115 0.0859786399);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 0.1 none 0.3), oklab)}\n"),
            "a {\
         \n  b: oklab(25.3247541535% 0.0750438961 -0.1364735034);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 none 0.2 0.3), oklab)}\n"),
            "a {\
         \n  b: oklab(34.1361541836% -0.1128551253 -0.067830888);\
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
             \na {b: color.to-space(color(rec2020 -999999 0 0), oklab)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklab, color(xyz -11119280444659.65 -4585917923503.701 0.0024414063) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 -1 0.4 2), oklab)}\n"),
            "a {\
         \n  b: oklab(33.9832655696% -2.4377896861 -0.667787683);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(rec2020 1 1 1), oklab)}\n"),
        "a {\
         \n  b: oklab(99.9999993474% 0.0000000001 0.0000000373);\
         \n}\n"
    );
}
