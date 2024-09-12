//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/three_args/out_of_gamut.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("out_of_gamut")
}

mod saturation {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn above() {
        assert_eq!(
            runner().ok("a {b: hsl(0, 500%, 50%)}\n"),
            "a {\
         \n  b: hsl(0, 500%, 50%);\
         \n}\n"
        );
    }
}
