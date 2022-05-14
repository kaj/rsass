//! Tests auto-converted from "sass-spec/spec/values/numbers/divide/slash_free/argument.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("argument")
}

mod function {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn built_in() {
        assert_eq!(
            runner().ok("c {d: join(1/2, 3/4)}\n"),
            "c {\
         \n  d: 0.5 0.75;\
         \n}\n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            runner().ok("c {d: join($list1: 1/2, $list2: 3/4)}\n"),
            "c {\
         \n  d: 0.5 0.75;\
         \n}\n"
        );
    }
    mod rest {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn kwargs() {
            assert_eq!(
                runner().ok("c {d: join(1/2..., (\"list2\": 3/4)...)}\n"),
                "c {\
         \n  d: 0.5 0.75;\
         \n}\n"
            );
        }
        #[test]
        fn list() {
            assert_eq!(
                runner().ok("c {d: join(1/2 3/4...)}\n"),
                "c {\
         \n  d: 0.5 0.75;\
         \n}\n"
            );
        }
        #[test]
        fn map() {
            assert_eq!(
                runner()
                    .ok("c {d: join((\"list1\": 1/2, \"list2\": 3/4)...)}\n"),
                "c {\
         \n  d: 0.5 0.75;\
         \n}\n"
            );
        }
        #[test]
        fn single() {
            assert_eq!(
                runner().ok("c {d: join(1/2, 3/4...)}\n"),
                "c {\
         \n  d: 0.5 0.75;\
         \n}\n"
            );
        }
    }
    #[test]
    fn user_defined() {
        assert_eq!(
            runner().ok("@function a($b) {@return 1 $b 2}\n\
             \nc {d: a(1/2)}\n"),
            "c {\
         \n  d: 1 0.5 2;\
         \n}\n"
        );
    }
}
mod test_macro {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn named() {
        assert_eq!(
            runner().ok("c {d: if(true, $if-true: 1/2, $if-false: null)}\n"),
            "c {\
         \n  d: 0.5;\
         \n}\n"
        );
    }
    #[test]
    fn positional() {
        assert_eq!(
            runner().ok("c {d: if(true, 1/2, null)}\n"),
            "c {\
         \n  d: 0.5;\
         \n}\n"
        );
    }
    #[test]
    fn rest() {
        assert_eq!(
            runner().ok("c {d: if(true, 1/2 null...)}\n"),
            "c {\
         \n  d: 0.5;\
         \n}\n"
        );
    }
}
mod mixin {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn default() {
        assert_eq!(
            runner().ok("@mixin a($b: 1/2) {c {d: $b}}\n\
             \n@include a;\n"),
            "c {\
         \n  d: 0.5;\
         \n}\n"
        );
    }
    #[test]
    fn user_defined() {
        assert_eq!(
            runner().ok("@mixin a($b) {c {d: $b}}\n\
             \n@include a(1/2);\n"),
            "c {\
         \n  d: 0.5;\
         \n}\n"
        );
    }
}
