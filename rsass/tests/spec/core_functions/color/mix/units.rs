//! Tests auto-converted from "sass-spec/spec/core_functions/color/mix/units.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("units")
}

mod weight {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn unitless() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.mix(#91e16f, #0144bf, 50)}\n"),
            "a {\
         \n  b: rgb(28.6274509804%, 57.4509803922%, 59.2156862745%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn unknown() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.mix(#91e16f, #0144bf, 50px)}\n"),
            "a {\
         \n  b: rgb(28.6274509804%, 57.4509803922%, 59.2156862745%);\
         \n}\n"
        );
    }
}
