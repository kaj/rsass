//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/format.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("format")
}

mod input {
    use super::runner;

    mod multiple_extendees {
        use super::runner;

        #[test]
        fn compound() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c.d\", \"c.d\", \".e\")}\n"),
                "a {\
         \n  b: c.d, .e;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn list() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c.d\", \"c, .d\", \".e\")}\n"),
                "a {\
         \n  b: c.d, .e;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn list_of_compound() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c.d.e.f\", \"c.d, .e.f\", \".g\")}\n"),
                "a {\
         \n  b: c.d.e.f, .g;\
         \n}\n"
            );
        }
    }
    mod non_string {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn extendee() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c.d\", (c, \".d\"), \".e\")}\n"),
                "a {\
         \n  b: c.d, .e;\
         \n}\n"
            );
        }
        #[test]
        fn extender() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c\", \"c\", (d, e f))}\n"),
                "a {\
         \n  b: c, d, e f;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn selector() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend((c, d c), \"c\", \"e\")}\n"),
                "a {\
         \n  b: c, e, d c;\
         \n}\n"
            );
        }
    }
}
#[test]
fn output() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \n$result: selector.extend(\"c d, e f\", \"g\", \"g\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (\"c\" \"d\", \"e\" \"f\");\
             \n}\n"),
        "a {\
         \n  result: c d, e f;\
         \n  structure: true;\
         \n}\n"
    );
}
