//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/complex/without_unification.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("without_unification")
}

mod leading_combinator {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn both() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"~ .c\", \".c\", \"+ .d\")}\n"),
            "a {\
         \n  b: ~ .c;\
         \n}\n"
        );
    }
    #[test]
    fn extender() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c\", \".c\", \"+ .d\")}\n"),
            "a {\
         \n  b: .c, + .d;\
         \n}\n"
        );
    }
    #[test]
    fn selector() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"> .c\", \".c\", \".d\")}\n"),
            "a {\
         \n  b: > .c, > .d;\
         \n}\n"
        );
    }
}
mod multiple_combinators {
    use super::runner;

    mod leading {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn extender() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c\", \".c\", \"+ ~ .d\")}\n"),
                "a {\
         \n  b: .c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn selector() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"> + .c\", \".c\", \".d\")}\n"),
                "a {\
         \n  b: > + .c;\
         \n}\n"
            );
        }
    }
    mod middle {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn extender() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c\", \".c\", \".d ~ + .e\")}\n"),
                "a {\
         \n  b: .c;\
         \n}\n"
            );
        }
        #[test]
        fn selector() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c ~ ~ .d\", \".c\", \".e\")}\n"),
                "a {\
         \n  b: .c ~ ~ .d;\
         \n}\n"
            );
        }
    }
    mod trailing {
        use super::runner;

        #[test]
        fn extender() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c\", \".c\", \".d + +\")}\n"),
                "a {\
         \n  b: .c;\
         \n}\n"
            );
        }
        #[test]
        fn selector() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c > ~\", \".c\", \".d\")}\n"),
                "a {\
         \n  b: .c > ~;\
         \n}\n"
            );
        }
    }
}
mod parent {
    use super::runner;

    mod with_grandparent {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn complex() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c .d .e\", \".d\", \".f .g\")}\n"),
                "a {\
         \n  b: .c .d .e, .c .f .g .e, .f .c .g .e;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn list() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c .d .e\", \".d\", \".f, .g\")}\n"),
                "a {\
         \n  b: .c .d .e, .c .f .e, .c .g .e;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn simple() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c .d .e\", \".d\", \".f\")}\n"),
                "a {\
         \n  b: .c .d .e, .c .f .e;\
         \n}\n"
            );
        }
    }
    mod without_grandparent {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn complex() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c .d\", \".c\", \".e .f\")}\n"),
                "a {\
         \n  b: .c .d, .e .f .d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn list() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c .d\", \".c\", \".e, .f\")}\n"),
                "a {\
         \n  b: .c .d, .e .d, .f .d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn simple() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c .d\", \".c\", \".e\")}\n"),
                "a {\
         \n  b: .c .d, .e .d;\
         \n}\n"
            );
        }
    }
}
mod trailing_combinator {
    use super::runner;

    #[test]
    fn both() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c ~\", \".c\", \".d >\")}\n"),
            "a {\
         \n  b: .c ~;\
         \n}\n"
        );
    }
    mod extender {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn child() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c .d\", \".c\", \".e >\")}\n"),
                "a {\
         \n  b: .c .d, .e > .d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn next_sibling() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c .d\", \".c\", \".e +\")}\n"),
                "a {\
         \n  b: .c .d, .e + .d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn sibling() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c .d\", \".c\", \".e ~\")}\n"),
                "a {\
         \n  b: .c .d, .e ~ .d;\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn selector() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c +\", \".c\", \".d\")}\n"),
            "a {\
         \n  b: .c +, .d +;\
         \n}\n"
        );
    }
}
