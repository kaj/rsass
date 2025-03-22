//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/three_args/out_of_gamut.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("out_of_gamut")
}

mod saturation {
    use super::runner;

    #[test]
    fn above() {
        assert_eq!(
            runner().ok("a {b: hsl(0, 500%, 50%)}\n"),
            "a {\
         \n  b: hsl(0, 500%, 50%);\
         \n}\n"
        );
    }
}
