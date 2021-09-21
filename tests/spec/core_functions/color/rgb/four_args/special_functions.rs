//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/four_args/special_functions.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod calc {
    #[allow(unused)]
    use super::runner;

    mod calculation {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("a {b: rgb(calc(1px + 1%), 2, 3, 0.4)}\n"),
                "a {\
         \n  b: rgb(calc(1px + 1%), 2, 3, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: rgb(1, calc(1px + 1%), 3, 0.4)}\n"),
                "a {\
         \n  b: rgb(1, calc(1px + 1%), 3, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: rgb(1, 2, calc(1px + 1%), 0.4)}\n"),
                "a {\
         \n  b: rgb(1, 2, calc(1px + 1%), 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                runner().ok("a {b: rgb(1, 2, 3, calc(1px + 1%))}\n"),
                "a {\
         \n  b: rgb(1, 2, 3, calc(1px + 1%));\
         \n}\n"
            );
        }
    }
    mod string {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("a {b: rgb(unquote(\"calc(1)\"), 2, 3, 0.4)}\n"),
                "a {\
         \n  b: rgb(calc(1), 2, 3, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: rgb(1, unquote(\"calc(2)\"), 3, 0.4)}\n"),
                "a {\
         \n  b: rgb(1, calc(2), 3, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: rgb(1, 2, unquote(\"calc(3)\"), 0.4)}\n"),
                "a {\
         \n  b: rgb(1, 2, calc(3), 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                runner().ok("a {b: rgb(1, 2, 3, unquote(\"calc(0.4)\"))}\n"),
                "a {\
         \n  b: rgb(1, 2, 3, calc(0.4));\
         \n}\n"
            );
        }
    }
}
mod clamp {
    #[allow(unused)]
    use super::runner;

    mod string {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok(
                    "a {b: rgb(unquote(\"clamp(1, 2, 3)\"), 2, 3, 0.4)}\n"
                ),
                "a {\
         \n  b: rgb(clamp(1, 2, 3), 2, 3, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok(
                    "a {b: rgb(1, unquote(\"clamp(2, 3, 4)\"), 3, 0.4)}\n"
                ),
                "a {\
         \n  b: rgb(1, clamp(2, 3, 4), 3, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok(
                    "a {b: rgb(1, 2, unquote(\"clamp(3, 4, 5)\"), 0.4)}\n"
                ),
                "a {\
         \n  b: rgb(1, 2, clamp(3, 4, 5), 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
        runner().ok(
            "a {b: rgb(1, 2, 3, unquote(\"clamp(0.4, 0.5, 0.6)\"))}\n"
        ),
        "a {\
         \n  b: rgb(1, 2, 3, clamp(0.4, 0.5, 0.6));\
         \n}\n"
    );
        }
    }
}
mod env {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn arg_1() {
        assert_eq!(
            runner().ok("a {b: rgb(env(--foo), 2, 3, 0.4)}\n"),
            "a {\
         \n  b: rgb(env(--foo), 2, 3, 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_2() {
        assert_eq!(
            runner().ok("a {b: rgb(1, env(--foo), 3, 0.4)}\n"),
            "a {\
         \n  b: rgb(1, env(--foo), 3, 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_3() {
        assert_eq!(
            runner().ok("a {b: rgb(1, 2, env(--foo), 0.4)}\n"),
            "a {\
         \n  b: rgb(1, 2, env(--foo), 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_4() {
        assert_eq!(
            runner().ok("a {b: rgb(1, 2, 3, env(--foo))}\n"),
            "a {\
         \n  b: rgb(1, 2, 3, env(--foo));\
         \n}\n"
        );
    }
}
mod max {
    #[allow(unused)]
    use super::runner;

    mod string {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("a {b: rgb(unquote(\"max(1)\"), 2, 3, 0.4)}\n"),
                "a {\
         \n  b: rgb(max(1), 2, 3, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: rgb(1, unquote(\"max(2)\"), 3, 0.4)}\n"),
                "a {\
         \n  b: rgb(1, max(2), 3, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: rgb(1, 2, unquote(\"max(3)\"), 0.4)}\n"),
                "a {\
         \n  b: rgb(1, 2, max(3), 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                runner().ok("a {b: rgb(1, 2, 3, unquote(\"max(0.4)\"))}\n"),
                "a {\
         \n  b: rgb(1, 2, 3, max(0.4));\
         \n}\n"
            );
        }
    }
}
mod min {
    #[allow(unused)]
    use super::runner;

    mod string {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("a {b: rgb(unquote(\"min(1)\"), 2, 3, 0.4)}\n"),
                "a {\
         \n  b: rgb(min(1), 2, 3, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: rgb(1, unquote(\"min(2)\"), 3, 0.4)}\n"),
                "a {\
         \n  b: rgb(1, min(2), 3, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: rgb(1, 2, unquote(\"min(3)\"), 0.4)}\n"),
                "a {\
         \n  b: rgb(1, 2, min(3), 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                runner().ok("a {b: rgb(1, 2, 3, unquote(\"min(0.4)\"))}\n"),
                "a {\
         \n  b: rgb(1, 2, 3, min(0.4));\
         \n}\n"
            );
        }
    }
}
mod var {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn arg_1() {
        assert_eq!(
            runner().ok("a {b: rgb(var(--foo), 2, 3, 0.4)}\n"),
            "a {\
         \n  b: rgb(var(--foo), 2, 3, 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_2() {
        assert_eq!(
            runner().ok("a {b: rgb(1, var(--foo), 3, 0.4)}\n"),
            "a {\
         \n  b: rgb(1, var(--foo), 3, 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_3() {
        assert_eq!(
            runner().ok("a {b: rgb(1, 2, var(--foo), 0.4)}\n"),
            "a {\
         \n  b: rgb(1, 2, var(--foo), 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn arg_4() {
        assert_eq!(
            runner().ok("a {b: rgb(1, 2, 3, var(--foo))}\n"),
            "a {\
         \n  b: rgb(1, 2, 3, var(--foo));\
         \n}\n"
        );
    }
}
