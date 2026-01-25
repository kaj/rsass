//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/display_p3_linear/display_p3_linear.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("display_p3_linear")
}

#[test]
#[ignore] // unexepected error
fn in_range() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.2 0.4 0.8), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.2 0.4 0.8);\
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
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 none), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.1 0.2 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 none 0.3), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.1 none 0.3);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear none 0.2 0.3), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear none 0.2 0.3);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn out_of_range() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear -1 0.4 2), display-p3-linear)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear -1 0.4 2);\
         \n}\n"
    );
}
