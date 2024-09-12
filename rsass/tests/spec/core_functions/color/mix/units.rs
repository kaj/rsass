//! Tests auto-converted from "sass-spec/spec/core_functions/color/mix/units.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("units")
}

mod weight {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn unitless() {
        assert_eq!(
            runner().ok("a {b: mix(#91e16f, #0144bf, 50)}\n"),
            "a {\
         \n  b: rgb(73, 146.5, 151);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn unknown() {
        assert_eq!(
            runner().ok("a {b: mix(#91e16f, #0144bf, 50px)}\n"),
            "a {\
         \n  b: rgb(73, 146.5, 151);\
         \n}\n"
        );
    }
}
