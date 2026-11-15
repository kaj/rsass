//! Tests auto-converted from "sass-spec/spec/callable/arguments/keyword.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("keyword")
}

mod escape {
    use super::runner;

    mod literal {
        use super::runner;

        #[test]
        fn dash_insensitive() {
            assert_eq!(
                runner().ok("@function a($b-c) {@return $b-c}\
             \nd {e: a($b\\5f c: f)}\n"),
                "d {\
         \n  e: f;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn non_normalized() {
            assert_eq!(
                runner().ok("@function a($\\$) {@return $\\$}\
             \nb {c: a($\\24: d)}\n"),
                "b {\
         \n  c: d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn normalized() {
            assert_eq!(
                runner().ok("@function a($\\$) {@return $\\$}\
             \nb {c: a($\\$: d)}\n"),
                "b {\
         \n  c: d;\
         \n}\n"
            );
        }
    }
    mod rest {
        use super::runner;

        mod quoted {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn normalized() {
                assert_eq!(
                    runner().ok("@function a($\\$) {@return $\\$}\
             \nb {c: a((\"\\\\$\": d)...)}\n"),
                    "b {\
         \n  c: d;\
         \n}\n"
                );
            }
        }
        mod unquoted {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn normalized() {
                assert_eq!(
                    runner().ok("@function a($\\$) {@return $\\$}\
             \nb {c: a((\\$: d)...)}\n"),
                    "b {\
         \n  c: d;\
         \n}\n"
                );
            }
        }
    }
}
