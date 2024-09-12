//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/a98_rgb/srgb_linear.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("srgb_linear")
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
             \na {b: color.to-space(color(a98-rgb 0.1 0.2 0.3 / 0.4), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear -0.0027244217 0.0290276622 0.0726003858 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.1 0.2 0.3 / 0.0), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear -0.0027244217 0.0290276622 0.0726003858 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0 0 0), srgb-linear)}\n"),
        "a {\
         \n  b: color(srgb-linear 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.5 0.5 0.5), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.2177555281 0.2177555281 0.2177555281);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.2 0.4 0.8), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear -0.012511378 0.1333039049 0.6327296478);\
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
             \na {b: color.to-space(color(a98-rgb 0.1 0.2 none), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear -0.0027244217 0.0290276622 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.1 none 0.3), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.0088389143 none 0.073846514);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb none 0.2 0.3), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear none 0.0290276622 0.0726003858);\
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
             \na {b: color.to-space(color(a98-rgb -999999 0 0), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear -21924475654205.21 0 0);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb -1 0.4 2), srgb-linear)}\n"
            ),
            "a {\
         \n  b: color(srgb-linear -1.4514581202 0.1333039049 4.7837263649);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 1 1 1), srgb-linear)}\n"),
        "a {\
         \n  b: color(srgb-linear 1 1 1);\
         \n}\n"
    );
}
