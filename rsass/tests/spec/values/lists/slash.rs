//! Tests auto-converted from "sass-spec/spec/values/lists/slash.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("slash")
}

mod functions {
    use super::runner;

    #[test]
    fn length() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.length(list.slash(c, d, e, f, g))}\n"),
            "a {\
         \n  b: 5;\
         \n}\n"
        );
    }
    #[test]
    fn nth() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: list.nth(list.slash(c, d, e, f, g), 3)}\n"),
            "a {\
         \n  b: e;\
         \n}\n"
        );
    }
}
mod output {
    use super::runner;

    #[test]
    fn bracketed_slash() {
        assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n// CSS doesn\'t use slash-separated bracketed lists, but Sass supports them in\
             \n// case one day that changes.\
             \na {b: list.join(c d, e f, $separator: slash, $bracketed: true)}\n"
        ),
        "a {\
         \n  b: [c / d / e / f];\
         \n}\n"
    );
    }
    mod nested {
        use super::runner;

        mod comma_in {
            use super::runner;

            #[test]
            fn slash() {
                assert_eq!(
                    runner().ok("@use \"sass:list\";\
             \na {b: list.slash((c, d), (e, f))}\n"),
                    "a {\
         \n  b: c, d / e, f;\
         \n}\n"
                );
            }
        }
        mod test_in {
            use super::runner;

            #[test]
            fn comma() {
                assert_eq!(
                    runner().ok("@use \"sass:list\";\
             \na {b: list.slash(c, d), list.slash(e, f)}\n"),
                    "a {\
         \n  b: c / d, e / f;\
         \n}\n"
                );
            }
            #[test]
            fn slash() {
                assert_eq!(
                    runner().ok("@use \"sass:list\";\
             \na {b: list.slash(list.slash(c, d), list.slash(e, f))}\n"),
                    "a {\
         \n  b: c / d / e / f;\
         \n}\n"
                );
            }
            #[test]
            fn space() {
                assert_eq!(
                    runner().ok("@use \"sass:list\";\
             \na {b: list.slash(c, d) list.slash(e, f)}\n"),
                    "a {\
         \n  b: c / d e / f;\
         \n}\n"
                );
            }
        }
        mod outside {
            use super::runner;

            #[test]
            fn space() {
                assert_eq!(
                    runner().ok("@use \"sass:list\";\
             \na {b: list.slash(c d, e f)}\n"),
                    "a {\
         \n  b: c d / e f;\
         \n}\n"
                );
            }
        }
    }
}
