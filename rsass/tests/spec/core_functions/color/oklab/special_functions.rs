//! Tests auto-converted from "sass-spec/spec/core_functions/color/oklab/special_functions.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("special_functions")
}

mod calculation {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn arg_1() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(calc(1px + 1%) 2 3));\n"),
            "a {\
         \n  value: oklab(calc(1px + 1%) 2 3);\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_2() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% calc(1px + 1%) 3));\n"),
            "a {\
         \n  value: oklab(1% calc(1px + 1%) 3);\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_3() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% 2 calc(1px + 1%)));\n"),
            "a {\
         \n  value: oklab(1% 2 calc(1px + 1%));\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_4() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% 2 3 / calc(1px + 1%)));\n"),
            "a {\
         \n  value: oklab(1% 2 3/calc(1px + 1%));\
         \n  type: string;\
         \n}\n"
        );
    }
}
mod multi_argument_var {
    use super::runner;

    mod t1_of_1 {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn alpha() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(var(--foo) / 0.4));\n"),
                "a {\
         \n  value: oklab(var(--foo)/0.4);\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn no_alpha() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(var(--foo)));\n"),
                "a {\
         \n  value: oklab(var(--foo));\
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
             \n@include utils.inspect(oklab(var(--foo) 2));\n"
        ),
        "a {\
         \n  value: oklab(var(--foo) 2);\
         \n  type: string;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn t2_of_2() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% var(--foo)));\n"),
            "a {\
         \n  value: oklab(1% var(--foo));\
         \n  type: string;\
         \n}\n"
        );
    }
}
mod var {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn arg_1() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(var(--foo) 2 3));\n"),
            "a {\
         \n  value: oklab(var(--foo) 2 3);\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_2() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% var(--foo) 3));\n"),
            "a {\
         \n  value: oklab(1% var(--foo) 3);\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_3() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% 2 var(--foo)));\n"),
            "a {\
         \n  value: oklab(1% 2 var(--foo));\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_4() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(oklab(1% 2 3 / var(--foo)));\n"),
            "a {\
         \n  value: oklab(1% 2 3/var(--foo));\
         \n  type: string;\
         \n}\n"
        );
    }
}
