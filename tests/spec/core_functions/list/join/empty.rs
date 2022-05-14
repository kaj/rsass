//! Tests auto-converted from "sass-spec/spec/core_functions/list/join/empty.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("empty")
}

mod both {
    #[allow(unused)]
    use super::runner;

    mod comma {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn first() {
            assert_eq!(
                runner().ok("@import \"core_functions/list/utils\";\n\
             \n$result: join($empty-comma-list, ());\
             \na {\
             \n  value: inspect($result);\
             \n  separator: real-separator($result);\
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
                runner().ok("@import \"core_functions/list/utils\";\n\
             \n$result: join((), $empty-comma-list);\
             \na {\
             \n  value: inspect($result);\
             \n  separator: real-separator($result);\
             \n}\n"),
                "a {\
         \n  value: ();\
         \n  separator: comma;\
         \n}\n"
            );
        }
    }
    mod slash {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn first() {
            assert_eq!(
                runner().ok("@import \"core_functions/list/utils\";\n\
             \n$empty-slash-list: join((), (), $separator: slash);\
             \n$result: join($empty-slash-list, ());\
             \na {\
             \n  value: inspect($result);\
             \n  separator: real-separator($result);\
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
                runner().ok("@import \"core_functions/list/utils\";\n\
             \n$empty-slash-list: join((), (), $separator: slash);\
             \n$result: join((), $empty-slash-list);\
             \na {\
             \n  value: inspect($result);\
             \n  separator: real-separator($result);\
             \n}\n"),
                "a {\
         \n  value: ();\
         \n  separator: slash;\
         \n}\n"
            );
        }
    }
    mod space {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn first() {
            assert_eq!(
                runner().ok("@import \"core_functions/list/utils\";\n\
             \n$result: join($empty-space-list, ());\
             \na {\
             \n  value: inspect($result);\
             \n  separator: real-separator($result);\
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
                runner().ok("@import \"core_functions/list/utils\";\n\
             \n$result: join((), $empty-space-list);\
             \na {\
             \n  value: inspect($result);\
             \n  separator: real-separator($result);\
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
            "@import \"core_functions/list/utils\";\n\
             \n$result: join((), ());\
             \na {\
             \n  value: inspect($result);\n\
             \n  // `join()` should always produce a real separator, even when the inputs have\
             \n  // undecided separators. It should default to `space`.\
             \n  separator: real-separator($result);\
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
    #[allow(unused)]
    use super::runner;

    #[test]
    fn comma() {
        assert_eq!(
            runner().ok("@import \"core_functions/list/utils\";\
             \na {b: join($empty-comma-list, 1 2 3)}\n"),
            "a {\
         \n  b: 1, 2, 3;\
         \n}\n"
        );
    }
    #[test]
    fn slash() {
        assert_eq!(
            runner()
                .ok("$empty-slash-list: join((), (), $separator: slash);\
             \na {b: join($empty-slash-list, 1 2 3)}\n"),
            "a {\
         \n  b: 1 / 2 / 3;\
         \n}\n"
        );
    }
    #[test]
    fn space() {
        assert_eq!(
            runner().ok("@import \"core_functions/list/utils\";\
             \na {b: join($empty-space-list, (1, 2, 3))}\n"),
            "a {\
         \n  b: 1 2 3;\
         \n}\n"
        );
    }
    mod undecided {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn and_comma() {
            assert_eq!(
                runner().ok("a {b: join((), (1, 2, 3))}\n"),
                "a {\
         \n  b: 1, 2, 3;\
         \n}\n"
            );
        }
        #[test]
        fn and_slash() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: join((), list.slash(1, 2, 3))}\n"),
                "a {\
         \n  b: 1 / 2 / 3;\
         \n}\n"
            );
        }
        #[test]
        fn and_space() {
            assert_eq!(
                runner().ok("a {b: join((), 1 2 3)}\n"),
                "a {\
         \n  b: 1 2 3;\
         \n}\n"
            );
        }
    }
}
mod map {
    #[allow(unused)]
    use super::runner;

    mod first {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn comma() {
            assert_eq!(
                runner().ok("@import \"core_functions/list/utils\";\
             \na {b: join($empty-map, (1, 2, 3))}\n"),
                "a {\
         \n  b: 1, 2, 3;\
         \n}\n"
            );
        }
        #[test]
        fn slash() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@import \"core_functions/list/utils\";\
             \na {b: join($empty-map, list.slash(1, 2, 3))}\n"),
                "a {\
         \n  b: 1 / 2 / 3;\
         \n}\n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                runner().ok("@import \"core_functions/list/utils\";\
             \na {b: join($empty-map, 1 2 3)}\n"),
                "a {\
         \n  b: 1 2 3;\
         \n}\n"
            );
        }
        #[test]
        fn undecided() {
            assert_eq!(
                runner().ok("@import \"core_functions/list/utils\";\n\
             \n$result: join($empty-map, ());\
             \na {\
             \n  value: inspect($result);\
             \n  separator: real-separator($result);\
             \n}\n"),
                "a {\
         \n  value: ();\
         \n  separator: space;\
         \n}\n"
            );
        }
    }
    mod second {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn comma() {
            assert_eq!(
                runner().ok("@import \"core_functions/list/utils\";\
             \na {b: join((1, 2, 3), $empty-map)}\n"),
                "a {\
         \n  b: 1, 2, 3;\
         \n}\n"
            );
        }
        #[test]
        fn slash() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@import \"core_functions/list/utils\";\
             \na {b: join(list.slash(1, 2, 3), $empty-map)}\n"),
                "a {\
         \n  b: 1 / 2 / 3;\
         \n}\n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                runner().ok("@import \"core_functions/list/utils\";\
             \na {b: join(1 2 3, $empty-map)}\n"),
                "a {\
         \n  b: 1 2 3;\
         \n}\n"
            );
        }
        #[test]
        fn undecided() {
            assert_eq!(
                runner().ok("@import \"core_functions/list/utils\";\n\
             \n$result: join($empty-map, ());\
             \na {\
             \n  value: inspect($result);\
             \n  separator: real-separator($result);\
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
    #[allow(unused)]
    use super::runner;

    #[test]
    fn comma() {
        assert_eq!(
            runner().ok("@import \"core_functions/list/utils\";\
             \na {b: join(1 2 3, $empty-comma-list)}\n"),
            "a {\
         \n  b: 1 2 3;\
         \n}\n"
        );
    }
    #[test]
    fn slash() {
        assert_eq!(
            runner()
                .ok("$empty-slash-list: join((), (), $separator: slash);\
             \na {b: join(1 2 3, $empty-slash-list)}\n"),
            "a {\
         \n  b: 1 2 3;\
         \n}\n"
        );
    }
    #[test]
    fn space() {
        assert_eq!(
            runner().ok("@import \"core_functions/list/utils\";\
             \na {b: join((1, 2, 3), $empty-space-list)}\n"),
            "a {\
         \n  b: 1, 2, 3;\
         \n}\n"
        );
    }
    mod undecided {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn comma() {
            assert_eq!(
                runner().ok("a {b: join((1, 2, 3), ())}\n"),
                "a {\
         \n  b: 1, 2, 3;\
         \n}\n"
            );
        }
        #[test]
        fn slash() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: join(list.slash(1, 2, 3), ())}\n"),
                "a {\
         \n  b: 1 / 2 / 3;\
         \n}\n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                runner().ok("a {b: join(1 2 3, ())}\n"),
                "a {\
         \n  b: 1 2 3;\
         \n}\n"
            );
        }
    }
}
