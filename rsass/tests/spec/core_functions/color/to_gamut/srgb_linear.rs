//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_gamut/srgb_linear.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("srgb_linear")
}

mod clip {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn in_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(srgb-linear 0.2 0.5 0.8), $method: clip)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.2 0.5 0.8);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(srgb-linear 1.5 0.5 0.8), $method: clip)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 1 0.5 0.8);\
         \n}\n"
    );
    }
}
mod local_minde {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn in_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(srgb-linear 0.2 0.5 0.8), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0.2 0.5 0.8);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(srgb-linear 1.5 0.5 0.8), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 1 0.6455185586 0.7900912983);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn to_black() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(srgb-linear 0 -0.05 -0.05), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 0 0 0);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn to_white() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(srgb-linear 0.9 1.2 1.6), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(srgb-linear 1 1 1);\
         \n}\n"
    );
    }
}
