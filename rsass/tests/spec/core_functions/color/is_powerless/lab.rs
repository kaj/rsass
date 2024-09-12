//! Tests auto-converted from "sass-spec/spec/core_functions/color/is_powerless/lab.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lab")
}

mod a {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn full_lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.is-powerless(lab(100% 0 0), \"a\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero_lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.is-powerless(lab(0% 0 0), \"a\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
}
mod b {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn full_lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.is-powerless(lab(100% 0 0), \"b\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero_lightness() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.is-powerless(lab(0% 0 0), \"b\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
}
