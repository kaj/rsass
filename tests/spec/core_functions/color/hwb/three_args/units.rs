//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/three_args/units.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod hue {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn deg() {
        assert_eq!(
            runner().ok("@use \'sass:color\';\
             \na {b: color.hwb(0deg, 30%, 40%)}\n"),
            "a {\
         \n  b: #994d4d;\
         \n}\n"
        );
    }
}
