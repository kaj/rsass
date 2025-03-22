//! Tests auto-converted from "sass-spec/spec/core_functions/list/join/single.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("single")
}

mod both {
    use super::runner;

    mod comma {
        use super::runner;

        #[test]
        fn first() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"core_functions/list/utils\";\
             \na {b: list.join((1,), [2])}\n"),
                "a {\
         \n  b: 1, 2;\
         \n}\n"
            );
        }
        #[test]
        fn last() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"core_functions/list/utils\";\n\
             \na {b: list.join([1], (2,))}\n"),
                "a {\
         \n  b: [1, 2];\
         \n}\n"
            );
        }
    }
    mod slash {
        use super::runner;

        #[test]
        fn first() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join(list.join(1, (), $separator: slash), [2])}\n"),
                "a {\
         \n  b: 1 / 2;\
         \n}\n"
            );
        }
        #[test]
        fn last() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join([1], list.join(2, (), $separator: slash))}\n"),
                "a {\
         \n  b: [1 / 2];\
         \n}\n"
            );
        }
    }
    mod space {
        use super::runner;

        #[test]
        fn first() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"core_functions/list/utils\";\
             \na {b: list.join(utils.with-separator(1, space), [2])}\n"),
                "a {\
         \n  b: 1 2;\
         \n}\n"
            );
        }
        #[test]
        fn last() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"core_functions/list/utils\";\
             \na {b: list.join([1], utils.with-separator(2, space))}\n"),
                "a {\
         \n  b: [1 2];\
         \n}\n"
            );
        }
    }
    #[test]
    fn undecided() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join([1], [2])}\n"),
            "a {\
         \n  b: [1 2];\
         \n}\n"
        );
    }
}
mod first {
    use super::runner;

    #[test]
    fn comma() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join((1,), 2 3 4)}\n"),
            "a {\
         \n  b: 1, 2, 3, 4;\
         \n}\n"
        );
    }
    #[test]
    fn slash() {
        assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \na {b: list.join(list.join(1, (), $separator: slash), 2 3 4)}\n"
        ),
        "a {\
         \n  b: 1 / 2 / 3 / 4;\
         \n}\n"
    );
    }
    #[test]
    fn space() {
        assert_eq!(
            runner().ok(
                "@use \"sass:list\";\
             \n@use \"core_functions/list/utils\";\
             \na {b: list.join(utils.with-separator(1, space), (2, 3, 4))}\n"
            ),
            "a {\
         \n  b: 1 2 3 4;\
         \n}\n"
        );
    }
    mod undecided {
        use super::runner;

        #[test]
        fn and_comma() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join([1], (2, 3, 4))}\n"),
                "a {\
         \n  b: [1, 2, 3, 4];\
         \n}\n"
            );
        }
        #[test]
        fn and_slash() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join([1], list.slash(2, 3, 4))}\n"),
                "a {\
         \n  b: [1 / 2 / 3 / 4];\
         \n}\n"
            );
        }
        #[test]
        fn and_space() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join([1], 2 3 4)}\n"),
                "a {\
         \n  b: [1 2 3 4];\
         \n}\n"
            );
        }
    }
}
mod non_list {
    use super::runner;

    #[test]
    fn both() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join(c, d)}\n"),
            "a {\
         \n  b: c d;\
         \n}\n"
        );
    }
    mod first {
        use super::runner;

        #[test]
        fn comma() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join(c, (d, e))}\n"),
                "a {\
         \n  b: c, d, e;\
         \n}\n"
            );
        }
        #[test]
        fn slash() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join(c, list.slash(d, e))}\n"),
                "a {\
         \n  b: c / d / e;\
         \n}\n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(list.join(c, d e))}\n"),
                "a {\
         \n  b: c d e;\
         \n}\n"
            );
        }
        #[test]
        fn undecided() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"core_functions/list/utils\";\n\
             \n$result: list.join(c, ());\
             \na {\
             \n  value: meta.inspect($result);\
             \n  type: meta.type-of($result);\
             \n  separator: utils.real-separator($result);\
             \n}\n"),
                "a {\
         \n  value: c;\
         \n  type: list;\
         \n  separator: space;\
         \n}\n"
            );
        }
    }
    mod second {
        use super::runner;

        #[test]
        fn comma() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join((c, d), e)}\n"),
                "a {\
         \n  b: c, d, e;\
         \n}\n"
            );
        }
        #[test]
        fn slash() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join(list.slash(c, d), e)}\n"),
                "a {\
         \n  b: c / d / e;\
         \n}\n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(list.join(c d, e))}\n"),
                "a {\
         \n  b: c d e;\
         \n}\n"
            );
        }
        #[test]
        fn undecided() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"core_functions/list/utils\";\n\
             \n$result: list.join((), c);\
             \na {\
             \n  value: meta.inspect($result);\
             \n  type: meta.type-of($result);\
             \n  separator: utils.real-separator($result);\
             \n}\n"),
                "a {\
         \n  value: c;\
         \n  type: list;\
         \n  separator: space;\
         \n}\n"
            );
        }
    }
}
mod second {
    use super::runner;

    #[test]
    fn comma() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join(1 2 3, (4,))}\n"),
            "a {\
         \n  b: 1 2 3 4;\
         \n}\n"
        );
    }
    #[test]
    fn slash() {
        assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \na {b: list.join(1 2 3, list.join(4, (), $separator: slash))}\n"
        ),
        "a {\
         \n  b: 1 2 3 4;\
         \n}\n"
    );
    }
    #[test]
    fn space() {
        assert_eq!(
            runner().ok(
                "@use \"sass:list\";\
             \n@use \"core_functions/list/utils\";\
             \na {b: list.join((1, 2, 3), utils.with-separator(4, space))}\n"
            ),
            "a {\
         \n  b: 1, 2, 3, 4;\
         \n}\n"
        );
    }
    mod undecided {
        use super::runner;

        #[test]
        fn comma() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join((1, 2, 3), [4])}\n"),
                "a {\
         \n  b: 1, 2, 3, 4;\
         \n}\n"
            );
        }
        #[test]
        fn slash() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join(list.slash(1, 2, 3), [4])}\n"),
                "a {\
         \n  b: 1 / 2 / 3 / 4;\
         \n}\n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join(1 2 3, [4])}\n"),
                "a {\
         \n  b: 1 2 3 4;\
         \n}\n"
            );
        }
    }
}
