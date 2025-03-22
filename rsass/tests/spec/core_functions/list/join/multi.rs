//! Tests auto-converted from "sass-spec/spec/core_functions/list/join/multi.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("multi")
}

mod auto {
    use super::runner;

    #[test]
    fn bracketed() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join(c d, e f, $bracketed: auto)}\n"),
            "a {\
         \n  b: c d e f;\
         \n}\n"
        );
    }
    #[test]
    fn separator() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join((c, d), e f, $separator: auto)}\n"),
            "a {\
         \n  b: c, d, e, f;\
         \n}\n"
        );
    }
}
mod bracketed {
    use super::runner;

    #[test]
    fn and_separator() {
        assert_eq!(
            runner().ok(
                "@use \"sass:list\";\
             \na {b: list.join(c, d, $bracketed: true, $separator: comma)}\n"
            ),
            "a {\
         \n  b: [c, d];\
         \n}\n"
        );
    }
    #[test]
    fn both() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join([c d], [e f])}\n"),
            "a {\
         \n  b: [c d e f];\
         \n}\n"
        );
    }
    #[test]
    fn test_false() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join([c], [d], $bracketed: false)}\n"),
            "a {\
         \n  b: c d;\
         \n}\n"
        );
    }
    #[test]
    fn falsey() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join([c], [d], $bracketed: null)}\n"),
            "a {\
         \n  b: c d;\
         \n}\n"
        );
    }
    #[test]
    fn first() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join([c d], e f)}\n"),
            "a {\
         \n  b: [c d e f];\
         \n}\n"
        );
    }
    #[test]
    fn positional() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join(c, d, comma, true)}\n"),
            "a {\
         \n  b: [c, d];\
         \n}\n"
        );
    }
    #[test]
    fn second() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join(c d, [e f])}\n"),
            "a {\
         \n  b: c d e f;\
         \n}\n"
        );
    }
    #[test]
    fn test_true() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join(c, d, $bracketed: true)}\n"),
            "a {\
         \n  b: [c d];\
         \n}\n"
        );
    }
    #[test]
    fn truthy() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join(c, d, $bracketed: e)}\n"),
            "a {\
         \n  b: [c d];\
         \n}\n"
        );
    }
}
mod comma {
    use super::runner;

    #[test]
    fn both() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join((c, d), (e, f))}\n"),
            "a {\
         \n  b: c, d, e, f;\
         \n}\n"
        );
    }
    #[test]
    fn first() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join((c, d), e f)}\n"),
            "a {\
         \n  b: c, d, e, f;\
         \n}\n"
        );
    }
    #[test]
    fn second() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join(c d, (e, f))}\n"),
            "a {\
         \n  b: c d e f;\
         \n}\n"
        );
    }
    mod separator {
        use super::runner;

        #[test]
        fn forces_comma() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join(c, d, $separator: comma)}\n"),
                "a {\
         \n  b: c, d;\
         \n}\n"
            );
        }
        #[test]
        fn forces_not_comma() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join((c, d), (e, f), $separator: space)}\n"),
                "a {\
         \n  b: c d e f;\
         \n}\n"
            );
        }
    }
}
mod map {
    use super::runner;

    #[test]
    fn both() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join((c: d, e: f), (g: h, i: j))}\n"),
            "a {\
         \n  b: c d, e f, g h, i j;\
         \n}\n"
        );
    }
    mod first {
        use super::runner;

        #[test]
        fn comma() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join((c: d, e: f), (g, h))}\n"),
                "a {\
         \n  b: c d, e f, g, h;\
         \n}\n"
            );
        }
        #[test]
        fn slash() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join((c: d, e: f), list.slash(g, h))}\n"),
                "a {\
         \n  b: c d, e f, g, h;\
         \n}\n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \na {b: meta.inspect(list.join((c: d, e: f), g h))}\n"),
                "a {\
         \n  b: c d, e f, g, h;\
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
             \na {b: list.join((c, d), (e: f, g: h))}\n"),
                "a {\
         \n  b: c, d, e f, g h;\
         \n}\n"
            );
        }
        #[test]
        fn slash() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join(list.slash(c, d), (e: f, g: h))}\n"),
                "a {\
         \n  b: c / d / e f / g h;\
         \n}\n"
            );
        }
        #[test]
        fn space() {
            assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n// Use inspect() to prove that the map is converted to a list of pairs.\
             \na {b: meta.inspect(list.join(c d, (e: f, g: h)))}\n"
        ),
        "a {\
         \n  b: c d (e f) (g h);\
         \n}\n"
    );
        }
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \na {b: list.join($list1: a b, $list2: c d, $separator: comma, $bracketed: true)}\n"
        ),
        "a {\
         \n  b: [a, b, c, d];\
         \n}\n"
    );
}
mod slash {
    use super::runner;

    #[test]
    fn both() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join(list.slash(c, d), list.slash(e, f))}\n"),
            "a {\
         \n  b: c / d / e / f;\
         \n}\n"
        );
    }
    #[test]
    fn first() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join(list.slash(c, d), e f)}\n"),
            "a {\
         \n  b: c / d / e / f;\
         \n}\n"
        );
    }
    #[test]
    fn second() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join(c d, list.slash(e, f))}\n"),
            "a {\
         \n  b: c d e f;\
         \n}\n"
        );
    }
    mod separator {
        use super::runner;

        #[test]
        fn forces_not_slash() {
            assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \na {b: list.join(list.slash(c, d), list.slash(e, f), $separator: space)}\n"
        ),
        "a {\
         \n  b: c d e f;\
         \n}\n"
    );
        }
        #[test]
        fn forces_slash() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join(c, d, $separator: slash)}\n"),
                "a {\
         \n  b: c / d;\
         \n}\n"
            );
        }
    }
}
mod space {
    use super::runner;

    #[test]
    fn both() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.join(c d, e f)}\n"),
            "a {\
         \n  b: c d e f;\
         \n}\n"
        );
    }
    mod separator {
        use super::runner;

        #[test]
        fn forces_not_space() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join(c d, e f, $separator: comma)}\n"),
                "a {\
         \n  b: c, d, e, f;\
         \n}\n"
            );
        }
        #[test]
        fn forces_space() {
            assert_eq!(
                runner().ok("@use \"sass:list\";\
             \na {b: list.join(c, d, $separator: space)}\n"),
                "a {\
         \n  b: c d;\
         \n}\n"
            );
        }
    }
}
