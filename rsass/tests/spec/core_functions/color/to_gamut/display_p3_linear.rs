//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_gamut/display_p3_linear.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("display_p3_linear")
}

mod clip {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn in_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(display-p3-linear 0.2 0.5 0.8), $method: clip)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.2 0.5 0.8);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(display-p3-linear 1.5 0.5 0.8), $method: clip)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 1 0.5 0.8);\
         \n}\n"
    );
    }
}
mod local_minde {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn in_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(display-p3-linear 0.2 0.5 0.8), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0.2 0.5 0.8);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(display-p3-linear 1.5 0.5 0.8), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 1 0.675538366 0.8056759629);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn to_black() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(display-p3-linear 0 -0.05 -0.05), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 0 0 0);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn to_white() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(display-p3-linear 0.8 1.1 1.4), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(display-p3-linear 1 1 1);\
         \n}\n"
    );
    }
}
