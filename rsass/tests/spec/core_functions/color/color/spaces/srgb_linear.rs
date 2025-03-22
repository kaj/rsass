//! Tests auto-converted from "sass-spec/spec/core_functions/color/color/spaces/srgb_linear.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("srgb_linear")
}

mod percent {
    use super::runner;

    mod blue {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb-linear 0.1 0.2 130%));\n"),
                "a {\
         \n  value: color(srgb-linear 0.1 0.2 1.3);\
         \n  space: srgb-linear;\
         \n  channels: 0.1 0.2 1.3 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb-linear 0.1 0.2 -30%));\n"),
                "a {\
         \n  value: color(srgb-linear 0.1 0.2 -0.3);\
         \n  space: srgb-linear;\
         \n  channels: 0.1 0.2 -0.3 / 1;\
         \n}\n"
            );
        }
    }
    mod green {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb-linear 0.1 120% 0.3));\n"),
                "a {\
         \n  value: color(srgb-linear 0.1 1.2 0.3);\
         \n  space: srgb-linear;\
         \n  channels: 0.1 1.2 0.3 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb-linear 0.1 -20% 0.3));\n"),
                "a {\
         \n  value: color(srgb-linear 0.1 -0.2 0.3);\
         \n  space: srgb-linear;\
         \n  channels: 0.1 -0.2 0.3 / 1;\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn in_range() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb-linear 10% 20% 30%));\n"),
            "a {\
         \n  value: color(srgb-linear 0.1 0.2 0.3);\
         \n  space: srgb-linear;\
         \n  channels: 0.1 0.2 0.3 / 1;\
         \n}\n"
        );
    }
    mod red {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb-linear 110% 0.2 0.3));\n"),
                "a {\
         \n  value: color(srgb-linear 1.1 0.2 0.3);\
         \n  space: srgb-linear;\
         \n  channels: 1.1 0.2 0.3 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb-linear -10% 0.2 0.3));\n"),
                "a {\
         \n  value: color(srgb-linear -0.1 0.2 0.3);\
         \n  space: srgb-linear;\
         \n  channels: -0.1 0.2 0.3 / 1;\
         \n}\n"
            );
        }
    }
}
mod unitless {
    use super::runner;

    mod blue {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb-linear 0.1 0.2 1.3));\n"),
                "a {\
         \n  value: color(srgb-linear 0.1 0.2 1.3);\
         \n  space: srgb-linear;\
         \n  channels: 0.1 0.2 1.3 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb-linear 0.1 0.2 -0.3));\n"),
                "a {\
         \n  value: color(srgb-linear 0.1 0.2 -0.3);\
         \n  space: srgb-linear;\
         \n  channels: 0.1 0.2 -0.3 / 1;\
         \n}\n"
            );
        }
    }
    mod green {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb-linear 0.1 1.2 0.3));\n"),
                "a {\
         \n  value: color(srgb-linear 0.1 1.2 0.3);\
         \n  space: srgb-linear;\
         \n  channels: 0.1 1.2 0.3 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb-linear 0.1 -0.2 0.3));\n"),
                "a {\
         \n  value: color(srgb-linear 0.1 -0.2 0.3);\
         \n  space: srgb-linear;\
         \n  channels: 0.1 -0.2 0.3 / 1;\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn in_range() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb-linear 0.1 0.2 0.3));\n"),
            "a {\
         \n  value: color(srgb-linear 0.1 0.2 0.3);\
         \n  space: srgb-linear;\
         \n  channels: 0.1 0.2 0.3 / 1;\
         \n}\n"
        );
    }
    mod red {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb-linear 1.1 0.2 0.3));\n"),
                "a {\
         \n  value: color(srgb-linear 1.1 0.2 0.3);\
         \n  space: srgb-linear;\
         \n  channels: 1.1 0.2 0.3 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(srgb-linear -0.1 0.2 0.3));\n"),
                "a {\
         \n  value: color(srgb-linear -0.1 0.2 0.3);\
         \n  space: srgb-linear;\
         \n  channels: -0.1 0.2 0.3 / 1;\
         \n}\n"
            );
        }
    }
}
