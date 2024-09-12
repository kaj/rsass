//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_gamut/display_p3.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("display_p3")
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
             \na {b: color.to-gamut(color(display-p3 0.2 0.5 0.8), $method: clip)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.2 0.5 0.8);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(display-p3 1.5 0.5 0.8), $method: clip)}\n"
        ),
        "a {\
         \n  b: color(display-p3 1 0.5 0.8);\
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
             \na {b: color.to-gamut(color(display-p3 0.2 0.5 0.8), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0.2 0.5 0.8);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn out_of_gamut() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(display-p3 1.5 0.5 0.8), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(display-p3 1 0.9080382844 0.9302909848);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn to_black() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(display-p3 0 -0.05 -0.05), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(display-p3 0 0 0);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn to_white() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.to-gamut(color(display-p3 0.8 1.1 1.4), $method: local-minde)}\n"
        ),
        "a {\
         \n  b: color(display-p3 1 1 1);\
         \n}\n"
    );
    }
}
