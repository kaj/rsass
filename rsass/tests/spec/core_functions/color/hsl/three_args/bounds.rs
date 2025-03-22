//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/three_args/bounds.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("bounds")
}

mod lightness {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(hsl(0, 100%, 500%));\n"),
            "a {\
         \n  value: hsl(0, 100%, 500%);\
         \n  space: hsl;\
         \n  channels: 0deg 100% 500% / 1;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn below() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(hsl(0, 100%, -100%));\n"),
            "a {\
         \n  value: hsl(0, 100%, -100%);\
         \n  space: hsl;\
         \n  channels: 0deg 100% -100% / 1;\
         \n}\n"
        );
    }
}
mod saturation {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn above() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(hsl(0, 200%, 50%));\n"),
            "a {\
         \n  value: hsl(0, 200%, 50%);\
         \n  space: hsl;\
         \n  channels: 0deg 200% 50% / 1;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn below() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(hsl(0, -100%, 50%));\n"),
            "a {\
         \n  value: hsl(0, 0%, 50%);\
         \n  space: hsl;\
         \n  channels: 0deg 0% 50% / 1;\
         \n}\n"
        );
    }
}
