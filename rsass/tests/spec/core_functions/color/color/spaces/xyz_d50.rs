//! Tests auto-converted from "sass-spec/spec/core_functions/color/color/spaces/xyz_d50.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("xyz_d50")
}

mod percent {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn in_range() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(xyz-d50 10% 20% 30%));\n"),
            "a {\
         \n  value: color(xyz-d50 0.1 0.2 0.3);\
         \n  space: xyz-d50;\
         \n  channels: 0.1 0.2 0.3 / 1;\
         \n}\n"
        );
    }
    mod x {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(xyz-d50 110% 0.2 0.3));\n"),
                "a {\
         \n  value: color(xyz-d50 1.1 0.2 0.3);\
         \n  space: xyz-d50;\
         \n  channels: 1.1 0.2 0.3 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(xyz-d50 -10% 0.2 0.3));\n"),
                "a {\
         \n  value: color(xyz-d50 -0.1 0.2 0.3);\
         \n  space: xyz-d50;\
         \n  channels: -0.1 0.2 0.3 / 1;\
         \n}\n"
            );
        }
    }
    mod y {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(xyz-d50 0.1 120% 0.3));\n"),
                "a {\
         \n  value: color(xyz-d50 0.1 1.2 0.3);\
         \n  space: xyz-d50;\
         \n  channels: 0.1 1.2 0.3 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(xyz-d50 0.1 -20% 0.3));\n"),
                "a {\
         \n  value: color(xyz-d50 0.1 -0.2 0.3);\
         \n  space: xyz-d50;\
         \n  channels: 0.1 -0.2 0.3 / 1;\
         \n}\n"
            );
        }
    }
    mod z {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(xyz-d50 0.1 0.2 130%));\n"),
                "a {\
         \n  value: color(xyz-d50 0.1 0.2 1.3);\
         \n  space: xyz-d50;\
         \n  channels: 0.1 0.2 1.3 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(xyz-d50 0.1 0.2 -30%));\n"),
                "a {\
         \n  value: color(xyz-d50 0.1 0.2 -0.3);\
         \n  space: xyz-d50;\
         \n  channels: 0.1 0.2 -0.3 / 1;\
         \n}\n"
            );
        }
    }
}
mod unitless {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn in_range() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(xyz-d50 0.1 0.2 0.3));\n"),
            "a {\
         \n  value: color(xyz-d50 0.1 0.2 0.3);\
         \n  space: xyz-d50;\
         \n  channels: 0.1 0.2 0.3 / 1;\
         \n}\n"
        );
    }
    mod x {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(xyz-d50 1.1 0.2 0.3));\n"),
                "a {\
         \n  value: color(xyz-d50 1.1 0.2 0.3);\
         \n  space: xyz-d50;\
         \n  channels: 1.1 0.2 0.3 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(xyz-d50 -0.1 0.2 0.3));\n"),
                "a {\
         \n  value: color(xyz-d50 -0.1 0.2 0.3);\
         \n  space: xyz-d50;\
         \n  channels: -0.1 0.2 0.3 / 1;\
         \n}\n"
            );
        }
    }
    mod y {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(xyz-d50 0.1 1.2 0.3));\n"),
                "a {\
         \n  value: color(xyz-d50 0.1 1.2 0.3);\
         \n  space: xyz-d50;\
         \n  channels: 0.1 1.2 0.3 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(xyz-d50 0.1 -0.2 0.3));\n"),
                "a {\
         \n  value: color(xyz-d50 0.1 -0.2 0.3);\
         \n  space: xyz-d50;\
         \n  channels: 0.1 -0.2 0.3 / 1;\
         \n}\n"
            );
        }
    }
    mod z {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn above_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(xyz-d50 0.1 0.2 1.3));\n"),
                "a {\
         \n  value: color(xyz-d50 0.1 0.2 1.3);\
         \n  space: xyz-d50;\
         \n  channels: 0.1 0.2 1.3 / 1;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn below_range() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color(xyz-d50 0.1 0.2 -0.3));\n"),
                "a {\
         \n  value: color(xyz-d50 0.1 0.2 -0.3);\
         \n  space: xyz-d50;\
         \n  channels: 0.1 0.2 -0.3 / 1;\
         \n}\n"
            );
        }
    }
}
