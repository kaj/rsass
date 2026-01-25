//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/display_p3_linear/srgb_linear.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("srgb_linear")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 0.3 / 0.4), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.0775059824 0.2042056955 0.3117911155 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 0.3 / 0.0), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.0775059824 0.2042056955 0.3117911155 / 0);\
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
             \na {b: color.to-space(color(display-p3-linear 0 0 0), srgb-linear)}\n"
        ),
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
             \na {b: color.to-space(color(display-p3-linear 0.5 0.5 0.5), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.5 0.5 0.5);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.2 0.4 0.8), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.1550119647 0.4084113909 0.843236951);\
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
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 none), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.0775059824 0.2042056955 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear 0.1 none 0.3), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.1224940176 none 0.3275183246);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear none 0.2 0.3), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear none 0.2084113909 0.3137548709);\
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
             \na {b: color.to-space(color(display-p3-linear -999999 0 0), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear -1224938.9513403836 42056.9126527335 19637.5349527798);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(display-p3-linear -1 0.4 2), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear -1.3149162468 0.4588797366 2.1847303367);\
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
             \na {b: color.to-space(color(display-p3-linear 1 1 1), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 1 1 1);\
         \n}\n"
    );
}
