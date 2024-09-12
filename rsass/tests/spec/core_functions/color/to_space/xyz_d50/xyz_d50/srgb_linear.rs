//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/xyz_d50/xyz-d50/srgb_linear.hrx"

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
             \na {b: color.to-space(color(xyz-d50 0.1 0.2 0.3 / 0.4), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear -0.1572622797 0.2954041898 0.3830159978 / 0.4);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.1 0.2 0.3 / 0.0), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear -0.1572622797 0.2954041898 0.3830159978 / 0);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0 0 0), srgb-linear)}\n"),
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
             \na {b: color.to-space(color(xyz-d50 0.5 0.5 0.5), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.5130438185 0.4854508871 0.6241823339);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.2 0.4 0.8), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear -0.412657003 0.5974969544 1.0471092027);\
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
             \na {b: color.to-space(color(xyz-d50 0.1 0.2 none), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear -0.0100636143 0.2853713278 none);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 0.1 none 0.3), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.1662149199 none 0.4288113498);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 none 0.2 0.3), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear none 0.3932837375 0.3758204586);\
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
             \na {b: color.to-space(color(xyz-d50 -999999 0 0), srgb-linear)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear -3134132.718764265 978794.4977603011 -71955.3206025548);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
            runner().ok(
                "@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 -1 0.4 2), srgb-linear)}\n"
            ),
            "a {\
         \n  b: color(srgb-linear -4.7624146879 1.8121829743 2.6472259737);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.to-space(color(xyz-d50 1 1 1), srgb-linear)}\n"),
        "a {\
         \n  b: color(srgb-linear 1.026087637 0.9709017742 1.2483646679);\
         \n}\n"
    );
}
