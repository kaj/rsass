//! Tests auto-converted from "sass-spec/spec/core_functions/selector/parse/structure.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("structure")
}

mod decomposed {
    #[allow(unused)]
    use super::runner;

    mod complex {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn mixed() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse(c \"d\" e)}\n"),
                "a {\
         \n  b: c d e;\
         \n}\n"
            );
        }
        #[test]
        fn quoted() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse(\"c\" \"d\" \"e\")}\n"),
                "a {\
         \n  b: c d e;\
         \n}\n"
            );
        }
        #[test]
        fn unquoted() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse(c d e)}\n"),
                "a {\
         \n  b: c d e;\
         \n}\n"
            );
        }
    }
    mod full {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn mixed() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse((c \"d\", e \"f\"))}\n"),
                "a {\
         \n  b: c d, e f;\
         \n}\n"
            );
        }
        #[test]
        fn quoted() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse((\"c\" \"d\", \"e\" \"f\"))}\n"),
                "a {\
         \n  b: c d, e f;\
         \n}\n"
            );
        }
        #[test]
        fn unquoted() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse((c d, e f))}\n"),
                "a {\
         \n  b: c d, e f;\
         \n}\n"
            );
        }
    }
    mod middle {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn mixed() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse(c \"d, e\" f)}\n"),
                "a {\
         \n  b: c d, e f;\
         \n}\n"
            );
        }
        #[test]
        fn quoted() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse(\"c\" \"d, e\" \"f\")}\n"),
                "a {\
         \n  b: c d, e f;\
         \n}\n"
            );
        }
        #[test]
        fn unquoted() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \n@use \"sass:string\";\
             \na {b: selector.parse(c string.unquote(\"d, e\") f)}\n"),
                "a {\
         \n  b: c d, e f;\
         \n}\n"
            );
        }
    }
    mod partial {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn mixed() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \n@use \"sass:string\";\
             \na {b: selector.parse((c d, string.unquote(\"e f\")))}\n"),
                "a {\
         \n  b: c d, e f;\
         \n}\n"
            );
        }
        #[test]
        fn quoted() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse((\"c d\", \"e f\"))}\n"),
                "a {\
         \n  b: c d, e f;\
         \n}\n"
            );
        }
        #[test]
        fn unquoted() {
            assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \n@use \"sass:string\";\
             \na {b: selector.parse((string.unquote(\"c d\"), string.unquote(\"e f\")))}\n"
        ),
        "a {\
         \n  b: c d, e f;\
         \n}\n"
    );
        }
    }
}
mod full_string {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn quoted() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.parse(\"c d, e f\")}\n"),
            "a {\
         \n  b: c d, e f;\
         \n}\n"
        );
    }
    #[test]
    fn unquoted() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \n@use \"sass:string\";\
             \na {b: selector.parse(string.unquote(\"c d, e f\"))}\n"),
            "a {\
         \n  b: c d, e f;\
         \n}\n"
        );
    }
}
