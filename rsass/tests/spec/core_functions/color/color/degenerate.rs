//! Tests auto-converted from "sass-spec/spec/core_functions/color/color/degenerate.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("degenerate")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn nan() {
        assert_eq!(
            runner().ok("a {b: color(srgb 0 0 0 / calc(NaN))}\n"),
            "a {\
         \n  b: color(srgb 0 0 0 / 0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn negative_infinity() {
        assert_eq!(
            runner().ok("a {b: color(srgb 0 0 0 / calc(-infinity))}\n"),
            "a {\
         \n  b: color(srgb 0 0 0 / 0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn positive_infinity() {
        assert_eq!(
            runner().ok("a {b: color(srgb 0 0 0 / calc(infinity))}\n"),
            "a {\
         \n  b: color(srgb 0 0 0);\
         \n}\n"
        );
    }
}
mod arg1 {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn nan() {
        assert_eq!(
            runner().ok("a {b: color(srgb calc(NaN) 0 0)}\n"),
            "a {\
         \n  b: color(srgb calc(NaN) 0 0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn negative_infinity() {
        assert_eq!(
            runner().ok("a {b: color(srgb calc(-infinity) 0 0)}\n"),
            "a {\
         \n  b: color(srgb calc(-infinity) 0 0);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn positive_infinity() {
        assert_eq!(
            runner().ok("a {b: color(srgb calc(infinity) 0 0)}\n"),
            "a {\
         \n  b: color(srgb calc(infinity) 0 0);\
         \n}\n"
        );
    }
}
mod before_alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn nan() {
        assert_eq!(
            runner().ok("a {b: color(srgb 0 0 calc(NaN) / 0.5)}\n"),
            "a {\
         \n  b: color(srgb 0 0 calc(NaN) / 0.5);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn negative_infinity() {
        assert_eq!(
            runner().ok("a {b: color(srgb 0 0 calc(-infinity) / 0.5)}\n"),
            "a {\
         \n  b: color(srgb 0 0 calc(-infinity) / 0.5);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn positive_infinity() {
        assert_eq!(
            runner().ok("a {b: color(srgb 0 0 calc(infinity) / 0.5)}\n"),
            "a {\
         \n  b: color(srgb 0 0 calc(infinity) / 0.5);\
         \n}\n"
        );
    }
}
