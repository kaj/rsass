//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla/one_arg/special_functions.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    mod calc {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("a {b: hsla(calc(1) 2% 3% / 0.4)}\n"),
                "a {\
         \n  b: hsla(calc(1), 2%, 3%, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: hsla(1 calc(2%) 3% / 0.4)}\n"),
                "a {\
         \n  b: hsla(1, calc(2%), 3%, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: hsla(1 2% calc(3%) / 0.4)}\n"),
                "a {\
         \n  b: hsla(1 2% calc(3%)/0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                runner().ok("a {b: hsla(1 2% 3% / calc(0.4))}\n"),
                "a {\
         \n  b: hsla(1 2% 3%/calc(0.4));\
         \n}\n"
            );
        }
    }
    mod clamp {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("a {b: hsla(clamp(1, 2, 3) 2% 3% / 0.4)}\n"),
                "a {\
         \n  b: hsla(clamp(1, 2, 3), 2%, 3%, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: hsla(1 clamp(2%, 3%, 4%) 3% / 0.4)}\n"),
                "a {\
         \n  b: hsla(1, clamp(2%, 3%, 4%), 3%, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: hsla(1 2% clamp(3%, 4%, 5%) / 0.4)}\n"),
                "a {\
         \n  b: hsla(1 2% clamp(3%, 4%, 5%)/0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                runner().ok("a {b: hsla(1 2% 3% / clamp(0.4, 0.5, 0.6))}\n"),
                "a {\
         \n  b: hsla(1 2% 3%/clamp(0.4, 0.5, 0.6));\
         \n}\n"
            );
        }
    }
    mod env {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("a {b: hsla(env(--foo) 2% 3% / 0.4)}\n"),
                "a {\
         \n  b: hsla(env(--foo), 2%, 3%, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: hsla(1 env(--foo) 3% / 0.4)}\n"),
                "a {\
         \n  b: hsla(1, env(--foo), 3%, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: hsla(1 2% env(--foo) / 0.4)}\n"),
                "a {\
         \n  b: hsla(1 2% env(--foo)/0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                runner().ok("a {b: hsla(1 2% 3% / env(--foo))}\n"),
                "a {\
         \n  b: hsla(1 2% 3%/env(--foo));\
         \n}\n"
            );
        }
    }
    mod max {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("a {b: hsla(max(1) 2% 3% / 0.4)}\n"),
                "a {\
         \n  b: hsla(max(1), 2%, 3%, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: hsla(1 max(2%) 3% / 0.4)}\n"),
                "a {\
         \n  b: hsla(1, max(2%), 3%, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: hsla(1 2% max(3%) / 0.4)}\n"),
                "a {\
         \n  b: hsla(1 2% max(3%)/0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                runner().ok("a {b: hsla(1 2% 3% / max(0.4))}\n"),
                "a {\
         \n  b: hsla(1 2% 3%/max(0.4));\
         \n}\n"
            );
        }
    }
    mod min {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("a {b: hsla(min(1) 2% 3% / 0.4)}\n"),
                "a {\
         \n  b: hsla(min(1), 2%, 3%, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: hsla(1 min(2%) 3% / 0.4)}\n"),
                "a {\
         \n  b: hsla(1, min(2%), 3%, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: hsla(1 2% min(3%) / 0.4)}\n"),
                "a {\
         \n  b: hsla(1 2% min(3%)/0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                runner().ok("a {b: hsla(1 2% 3% / min(0.4))}\n"),
                "a {\
         \n  b: hsla(1 2% 3%/min(0.4));\
         \n}\n"
            );
        }
    }
    mod multi_argument_var {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn t1_of_1() {
            assert_eq!(
                runner().ok("a {b: hsla(var(--foo) / 0.4)}\n"),
                "a {\
         \n  b: hsla(var(--foo)/0.4);\
         \n}\n"
            );
        }
        #[test]
        fn t1_of_2() {
            assert_eq!(
        runner().ok(
            "// var() is substituted before parsing, so it may contain multiple arguments.\
             \na {b: hsla(var(--foo) 50% / 0.4)}\n"
        ),
        "a {\
         \n  b: hsla(var(--foo) 50%/0.4);\
         \n}\n"
    );
        }
        #[test]
        fn t2_of_2() {
            assert_eq!(
                runner().ok("a {b: hsla(0 var(--foo) / 0.4)}\n"),
                "a {\
         \n  b: hsla(0 var(--foo)/0.4);\
         \n}\n"
            );
        }
    }
    mod var {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("a {b: hsla(var(--foo) 2% 3% / 0.4)}\n"),
                "a {\
         \n  b: hsla(var(--foo), 2%, 3%, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: hsla(1 var(--foo) 3% / 0.4)}\n"),
                "a {\
         \n  b: hsla(1, var(--foo), 3%, 0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: hsla(1 2% var(--foo) / 0.4)}\n"),
                "a {\
         \n  b: hsla(1 2% var(--foo)/0.4);\
         \n}\n"
            );
        }
        #[test]
        fn arg_4() {
            assert_eq!(
                runner().ok("a {b: hsla(1 2% 3% / var(--foo))}\n"),
                "a {\
         \n  b: hsla(1 2% 3%/var(--foo));\
         \n}\n"
            );
        }
    }
}
mod no_alpha {
    #[allow(unused)]
    use super::runner;

    mod calc {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("a {b: hsla(calc(1) 2% 3%)}\n"),
                "a {\
         \n  b: hsla(calc(1), 2%, 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: hsla(1 calc(2%) 3%)}\n"),
                "a {\
         \n  b: hsla(1, calc(2%), 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: hsla(1 2% calc(3%))}\n"),
                "a {\
         \n  b: hsla(1, 2%, calc(3%));\
         \n}\n"
            );
        }
    }
    mod clamp {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("a {b: hsla(clamp(1, 2, 3) 2% 3%)}\n"),
                "a {\
         \n  b: hsla(clamp(1, 2, 3), 2%, 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: hsla(1 clamp(2%, 3%, 4%) 3%)}\n"),
                "a {\
         \n  b: hsla(1, clamp(2%, 3%, 4%), 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: hsla(1 2% clamp(3%, 4%, 5%))}\n"),
                "a {\
         \n  b: hsla(1, 2%, clamp(3%, 4%, 5%));\
         \n}\n"
            );
        }
    }
    mod env {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("a {b: hsla(env(--foo) 2% 3%)}\n"),
                "a {\
         \n  b: hsla(env(--foo), 2%, 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: hsla(1 env(--foo) 3%)}\n"),
                "a {\
         \n  b: hsla(1, env(--foo), 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: hsla(1 2% env(--foo))}\n"),
                "a {\
         \n  b: hsla(1, 2%, env(--foo));\
         \n}\n"
            );
        }
    }
    mod max {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("a {b: hsla(max(1) 2% 3%)}\n"),
                "a {\
         \n  b: hsla(max(1), 2%, 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: hsla(1 max(2%) 3%)}\n"),
                "a {\
         \n  b: hsla(1, max(2%), 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: hsla(1 2% max(3%))}\n"),
                "a {\
         \n  b: hsla(1, 2%, max(3%));\
         \n}\n"
            );
        }
    }
    mod min {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("a {b: hsla(min(1) 2% 3%)}\n"),
                "a {\
         \n  b: hsla(min(1), 2%, 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: hsla(1 min(2%) 3%)}\n"),
                "a {\
         \n  b: hsla(1, min(2%), 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: hsla(1 2% min(3%))}\n"),
                "a {\
         \n  b: hsla(1, 2%, min(3%));\
         \n}\n"
            );
        }
    }
    mod multi_argument_var {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn t1_of_1() {
            assert_eq!(
                runner().ok("a {b: hsla(var(--foo))}\n"),
                "a {\
         \n  b: hsla(var(--foo));\
         \n}\n"
            );
        }
        #[test]
        fn t1_of_2() {
            assert_eq!(
        runner().ok(
            "// var() is substituted before parsing, so it may contain multiple arguments.\
             \na {b: hsla(var(--foo) 50%)}\n"
        ),
        "a {\
         \n  b: hsla(var(--foo) 50%);\
         \n}\n"
    );
        }
        #[test]
        fn t2_of_2() {
            assert_eq!(
                runner().ok("a {b: hsla(0 var(--foo))}\n"),
                "a {\
         \n  b: hsla(0 var(--foo));\
         \n}\n"
            );
        }
    }
    mod var {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn arg_1() {
            assert_eq!(
                runner().ok("a {b: hsla(var(--foo) 2% 3%)}\n"),
                "a {\
         \n  b: hsla(var(--foo), 2%, 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().ok("a {b: hsla(1 var(--foo) 3%)}\n"),
                "a {\
         \n  b: hsla(1, var(--foo), 3%);\
         \n}\n"
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().ok("a {b: hsla(1 2% var(--foo))}\n"),
                "a {\
         \n  b: hsla(1, 2%, var(--foo));\
         \n}\n"
            );
        }
    }
}
mod slash_list {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: hsla(list.slash(1 2% 3%, var(--c)))}\n"),
            "a {\
         \n  b: hsla(1, 2%, 3%, var(--c));\
         \n}\n"
        );
    }
    #[test]
    fn channels() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: hsla(list.slash(var(--foo), 0.4))}\n"),
            "a {\
         \n  b: hsla(var(--foo) / 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn some_channels() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: hsla(list.slash(1 var(--foo), 0.4))}\n"),
            "a {\
         \n  b: hsla(1 var(--foo) / 0.4);\
         \n}\n"
        );
    }
}
