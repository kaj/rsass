//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/a98_rgb/display_p3_linear.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("display_p3_linear")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.1 0.2 0.3 / 0.4), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0029127807 0.0279736772 0.0681590862 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.1 0.2 0.3 / 0.0), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0029127807 0.0279736772 0.0681590862 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0 0 0), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0 0 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.5 0.5 0.5), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.2177555281 0.2177555281 0.2177555281);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.2 0.4 0.8), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0133763802 0.1284636834 0.585550088);\
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
             \na {b: color.to-space(color(a98-rgb 0.1 0.2 none), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0029127807 0.0279736772 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb 0.1 none 0.3), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0072696708 none 0.0673897145);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb none 0.2 0.3), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear none 0.0276802766 0.0680080943);\
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
             \na {b: color.to-space(color(a98-rgb -999999 0 0), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear -18032047409587.723 -727765404568.7545 -374527721354.97406);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(a98-rgb -1 0.4 2), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear -1.1701025902 0.0806989991 4.3405343268);\
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
             \na {b: color.to-space(color(a98-rgb 1 1 1), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 1 1 1);\
         \n}\n"
    );
}
