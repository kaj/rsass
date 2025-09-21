//! Tests auto-converted from "sass-spec/spec/css/style_rule.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("style_rule")
}

mod comment {
    use super::runner;

    mod after_selector {
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(
                runner().ok("a /**/ {b: c}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("a //\
             \n  {b: c}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
    }
}
mod declaration {
    use super::runner;

    mod comment {
        use super::runner;

        mod after_colon {
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("a {b: /**/ c}\n"),
                    "a {\
         \n  b: c;\
         \n}\n"
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("a {b: //\
             \n  c}\n"),
                    "a {\
         \n  b: c;\
         \n}\n"
                );
            }
        }
        mod after_value {
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("a {b: c /**/}\n"),
                    "a {\
         \n  b: c;\
         \n}\n"
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("a {b: c //\
             \n  }\n"),
                    "a {\
         \n  b: c;\
         \n}\n"
                );
            }
        }
        mod before_colon {
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("a {b /**/ : c}\n"),
                    "a {\
         \n  b: c;\
         \n}\n"
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("a {b //\
             \n  : c}\n"),
                    "a {\
         \n  b: c;\
         \n}\n"
                );
            }
        }
    }
    mod interleaved {
        use super::runner;

        mod after_style_rule {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn extended_child() {
                assert_eq!(
                    runner().ok(".a {\
             \n  .b {c: d}\
             \n  e: f;\
             \n}\n\
             \n:where(.g) {@extend .b}\n"),
                    ".a .b, .a :where(.g) {\
         \n  c: d;\
         \n}\
         \n.a {\
         \n  e: f;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn extended_parent() {
                assert_eq!(
                    runner().ok(".a {\
             \n  .b {c: d}\
             \n  e: f;\
             \n}\n\
             \n:where(.g) {@extend .a}\n"),
                    ".a .b, :where(.g) .b {\
         \n  c: d;\
         \n}\
         \n.a, :where(.g) {\
         \n  e: f;\
         \n}\n"
                );
            }
            #[test]
            fn higher_specificity() {
                assert_eq!(
                    runner().ok(".a {\
             \n  .b {c: d}\
             \n  e: f;\
             \n}\n"),
                    ".a .b {\
         \n  c: d;\
         \n}\
         \n.a {\
         \n  e: f;\
         \n}\n"
                );
            }
            #[test]
            fn mixed_specificity_child() {
                assert_eq!(
                    runner().ok(".a {\
             \n  .b, :where(.b) {c: d}\
             \n  e: f;\
             \n}\n"),
                    ".a .b, .a :where(.b) {\
         \n  c: d;\
         \n}\
         \n.a {\
         \n  e: f;\
         \n}\n"
                );
            }
            #[test]
            fn mixed_specificity_parent() {
                assert_eq!(
                    runner().ok(".a, :where(.a) {\
             \n  .b {c: d}\
             \n  e: f;\
             \n}\n"),
                    ".a .b, :where(.a) .b {\
         \n  c: d;\
         \n}\
         \n.a, :where(.a) {\
         \n  e: f;\
         \n}\n"
                );
            }
            #[test]
            fn same_specificity() {
                assert_eq!(
                    runner().ok(".a {\
             \n  :where(.b) {c: d}\
             \n  e: f;\
             \n}\n"),
                    ".a :where(.b) {\
         \n  c: d;\
         \n}\
         \n.a {\
         \n  e: f;\
         \n}\n"
                );
            }
        }
        #[test]
        fn around_style_rule() {
            assert_eq!(
                runner().ok(".a {\
             \n  b: c;\
             \n  .d {e: f}\
             \n  g: h;\
             \n}\n"),
                ".a {\
         \n  b: c;\
         \n}\
         \n.a .d {\
         \n  e: f;\
         \n}\
         \n.a {\
         \n  g: h;\
         \n}\n"
            );
        }
        #[test]
        fn before_style_rule() {
            assert_eq!(
                runner().ok(".a {\
             \n  b: c;\
             \n  .d {e: f}\
             \n}\n"),
                ".a {\
         \n  b: c;\
         \n}\
         \n.a .d {\
         \n  e: f;\
         \n}\n"
            );
        }
        #[test]
        fn in_at_rule() {
            assert_eq!(
                runner().ok("@a {\
             \n  .b {\
             \n    .c {d: e}\
             \n    f: g;\
             \n  }\
             \n}\n"),
                "@a {\
         \n  .b .c {\
         \n    d: e;\
         \n  }\
         \n  .b {\
         \n    f: g;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        fn in_bubbled_rule() {
            assert_eq!(
                runner().ok(".a {\
             \n  .b {c: d}\
             \n  @e {f: g}\
             \n}\n"),
                ".a .b {\
         \n  c: d;\
         \n}\
         \n@e {\
         \n  .a {\
         \n    f: g;\
         \n  }\
         \n}\n"
            );
        }
    }
}
