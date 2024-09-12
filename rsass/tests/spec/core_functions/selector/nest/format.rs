//! Tests auto-converted from "sass-spec/spec/core_functions/selector/nest/format.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("format")
}

mod format {
    #[allow(unused)]
    use super::runner;

    mod input {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn initial() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest((c, d e), \"f\")}\n"),
                "a {\
         \n  b: c f, d e f;\
         \n}\n"
            );
        }
        #[test]
        fn later() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c\", (d, e f))}\n"),
                "a {\
         \n  b: c d, c e f;\
         \n}\n"
            );
        }
    }
}
