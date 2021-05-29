//! Tests auto-converted from "sass-spec/spec/values/lists/slash.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod functions {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn length() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: length(list.slash(c, d, e, f, g))}\n"),
            "a {\
         \n  b: 5;\
         \n}\n"
        );
    }
    #[test]
    fn nth() {
        assert_eq!(
            runner().ok("@use \"sass:list\";\
             \na {b: nth(list.slash(c, d, e, f, g), 3)}\n"),
            "a {\
         \n  b: e;\
         \n}\n"
        );
    }
}
mod output {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn bracketed_slash() {
        assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n// CSS doesn\'t use slash-separated bracketed lists, but Sass supports them in\
             \n// case one day that changes.\
             \na {b: join(c d, e f, $separator: slash, $bracketed: true)}\n"
        ),
        "a {\
         \n  b: [c / d / e / f];\
         \n}\n"
    );
    }
    mod nested {
        #[allow(unused)]
        use super::runner;

        mod comma_in {
            #[allow(unused)]
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
            #[allow(unused)]
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
            #[allow(unused)]
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
