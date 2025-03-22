//! Tests auto-converted from "sass-spec/spec/core_functions/meta/inspect/list/nested.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("nested")
}

mod bracketed {
    use super::runner;

    mod in_comma {
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect([[1, 2], [3, 4]]);\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
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
                runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect(((1, 2), (3, 4)));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
                "a {\
         \n  value: (1, 2), (3, 4);\
         \n  type: string;\
         \n}\n"
            );
        }
    }
    mod in_slash {
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n$result: meta.inspect(list.join([[1, 2], [3, 4]], (), $separator: slash));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
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
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:list\";\
             \n$result: meta.inspect(list.slash([1, 2], [3, 4]));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
                "a {\
         \n  value: [1, 2] / [3, 4];\
         \n  type: string;\
         \n}\n"
            );
        }
    }
    mod in_space {
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect([[1, 2] [3, 4]]);\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
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
                runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect([1, 2] [3, 4]);\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
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
    use super::runner;

    mod in_comma {
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect([(1, 2), (3, 4)]);\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
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
                runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect(((1, 2), (3, 4)));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
                "a {\
         \n  value: (1, 2), (3, 4);\
         \n  type: string;\
         \n}\n"
            );
        }
    }
    mod in_slash {
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n$result: meta.inspect(list.join([(1, 2), (3, 4)], (), $separator: slash));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
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
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:list\";\
             \n$result: meta.inspect(list.slash((1, 2), (3, 4)));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
                "a {\
         \n  value: (1, 2) / (3, 4);\
         \n  type: string;\
         \n}\n"
            );
        }
    }
    mod in_space {
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect([(1, 2) (3, 4)]);\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
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
                runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect((1, 2) (3, 4));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
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
    use super::runner;

    mod in_comma {
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect([(), ()]);\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
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
                runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect(((), ()));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
                "a {\
         \n  value: (), ();\
         \n  type: string;\
         \n}\n"
            );
        }
    }
    mod in_slash {
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n$result: meta.inspect(list.join([(), ()], (), $separator: slash));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
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
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:list\";\
             \n$result: meta.inspect(list.slash((), ()));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
                "a {\
         \n  value: () / ();\
         \n  type: string;\
         \n}\n"
            );
        }
    }
    mod in_space {
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect([() ()]);\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
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
                runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect(() ());\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
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
    use super::runner;

    #[test]
    fn bracketed() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \na {b: meta.inspect([[] []])}\n"),
            "a {\
         \n  b: [[] []];\
         \n}\n"
        );
    }
    #[test]
    fn unbracketed() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \na {b: meta.inspect(([] []))}\n"),
            "a {\
         \n  b: [] [];\
         \n}\n"
        );
    }
}
mod space {
    use super::runner;

    mod in_comma {
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect([1 2, 3 4]);\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
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
                runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect((1 2, 3 4));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
                "a {\
         \n  value: 1 2, 3 4;\
         \n  type: string;\
         \n}\n"
            );
        }
    }
    mod in_slash {
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n$result: meta.inspect(list.join([1 2, 3 4], (), $separator: slash));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
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
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:list\";\
             \n$result: meta.inspect(list.slash(1 2, 3 4));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
                "a {\
         \n  value: 1 2 / 3 4;\
         \n  type: string;\
         \n}\n"
            );
        }
    }
    mod in_space {
        use super::runner;

        #[test]
        fn bracketed() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect([(1 2) (3 4)]);\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
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
                runner().ok("@use \"sass:meta\";\
             \n$result: meta.inspect((1 2) (3 4));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
                "a {\
         \n  value: (1 2) (3 4);\
         \n  type: string;\
         \n}\n"
            );
        }
    }
}
