//! Tests auto-converted from "sass-spec/spec/core_functions/list/join/empty.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("empty")
}

mod both {
    use super::runner;

    mod comma {
        use super::runner;

        #[test]
        fn first() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"core_functions/list/utils\";\n\
             \n$result: list.join(utils.$empty-comma-list, ());\
             \na {\
             \n  value: meta.inspect($result);\
             \n  separator: utils.real-separator($result);\
             \n}\n"),
                "a {\
         \n  value: ();\
         \n  separator: comma;\
         \n}\n"
            );
        }
        #[test]
        fn last() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"core_functions/list/utils\";\n\
             \n$result: list.join((), utils.$empty-comma-list);\
             \na {\
             \n  value: meta.inspect($result);\
             \n  separator: utils.real-separator($result);\
             \n}\n"),
                "a {\
         \n  value: ();\
         \n  separator: comma;\
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
             \n@use \"sass:meta\";\
             \n@use \"core_functions/list/utils\";\n\
             \n$empty-slash-list: list.join((), (), $separator: slash);\
             \n$result: list.join($empty-slash-list, ());\
             \na {\
             \n  value: meta.inspect($result);\
             \n  separator: utils.real-separator($result);\
             \n}\n"),
                "a {\
         \n  value: ();\
         \n  separator: slash;\
         \n}\n"
            );
        }
        #[test]
        fn last() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"core_functions/list/utils\";\n\
             \n$empty-slash-list: list.join((), (), $separator: slash);\
             \n$result: list.join((), $empty-slash-list);\
             \na {\
             \n  value: meta.inspect($result);\
             \n  separator: utils.real-separator($result);\
             \n}\n"),
                "a {\
         \n  value: ();\
         \n  separator: slash;\
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
             \n@use \"sass:meta\";\
             \n@use \"core_functions/list/utils\";\n\
             \n$result: list.join(utils.$empty-space-list, ());\
             \na {\
             \n  value: meta.inspect($result);\
             \n  separator: utils.real-separator($result);\
             \n}\n"),
                "a {\
         \n  value: ();\
         \n  separator: space;\
         \n}\n"
            );
        }
        #[test]
        fn last() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"core_functions/list/utils\";\n\
             \n$result: list.join((), utils.$empty-space-list);\
             \na {\
             \n  value: meta.inspect($result);\
             \n  separator: utils.real-separator($result);\
             \n}\n"),
                "a {\
         \n  value: ();\
         \n  separator: space;\
         \n}\n"
            );
        }
    }
    #[test]
    fn undecided() {
        assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"core_functions/list/utils\";\n\
             \n$result: list.join((), ());\
             \na {\
             \n  value: meta.inspect($result);\n\
             \n  // `join()` should always produce a real separator, even when the inputs have\
             \n  // undecided separators. It should default to `space`.\
             \n  separator: utils.real-separator($result);\
             \n}\n"
        ),
        "a {\
         \n  value: ();\
         \n  separator: space;\
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
             \n@use \"core_functions/list/utils\";\
             \na {b: list.join(utils.$empty-comma-list, 1 2 3)}\n"),
            "a {\
         \n  b: 1, 2, 3;\
         \n}\n"
        );
    }
    #[test]
    fn slash() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n$empty-slash-list: list.join((), (), $separator: slash);\
             \na {b: list.join($empty-slash-list, 1 2 3)}\n"),
            "a {\
         \n  b: 1 / 2 / 3;\
         \n}\n"
        );
    }
    #[test]
    fn space() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"core_functions/list/utils\";\
             \na {b: list.join(utils.$empty-space-list, (1, 2, 3))}\n"),
            "a {\
         \n  b: 1 2 3;\
         \n}\n"
        );
    }
    mod undecided {
        use super::runner;

        #[test]
        fn and_comma() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join((), (1, 2, 3))}\n"),
                "a {\
         \n  b: 1, 2, 3;\
         \n}\n"
            );
        }
        #[test]
        fn and_slash() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join((), list.slash(1, 2, 3))}\n"),
                "a {\
         \n  b: 1 / 2 / 3;\
         \n}\n"
            );
        }
        #[test]
        fn and_space() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join((), 1 2 3)}\n"),
                "a {\
         \n  b: 1 2 3;\
         \n}\n"
            );
        }
    }
}
mod map {
    use super::runner;

    mod first {
        use super::runner;

        #[test]
        fn comma() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"core_functions/list/utils\";\
             \na {b: list.join(utils.$empty-map, (1, 2, 3))}\n"),
                "a {\
         \n  b: 1, 2, 3;\
         \n}\n"
            );
        }
        #[test]
        fn slash() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"core_functions/list/utils\";\
             \na {b: list.join(utils.$empty-map, list.slash(1, 2, 3))}\n"),
                "a {\
         \n  b: 1 / 2 / 3;\
         \n}\n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"core_functions/list/utils\";\
             \na {b: list.join(utils.$empty-map, 1 2 3)}\n"),
                "a {\
         \n  b: 1 2 3;\
         \n}\n"
            );
        }
        #[test]
        fn undecided() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"core_functions/list/utils\";\n\
             \n$result: list.join(utils.$empty-map, ());\
             \na {\
             \n  value: meta.inspect($result);\
             \n  separator: utils.real-separator($result);\
             \n}\n"),
                "a {\
         \n  value: ();\
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
             \n@use \"core_functions/list/utils\";\
             \na {b: list.join((1, 2, 3), utils.$empty-map)}\n"),
                "a {\
         \n  b: 1, 2, 3;\
         \n}\n"
            );
        }
        #[test]
        fn slash() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"core_functions/list/utils\";\
             \na {b: list.join(list.slash(1, 2, 3), utils.$empty-map)}\n"),
                "a {\
         \n  b: 1 / 2 / 3;\
         \n}\n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"core_functions/list/utils\";\
             \na {b: list.join(1 2 3, utils.$empty-map)}\n"),
                "a {\
         \n  b: 1 2 3;\
         \n}\n"
            );
        }
        #[test]
        fn undecided() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@use \"core_functions/list/utils\";\n\
             \n$result: list.join(utils.$empty-map, ());\
             \na {\
             \n  value: meta.inspect($result);\
             \n  separator: utils.real-separator($result);\
             \n}\n"),
                "a {\
         \n  value: ();\
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
             \n@use \"core_functions/list/utils\";\
             \na {b: list.join(1 2 3, utils.$empty-comma-list)}\n"),
            "a {\
         \n  b: 1 2 3;\
         \n}\n"
        );
    }
    #[test]
    fn slash() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n$empty-slash-list: list.join((), (), $separator: slash);\
             \na {b: list.join(1 2 3, $empty-slash-list)}\n"),
            "a {\
         \n  b: 1 2 3;\
         \n}\n"
        );
    }
    #[test]
    fn space() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \n@use \"core_functions/list/utils\";\
             \na {b: list.join((1, 2, 3), utils.$empty-space-list)}\n"),
            "a {\
         \n  b: 1, 2, 3;\
         \n}\n"
        );
    }
    mod undecided {
        use super::runner;

        #[test]
        fn comma() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join((1, 2, 3), ())}\n"),
                "a {\
         \n  b: 1, 2, 3;\
         \n}\n"
            );
        }
        #[test]
        fn slash() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join(list.slash(1, 2, 3), ())}\n"),
                "a {\
         \n  b: 1 / 2 / 3;\
         \n}\n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join(1 2 3, ())}\n"),
                "a {\
         \n  b: 1 2 3;\
         \n}\n"
            );
        }
    }
}
