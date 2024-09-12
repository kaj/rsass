//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/prophoto_rgb/oklab.hrx"

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
             \na {b: color.to-space(color(prophoto-rgb 0.1 0.2 0.3 / 0.4), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(34.6066204299% -0.1151566731 -0.0744029999 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.1 0.2 0.3 / 0.0), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(34.6066204299% -0.1151566731 -0.0744029999 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0 0 0), oklab)}\n"),
        "a {\
         \n  b: oklab(0% 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.5 0.5 0.5), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(65.9753951081% 0.0000000001 0.0000000246);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.2 0.4 0.8), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(52.9739863521% -0.23472674 -0.1933931965);\
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
             \na {b: color.to-space(color(prophoto-rgb 0.1 0.2 none), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(33.9153744238% -0.1155256021 0.0932618247);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 0.1 none 0.3), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(20.6577507967% -0.042480718 -0.1739784043);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb none 0.2 0.3), oklab)}\n"
        ),
        "a {\
         \n  b: oklab(32.493750959% -0.2028589632 -0.0824787667);\
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
             \na {b: color.to-space(color(prophoto-rgb -999999 0 0), oklab)}\n"
        ),
        "a {\
         \n  b: color-mix(in oklab, color(xyz -47674467013.18752 -16929933315.11392 -247080732.77775192) 100%, black);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb -1 0.4 2), oklab)}\n"),
            "a {\
         \n  b: oklab(9.4597839538% -2.2223701334 -0.8062107579);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(prophoto-rgb 1 1 1), oklab)}\n"),
        "a {\
         \n  b: oklab(99.9999993474% 0.0000000001 0.0000000373);\
         \n}\n"
    );
}
