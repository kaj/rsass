//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_gamut/prophoto_rgb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("prophoto_rgb")
}

mod clip {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn in_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(prophoto-rgb 0.2 0.5 0.8), $method: clip)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.2 0.5 0.8);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(prophoto-rgb 1.5 0.5 0.8), $method: clip)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 1 0.5 0.8);\
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
             \na {b: color.to-gamut(color(prophoto-rgb 0.2 0.5 0.8), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0.2 0.5 0.8);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(prophoto-rgb 1.1 0.5 0.8), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 1 0.5625312191 0.7939519353);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn to_black() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(prophoto-rgb 0 -0.05 -0.05), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 0 0 0);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn to_white() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(prophoto-rgb 0.8 1.1 1.4), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(prophoto-rgb 1 1 1);\
         \n}\n"
    );
    }
}
