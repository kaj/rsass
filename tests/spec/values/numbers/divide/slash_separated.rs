//! Tests auto-converted from "sass-spec/spec/values/numbers/divide/slash_separated.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod list {
    #[allow(unused)]
    use super::runner;

    mod argument {
        #[allow(unused)]
        use super::runner;

        mod function {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn built_in() {
                assert_eq!(
                    runner().ok("c {d: join(1 2/3 4, ())}\n"),
                    "c {\
         \n  d: 1 2/3 4;\
         \n}\n"
                );
            }
            #[test]
            fn user_defined() {
                assert_eq!(
                    runner().ok("@function a($b) {@return $b}\n\
             \nc {d: a(1 2/3 4)}\n"),
                    "c {\
         \n  d: 1 2/3 4;\
         \n}\n"
                );
            }
        }
        #[test]
        fn mixin() {
            assert_eq!(
                runner().ok("@mixin a($b) {c {d: $b}}\n\
             \n@include a(1 2/3 4);\n"),
                "c {\
         \n  d: 1 2/3 4;\
         \n}\n"
            );
        }
    }
    #[test]
    fn bracketed() {
        assert_eq!(
            runner().ok("a {b: [1 2/3 4]}\n"),
            "a {\
         \n  b: [1 2/3 4];\
         \n}\n"
        );
    }
    #[test]
    fn comma() {
        assert_eq!(
            runner().ok("a {b: 1, 2/3, 4}\n"),
            "a {\
         \n  b: 1, 2/3, 4;\
         \n}\n"
        );
    }
    #[test]
    fn declaration() {
        assert_eq!(
            runner().ok("a {b: 1 2/3 4}\n"),
            "a {\
         \n  b: 1 2/3 4;\
         \n}\n"
        );
    }
    #[test]
    fn interpolation() {
        assert_eq!(
            runner().ok("a {b: #{1 2/3 4}}\n"),
            "a {\
         \n  b: 1 2/3 4;\
         \n}\n"
        );
    }
    #[test]
    fn parenthesized() {
        assert_eq!(
            runner().ok("a {b: (1 2/3 4)}\n"),
            "a {\
         \n  b: 1 2/3 4;\
         \n}\n"
        );
    }
    #[test]
    fn test_return() {
        assert_eq!(
            runner().ok("@function a() {@return 1 2/3 4}\n\
             \nb {c: a()}\n"),
            "b {\
         \n  c: 1 2/3 4;\
         \n}\n"
        );
    }
}
mod value {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn declaration() {
        assert_eq!(
            runner().ok("a {b: 1/2}\n"),
            "a {\
         \n  b: 1/2;\
         \n}\n"
        );
    }
    #[test]
    fn interpolation() {
        assert_eq!(
            runner().ok("a {b: #{1/2}}\n"),
            "a {\
         \n  b: 1/2;\
         \n}\n"
        );
    }
    #[test]
    fn multiple() {
        assert_eq!(
            runner().ok("a {b: 1/2/3/4/5}\n"),
            "a {\
         \n  b: 1/2/3/4/5;\
         \n}\n"
        );
    }
}
