//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_gamut/a98_rgb.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("a98_rgb")
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
             \na {b: color.to-gamut(color(a98-rgb 0.2 0.5 0.8), $method: clip)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.2 0.5 0.8);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(a98-rgb 1.5 0.5 0.8), $method: clip)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 1 0.5 0.8);\
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
             \na {b: color.to-gamut(color(a98-rgb 0.2 0.5 0.8), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0.2 0.5 0.8);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(a98-rgb 1.1 0.5 0.8), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 1 0.5850609644 0.8001989651);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn to_black() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(a98-rgb 0 -0.05 -0.05), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 0 0 0);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn to_white() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(a98-rgb 0.8 1.1 1.4), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(a98-rgb 1 1 1);\
         \n}\n"
    );
    }
}
