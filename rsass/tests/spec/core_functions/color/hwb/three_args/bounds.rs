//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/three_args/bounds.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("bounds")
}

mod blackness {
    use super::runner;

    mod above {
        use super::runner;

        mod whiteness {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn above() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.hwb(0, 120%, 150%));\n"),
                    "a {\
         \n  value: hsl(0, 0%, 44.4444444444%);\
         \n  space: hwb;\
         \n  channels: 0deg 44.4444444444% 55.5555555556% / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn below() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.hwb(0, -20%, 150%));\n"),
                    "a {\
         \n  value: hsl(0, 0%, -15.3846153846%);\
         \n  space: hwb;\
         \n  channels: 0deg -15.3846153846% 115.3846153846% / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn mid() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.hwb(0, 10%, 150%));\n"),
                    "a {\
         \n  value: hsl(0, 0%, 6.25%);\
         \n  space: hwb;\
         \n  channels: 0deg 6.25% 93.75% / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn zero() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.hwb(0, 0%, 150%));\n"),
                    "a {\
         \n  value: black;\
         \n  space: hwb;\
         \n  channels: 0deg 0% 100% / 1;\
         \n}\n"
                );
            }
        }
    }
    mod below {
        use super::runner;

        mod whiteness {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn above() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.hwb(0, 120%, -50%));\n"),
                    "a {\
         \n  value: hsl(180, 42.8571428571%, 135%);\
         \n  space: hwb;\
         \n  channels: 0deg 120% -50% / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn below() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.hwb(0, -20%, -50%));\n"),
                    "a {\
         \n  value: hsl(0, 242.8571428571%, 65%);\
         \n  space: hwb;\
         \n  channels: 0deg -20% -50% / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn mid() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.hwb(0, 10%, -50%));\n"),
                    "a {\
         \n  value: hsl(0, 350%, 80%);\
         \n  space: hwb;\
         \n  channels: 0deg 10% -50% / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn zero() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.hwb(0, 0%, -50%));\n"),
                    "a {\
         \n  value: hsl(0, 300%, 75%);\
         \n  space: hwb;\
         \n  channels: 0deg 0% -50% / 1;\
         \n}\n"
                );
            }
        }
    }
}
mod whiteness {
    use super::runner;

    mod above {
        use super::runner;

        mod blackness {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn above() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.hwb(0, 150%, 120%));\n"),
                    "a {\
         \n  value: hsl(0, 0%, 55.5555555556%);\
         \n  space: hwb;\
         \n  channels: 0deg 55.5555555556% 44.4444444444% / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn below() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.hwb(0, 150%, -20%));\n"),
                    "a {\
         \n  value: hsl(0, 0%, 115.3846153846%);\
         \n  space: hwb;\
         \n  channels: 0deg 115.3846153846% -15.3846153846% / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn mid() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.hwb(0, 150%, 10%));\n"),
                    "a {\
         \n  value: hsl(0, 0%, 93.75%);\
         \n  space: hwb;\
         \n  channels: 0deg 93.75% 6.25% / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn zero() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.hwb(0, 150%, 0%));\n"),
                    "a {\
         \n  value: white;\
         \n  space: hwb;\
         \n  channels: 0deg 100% 0% / 1;\
         \n}\n"
                );
            }
        }
    }
    mod below {
        use super::runner;

        mod blackness {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn above() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.hwb(0, -50%, 120%));\n"),
                    "a {\
         \n  value: hsl(180, 42.8571428571%, -35%);\
         \n  space: hwb;\
         \n  channels: 0deg -50% 120% / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn below() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.hwb(0, -50%, -20%));\n"),
                    "a {\
         \n  value: hsl(0, 242.8571428571%, 35%);\
         \n  space: hwb;\
         \n  channels: 0deg -50% -20% / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn mid() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.hwb(0, -50%, 10%));\n"),
                    "a {\
         \n  value: hsl(0, 350%, 20%);\
         \n  space: hwb;\
         \n  channels: 0deg -50% 10% / 1;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn zero() {
                assert_eq!(
                    runner().ok("@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.hwb(0, -50%, 0%));\n"),
                    "a {\
         \n  value: hsl(0, 300%, 25%);\
         \n  space: hwb;\
         \n  channels: 0deg -50% 0% / 1;\
         \n}\n"
                );
            }
        }
    }
}
