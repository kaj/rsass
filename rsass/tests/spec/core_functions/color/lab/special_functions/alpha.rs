//! Tests auto-converted from "sass-spec/spec/core_functions/color/lab/special_functions/alpha.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("alpha")
}

mod attr {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn arg_1() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(attr(c, %) 2 3 / 0.4));\n"),
            "a {\
         \n  value: lab(attr(c, %) 2 3/0.4);\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_2() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% attr(c, %) 3 / 0.4));\n"),
            "a {\
         \n  value: lab(1% attr(c, %) 3/0.4);\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_3() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 attr(c, %) / 0.4));\n"),
            "a {\
         \n  value: lab(1% 2 attr(c, %)/0.4);\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_4() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 3 / attr(c, %)));\n"),
            "a {\
         \n  value: lab(1% 2 3/attr(c, %));\
         \n  type: string;\
         \n}\n"
        );
    }
}
mod calc {
    use super::runner;

    mod calculation {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn arg_1() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(calc(1px + 1%) 2 3 / 0.4));\n"),
                "a {\
         \n  value: lab(calc(1px + 1%) 2 3/0.4);\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn arg_2() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% calc(1px + 1%) 3 / 0.4));\n"),
                "a {\
         \n  value: lab(1% calc(1px + 1%) 3/0.4);\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn arg_3() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 calc(1px + 1%) / 0.4));\n"),
                "a {\
         \n  value: lab(1% 2 calc(1px + 1%)/0.4);\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn arg_4() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 3 / calc(1px + 1%)));\n"),
                "a {\
         \n  value: lab(1% 2 3/calc(1px + 1%));\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn with_slash() {
            assert_eq!(
                runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 3 / calc(var(--a) / 2)));\n"),
                "a {\
         \n  value: lab(1% 2 3/calc(var(--a) / 2));\
         \n  type: string;\
         \n}\n"
            );
        }
    }
    mod string {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn arg_1() {
            assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(string.unquote(\"calc(1%)\") 2 3 / 0.4));\n"
        ),
        "a {\
         \n  value: lab(calc(1%) 2 3/0.4);\
         \n  type: string;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn arg_2() {
            assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% string.unquote(\"calc(2)\") 3 / 0.4));\n"
        ),
        "a {\
         \n  value: lab(1% calc(2) 3/0.4);\
         \n  type: string;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn arg_3() {
            assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 string.unquote(\"calc(3)\") / 0.4));\n"
        ),
        "a {\
         \n  value: lab(1% 2 calc(3)/0.4);\
         \n  type: string;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn arg_4() {
            assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 3 / string.unquote(\"calc(0.4)\")));\n"
        ),
        "a {\
         \n  value: lab(1% 2 3/calc(0.4));\
         \n  type: string;\
         \n}\n"
    );
        }
    }
}
mod clamp {
    use super::runner;

    mod string {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn arg_1() {
            assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(string.unquote(\"clamp(1%, 2, 3)\") 2 3 / 0.4));\n"
        ),
        "a {\
         \n  value: lab(clamp(1%, 2, 3) 2 3/0.4);\
         \n  type: string;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn arg_2() {
            assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% string.unquote(\"clamp(2, 3, 4)\") 3 / 0.4));\n"
        ),
        "a {\
         \n  value: lab(1% clamp(2, 3, 4) 3/0.4);\
         \n  type: string;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn arg_3() {
            assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 string.unquote(\"clamp(3, 4, 5)\") / 0.4));\n"
        ),
        "a {\
         \n  value: lab(1% 2 clamp(3, 4, 5)/0.4);\
         \n  type: string;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn arg_4() {
            assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 3 / string.unquote(\"clamp(0.4, 0.5, 0.6)\")));\n"
        ),
        "a {\
         \n  value: lab(1% 2 3/clamp(0.4, 0.5, 0.6));\
         \n  type: string;\
         \n}\n"
    );
        }
    }
}
mod env {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn arg_1() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(env(--foo) 2 3 / 0.4));\n"),
            "a {\
         \n  value: lab(env(--foo) 2 3/0.4);\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_2() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% env(--foo) 3 / 0.4));\n"),
            "a {\
         \n  value: lab(1% env(--foo) 3/0.4);\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_3() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 env(--foo) / 0.4));\n"),
            "a {\
         \n  value: lab(1% 2 env(--foo)/0.4);\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_4() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 3 / env(--foo)));\n"),
            "a {\
         \n  value: lab(1% 2 3/env(--foo));\
         \n  type: string;\
         \n}\n"
        );
    }
}
mod max {
    use super::runner;

    mod string {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn arg_1() {
            assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(string.unquote(\"max(1%)\") 2 3 / 0.4));\n"
        ),
        "a {\
         \n  value: lab(max(1%) 2 3/0.4);\
         \n  type: string;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn arg_2() {
            assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% string.unquote(\"max(2)\") 3 / 0.4));\n"
        ),
        "a {\
         \n  value: lab(1% max(2) 3/0.4);\
         \n  type: string;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn arg_3() {
            assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 string.unquote(\"max(3)\") / 0.4));\n"
        ),
        "a {\
         \n  value: lab(1% 2 max(3)/0.4);\
         \n  type: string;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn arg_4() {
            assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 3 / string.unquote(\"max(0.4)\")));\n"
        ),
        "a {\
         \n  value: lab(1% 2 3/max(0.4));\
         \n  type: string;\
         \n}\n"
    );
        }
    }
}
mod min {
    use super::runner;

    mod string {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn arg_1() {
            assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(string.unquote(\"min(1%)\") 2 3 / 0.4));\n"
        ),
        "a {\
         \n  value: lab(min(1%) 2 3/0.4);\
         \n  type: string;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn arg_2() {
            assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% string.unquote(\"min(2)\") 3 / 0.4));\n"
        ),
        "a {\
         \n  value: lab(1% min(2) 3/0.4);\
         \n  type: string;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn arg_3() {
            assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 string.unquote(\"min(3)\") / 0.4));\n"
        ),
        "a {\
         \n  value: lab(1% 2 min(3)/0.4);\
         \n  type: string;\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn arg_4() {
            assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 3 / string.unquote(\"min(0.4)\")));\n"
        ),
        "a {\
         \n  value: lab(1% 2 3/min(0.4));\
         \n  type: string;\
         \n}\n"
    );
        }
    }
}
mod multi_argument_var {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn t1_of_1() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(var(--foo) / 0.4));\n"),
            "a {\
         \n  value: lab(var(--foo)/0.4);\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn t1_of_2() {
        assert_eq!(
        runner().ok(
            "// var() is substituted before parsing, so it may contain multiple arguments.\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(var(--foo) 2 / 0.4));\n"
        ),
        "a {\
         \n  value: lab(var(--foo) 2/0.4);\
         \n  type: string;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn t2_of_2() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% var(--foo) / 0.4));\n"),
            "a {\
         \n  value: lab(1% var(--foo)/0.4);\
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
             \n@include utils.inspect(lab(var(--foo) 2 3 / 0.4));\n"),
            "a {\
         \n  value: lab(var(--foo) 2 3/0.4);\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_2() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% var(--foo) 3 / 0.4));\n"),
            "a {\
         \n  value: lab(1% var(--foo) 3/0.4);\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_3() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 var(--foo) / 0.4));\n"),
            "a {\
         \n  value: lab(1% 2 var(--foo)/0.4);\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_4() {
        assert_eq!(
            runner().ok("@use \'core_functions/color/utils\';\
             \n@include utils.inspect(lab(1% 2 3 / var(--foo)));\n"),
            "a {\
         \n  value: lab(1% 2 3/var(--foo));\
         \n  type: string;\
         \n}\n"
        );
    }
}
