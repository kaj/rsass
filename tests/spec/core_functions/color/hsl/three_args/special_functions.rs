//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/three_args/special_functions.hrx"

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
                runner().ok("a {b: hsl(calc(1px + 1%), 2%, 3%)}\n"),
                "a {\
         \n  b: hsl(calc(1px + 1%), 2%, 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: hsl(1, calc(1px + 1%), 3%)}\n"),
                "a {\
         \n  b: hsl(1, calc(1px + 1%), 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: hsl(1, 2%, calc(1px + 1%))}\n"),
                "a {\
         \n  b: hsl(1, 2%, calc(1px + 1%));\
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
                runner().ok("a {b: hsl(unquote(\"calc(1)\"), 2%, 3%)}\n"),
                "a {\
         \n  b: hsl(calc(1), 2%, 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: hsl(1, unquote(\"calc(2%)\"), 3%)}\n"),
                "a {\
         \n  b: hsl(1, calc(2%), 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: hsl(1, 2%, unquote(\"calc(3%)\"))}\n"),
                "a {\
         \n  b: hsl(1, 2%, calc(3%));\
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
                runner()
                    .ok("a {b: hsl(unquote(\"clamp(1, 2, 3)\"), 2%, 3%)}\n"),
                "a {\
         \n  b: hsl(clamp(1, 2, 3), 2%, 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok(
                    "a {b: hsl(1, unquote(\"clamp(2%, 3%, 4%)\"), 3%)}\n"
                ),
                "a {\
         \n  b: hsl(1, clamp(2%, 3%, 4%), 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok(
                    "a {b: hsl(1, 2%, unquote(\"clamp(3%, 4%, 5%)\"))}\n"
                ),
                "a {\
         \n  b: hsl(1, 2%, clamp(3%, 4%, 5%));\
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
            runner().ok("a {b: hsl(env(--foo), 2%, 3%)}\n"),
            "a {\
         \n  b: hsl(env(--foo), 2%, 3%);\
         \n}\n"
        );
    }
    #[test]
    fn arg_2() {
        assert_eq!(
            runner().ok("a {b: hsl(1, env(--foo), 3%)}\n"),
            "a {\
         \n  b: hsl(1, env(--foo), 3%);\
         \n}\n"
        );
    }
    #[test]
    fn arg_3() {
        assert_eq!(
            runner().ok("a {b: hsl(1, 2%, env(--foo))}\n"),
            "a {\
         \n  b: hsl(1, 2%, env(--foo));\
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
                runner().ok("a {b: hsl(unquote(\"max(1)\"), 2%, 3%)}\n"),
                "a {\
         \n  b: hsl(max(1), 2%, 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: hsl(1, unquote(\"max(2%)\"), 3%)}\n"),
                "a {\
         \n  b: hsl(1, max(2%), 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: hsl(1, 2%, unquote(\"max(3%)\"))}\n"),
                "a {\
         \n  b: hsl(1, 2%, max(3%));\
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
                runner().ok("a {b: hsl(unquote(\"min(1)\"), 2%, 3%)}\n"),
                "a {\
         \n  b: hsl(min(1), 2%, 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: hsl(1, unquote(\"min(2%)\"), 3%)}\n"),
                "a {\
         \n  b: hsl(1, min(2%), 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: hsl(1, 2%, unquote(\"min(3%)\"))}\n"),
                "a {\
         \n  b: hsl(1, 2%, min(3%));\
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
            runner().ok("a {b: hsl(var(--foo), 2%, 3%)}\n"),
            "a {\
         \n  b: hsl(var(--foo), 2%, 3%);\
         \n}\n"
        );
    }
    #[test]
    fn arg_2() {
        assert_eq!(
            runner().ok("a {b: hsl(1, var(--foo), 3%)}\n"),
            "a {\
         \n  b: hsl(1, var(--foo), 3%);\
         \n}\n"
        );
    }
    #[test]
    fn arg_3() {
        assert_eq!(
            runner().ok("a {b: hsl(1, 2%, var(--foo))}\n"),
            "a {\
         \n  b: hsl(1, 2%, var(--foo));\
         \n}\n"
        );
    }
}
