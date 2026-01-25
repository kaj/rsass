//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/srgb/display_p3_linear.hrx"

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
             \na {b: color.to-space(color(srgb 0.1 0.2 0.3 / 0.4), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0141207479 0.03233858 0.0692534455 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.1 0.2 0.3 / 0.0), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0141207479 0.03233858 0.0692534455 / 0);\
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
             \na {b: color.to-space(color(srgb 0 0 0), display-p3-linear)}\n"
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
             \na {b: color.to-space(color(srgb 0.5 0.5 0.5), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.2140411405 0.2140411405 0.2140411405);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.2 0.4 0.8), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0508165917 0.1295567503 0.5599816684);\
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
             \na {b: color.to-space(color(srgb 0.1 0.2 none), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0141207479 0.03233858 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb 0.1 none 0.3), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.0082433929 none 0.0668567451);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb none 0.2 0.3), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear none 0.0320058804 0.0690822293);\
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
             \na {b: color.to-space(color(srgb -999999 0 0), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear -181680759551756.2 -7332554561011.1875 -3773530500634.1445);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(srgb -1 0.4 2), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear -0.7988727885 0.0952636652 4.5031119758);\
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
             \na {b: color.to-space(color(srgb 1 1 1), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 1 1 1);\
         \n}\n"
    );
}
