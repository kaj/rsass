//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/one_arg/special_functions/no_alpha.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no_alpha")
}

mod attr {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn arg_1() {
        assert_eq!(
            runner().ok("a {b: hsl(attr(c, %) 2% 3%)}\n"),
            "a {\
         \n  b: hsl(attr(c, %), 2%, 3%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_2() {
        assert_eq!(
            runner().ok("a {b: hsl(1 attr(c, %) 3%)}\n"),
            "a {\
         \n  b: hsl(1, attr(c, %), 3%);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn arg_3() {
        assert_eq!(
            runner().ok("a {b: hsl(1 2% attr(c, %))}\n"),
            "a {\
         \n  b: hsl(1, 2%, attr(c, %));\
         \n}\n"
        );
    }
}
mod calc {
    use super::runner;

    mod calculation {
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("a {b: hsl(calc(1px + 1%) 2% 3%)}\n"),
                "a {\
         \n  b: hsl(calc(1px + 1%), 2%, 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: hsl(1 calc(1px + 1%) 3%)}\n"),
                "a {\
         \n  b: hsl(1, calc(1px + 1%), 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: hsl(1 2% calc(1px + 1%))}\n"),
                "a {\
         \n  b: hsl(1, 2%, calc(1px + 1%));\
         \n}\n"
            );
        }
    }
    mod string {
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: hsl(string.unquote(\"calc(1)\") 2% 3%)}\n"),
                "a {\
         \n  b: hsl(calc(1), 2%, 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: hsl(1 string.unquote(\"calc(2%)\") 3%)}\n"),
                "a {\
         \n  b: hsl(1, calc(2%), 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: hsl(1 2% string.unquote(\"calc(3%)\"))}\n"),
                "a {\
         \n  b: hsl(1, 2%, calc(3%));\
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
        fn arg_1() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: hsl(string.unquote(\"clamp(1, 2, 3)\") 2% 3%)}\n"),
                "a {\
         \n  b: hsl(clamp(1, 2, 3), 2%, 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: hsl(1 string.unquote(\"clamp(2%, 3%, 4%)\") 3%)}\n"),
                "a {\
         \n  b: hsl(1, clamp(2%, 3%, 4%), 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: hsl(1 2% string.unquote(\"clamp(3%, 4%, 5%)\"))}\n"),
                "a {\
         \n  b: hsl(1, 2%, clamp(3%, 4%, 5%));\
         \n}\n"
            );
        }
    }
}
mod env {
    use super::runner;

    #[test]
    fn arg_1() {
        assert_eq!(
            runner().ok("a {b: hsl(env(--foo) 2% 3%)}\n"),
            "a {\
         \n  b: hsl(env(--foo), 2%, 3%);\
         \n}\n"
        );
    }
    #[test]
    fn arg_2() {
        assert_eq!(
            runner().ok("a {b: hsl(1 env(--foo) 3%)}\n"),
            "a {\
         \n  b: hsl(1, env(--foo), 3%);\
         \n}\n"
        );
    }
    #[test]
    fn arg_3() {
        assert_eq!(
            runner().ok("a {b: hsl(1 2% env(--foo))}\n"),
            "a {\
         \n  b: hsl(1, 2%, env(--foo));\
         \n}\n"
        );
    }
}
mod max {
    use super::runner;

    mod string {
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: hsl(string.unquote(\"max(1)\") 2% 3%)}\n"),
                "a {\
         \n  b: hsl(max(1), 2%, 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: hsl(1 string.unquote(\"max(2%)\") 3%)}\n"),
                "a {\
         \n  b: hsl(1, max(2%), 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: hsl(1 2% string.unquote(\"max(3%)\"))}\n"),
                "a {\
         \n  b: hsl(1, 2%, max(3%));\
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
        fn arg_1() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: hsl(string.unquote(\"min(1)\") 2% 3%)}\n"),
                "a {\
         \n  b: hsl(min(1), 2%, 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: hsl(1 string.unquote(\"min(2%)\") 3%)}\n"),
                "a {\
         \n  b: hsl(1, min(2%), 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("@use \"sass:string\";\
             \na {b: hsl(1 2% string.unquote(\"min(3%)\"))}\n"),
                "a {\
         \n  b: hsl(1, 2%, min(3%));\
         \n}\n"
            );
        }
    }
}
mod multi {
    use super::runner;

    mod argument {
        use super::runner;

        mod var {
            use super::runner;

            mod arg_1 {
                use super::runner;

                mod of {
                    use super::runner;

                    #[test]
                    fn arg_1() {
                        assert_eq!(
                            runner().ok("a {b: hsl(var(--foo))}\n"),
                            "a {\
         \n  b: hsl(var(--foo));\
         \n}\n"
                        );
                    }
                    #[test]
                    fn arg_2() {
                        assert_eq!(
        runner().ok(
            "// var() is substituted before parsing, so it may contain multiple arguments.\
             \na {b: hsl(var(--foo) 50%)}\n"
        ),
        "a {\
         \n  b: hsl(var(--foo) 50%);\
         \n}\n"
    );
                    }
                }
            }
            mod arg_2 {
                use super::runner;

                mod of {
                    use super::runner;

                    #[test]
                    fn arg_2() {
                        assert_eq!(
                            runner().ok("a {b: hsl(0 var(--foo))}\n"),
                            "a {\
         \n  b: hsl(0 var(--foo));\
         \n}\n"
                        );
                    }
                }
            }
        }
    }
}
mod var {
    use super::runner;

    #[test]
    fn arg_1() {
        assert_eq!(
            runner().ok("a {b: hsl(var(--foo) 2% 3%)}\n"),
            "a {\
         \n  b: hsl(var(--foo), 2%, 3%);\
         \n}\n"
        );
    }
    #[test]
    fn arg_2() {
        assert_eq!(
            runner().ok("a {b: hsl(1 var(--foo) 3%)}\n"),
            "a {\
         \n  b: hsl(1, var(--foo), 3%);\
         \n}\n"
        );
    }
    #[test]
    fn arg_3() {
        assert_eq!(
            runner().ok("a {b: hsl(1 2% var(--foo))}\n"),
            "a {\
         \n  b: hsl(1, 2%, var(--foo));\
         \n}\n"
        );
    }
}
