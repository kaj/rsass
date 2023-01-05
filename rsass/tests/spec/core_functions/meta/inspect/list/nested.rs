//! Tests auto-converted from "sass-spec/spec/core_functions/meta/inspect/list/nested.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("nested")
}

mod bracketed {
    #[allow(unused)]
    use super::runner;

    mod in_comma {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
                runner().ok("$result: inspect([[1, 2], [3, 4]]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: [[1, 2], [3, 4]];\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        fn unbracketed() {
            assert_eq!(
                runner().ok("$result: inspect(((1, 2), (3, 4)));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: (1, 2), (3, 4);\
         \n  type: string;\
         \n}\n"
            );
        }
    }
    mod in_slash {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
        runner().ok(
            "$result: inspect(join([[1, 2], [3, 4]], (), $separator: slash));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"
        ),
        "a {\
         \n  value: [[1, 2] / [3, 4]];\
         \n  type: string;\
         \n}\n"
    );
        }
        #[test]
        fn unbracketed() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n$result: inspect(list.slash([1, 2], [3, 4]));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: [1, 2] / [3, 4];\
         \n  type: string;\
         \n}\n"
            );
        }
    }
    mod in_space {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
                runner().ok("$result: inspect([[1, 2] [3, 4]]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: [[1, 2] [3, 4]];\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        fn unbracketed() {
            assert_eq!(
                runner().ok("$result: inspect([1, 2] [3, 4]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: [1, 2] [3, 4];\
         \n  type: string;\
         \n}\n"
            );
        }
    }
}
mod comma {
    #[allow(unused)]
    use super::runner;

    mod in_comma {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
                runner().ok("$result: inspect([(1, 2), (3, 4)]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: [(1, 2), (3, 4)];\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        fn unbracketed() {
            assert_eq!(
                runner().ok("$result: inspect(((1, 2), (3, 4)));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: (1, 2), (3, 4);\
         \n  type: string;\
         \n}\n"
            );
        }
    }
    mod in_slash {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
        runner().ok(
            "$result: inspect(join([(1, 2), (3, 4)], (), $separator: slash));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"
        ),
        "a {\
         \n  value: [(1, 2) / (3, 4)];\
         \n  type: string;\
         \n}\n"
    );
        }
        #[test]
        fn unbracketed() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n$result: inspect(list.slash((1, 2), (3, 4)));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: (1, 2) / (3, 4);\
         \n  type: string;\
         \n}\n"
            );
        }
    }
    mod in_space {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
                runner().ok("$result: inspect([(1, 2) (3, 4)]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: [(1, 2) (3, 4)];\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        fn unbracketed() {
            assert_eq!(
                runner().ok("$result: inspect((1, 2) (3, 4));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: (1, 2) (3, 4);\
         \n  type: string;\
         \n}\n"
            );
        }
    }
}
mod empty {
    #[allow(unused)]
    use super::runner;

    mod in_comma {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
                runner().ok("$result: inspect([(), ()]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: [(), ()];\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        fn unbracketed() {
            assert_eq!(
                runner().ok("$result: inspect(((), ()));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: (), ();\
         \n  type: string;\
         \n}\n"
            );
        }
    }
    mod in_slash {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
        runner().ok(
            "$result: inspect(join([(), ()], (), $separator: slash));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"
        ),
        "a {\
         \n  value: [() / ()];\
         \n  type: string;\
         \n}\n"
    );
        }
        #[test]
        fn unbracketed() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n$result: inspect(list.slash((), ()));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: () / ();\
         \n  type: string;\
         \n}\n"
            );
        }
    }
    mod in_space {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
                runner().ok("$result: inspect([() ()]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: [() ()];\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        fn unbracketed() {
            assert_eq!(
                runner().ok("$result: inspect(() ());\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: () ();\
         \n  type: string;\
         \n}\n"
            );
        }
    }
}
mod empty_bracketed {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn bracketed() {
        assert_eq!(
            runner().ok("a {b: inspect([[] []])}\n"),
            "a {\
         \n  b: [[] []];\
         \n}\n"
        );
    }
    #[test]
    fn unbracketed() {
        assert_eq!(
            runner().ok("a {b: inspect(([] []))}\n"),
            "a {\
         \n  b: [] [];\
         \n}\n"
        );
    }
}
mod space {
    #[allow(unused)]
    use super::runner;

    mod in_comma {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
                runner().ok("$result: inspect([1 2, 3 4]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: [1 2, 3 4];\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        fn unbracketed() {
            assert_eq!(
                runner().ok("$result: inspect((1 2, 3 4));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: 1 2, 3 4;\
         \n  type: string;\
         \n}\n"
            );
        }
    }
    mod in_slash {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
        runner().ok(
            "$result: inspect(join([1 2, 3 4], (), $separator: slash));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"
        ),
        "a {\
         \n  value: [1 2 / 3 4];\
         \n  type: string;\
         \n}\n"
    );
        }
        #[test]
        fn unbracketed() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n$result: inspect(list.slash(1 2, 3 4));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: 1 2 / 3 4;\
         \n  type: string;\
         \n}\n"
            );
        }
    }
    mod in_space {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
                runner().ok("$result: inspect([(1 2) (3 4)]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: [(1 2) (3 4)];\
         \n  type: string;\
         \n}\n"
            );
        }
        #[test]
        fn unbracketed() {
            assert_eq!(
                runner().ok("$result: inspect((1 2) (3 4));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  value: (1 2) (3 4);\
         \n  type: string;\
         \n}\n"
            );
        }
    }
}
