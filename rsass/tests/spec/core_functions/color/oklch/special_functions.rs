//! Tests auto-converted from "sass-spec/spec/core_functions/color/oklch/special_functions.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("special_functions")
}

mod calculation {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn arg_1() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(calc(1px + 1%) 0.2 3deg));\n"),
            "a {\
         \n  value: oklch(calc(1px + 1%) 0.2 3deg);\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_2() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1% calc(1px + 1%) 3deg));\n"),
            "a {\
         \n  value: oklch(1% calc(1px + 1%) 3deg);\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_3() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1% 0.2 calc(1px + 1%)));\n"),
            "a {\
         \n  value: oklch(1% 0.2 calc(1px + 1%));\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_4() {
        assert_eq!(
        runner().ok(
            "@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1% 0.2 3deg / calc(1px + 1%)));\n"
        ),
        "a {\
         \n  value: oklch(1% 0.2 3deg/calc(1px + 1%));\
         \n  type: string;\
         \n}\n"
    );
    }
}
mod multi_argument_var {
    #[allow(unused)]
    use super::runner;

    mod t1_of_1 {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn alpha() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(var(--foo) / 0.4));\n"),
                "a {\
         \n  value: oklch(var(--foo)/0.4);\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn no_alpha() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(var(--foo)));\n"),
                "a {\
         \n  value: oklch(var(--foo));\
         \n  type: string;\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn t1_of_2() {
        assert_eq!(
        runner().ok(
            "// var() is substituted before parsing, so it may contain multiple arguments.\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(var(--foo) 2deg));\n"
        ),
        "a {\
         \n  value: oklch(var(--foo) 2deg);\
         \n  type: string;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn t2_of_2() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1% var(--foo)));\n"),
            "a {\
         \n  value: oklch(1% var(--foo));\
         \n  type: string;\
         \n}\n"
        );
    }
}
mod var {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn arg_1() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(var(--foo) 0.2 3deg));\n"),
            "a {\
         \n  value: oklch(var(--foo) 0.2 3deg);\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_2() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1% var(--foo) 3deg));\n"),
            "a {\
         \n  value: oklch(1% var(--foo) 3deg);\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_3() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1% 0.2 var(--foo)));\n"),
            "a {\
         \n  value: oklch(1% 0.2 var(--foo));\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_4() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklch(1% 0.2 3deg / var(--foo)));\n"),
            "a {\
         \n  value: oklch(1% 0.2 3deg/var(--foo));\
         \n  type: string;\
         \n}\n"
        );
    }
}
