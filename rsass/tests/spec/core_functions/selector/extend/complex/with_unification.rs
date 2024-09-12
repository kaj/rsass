//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/complex/with_unification.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("with_unification")
}

mod leading_combinator {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn both() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"~ .c.x\", \".c\", \"+ .d\")}\n"),
            "a {\
         \n  b: ~ .c.x;\
         \n}\n"
        );
    }
    #[test]
    fn extender() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c.x\", \".c\", \"+ .d\")}\n"),
            "a {\
         \n  b: .c.x, + .x.d;\
         \n}\n"
        );
    }
    #[test]
    fn selector() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"> .c.x\", \".c\", \".d\")}\n"),
            "a {\
         \n  b: > .c.x, > .x.d;\
         \n}\n"
        );
    }
}
mod multiple_combinators {
    #[allow(unused)]
    use super::runner;

    mod leading {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn extender() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c.x\", \".c\", \"+ ~ .d\")}\n"),
                "a {\
         \n  b: .c.x;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn selector() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"> + .c.x\", \".c\", \".d\")}\n"),
                "a {\
         \n  b: > + .c.x;\
         \n}\n"
            );
        }
    }
    mod middle {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn extender() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c.x\", \".c\", \".d ~ + .e\")}\n"),
                "a {\
         \n  b: .c.x;\
         \n}\n"
            );
        }
        #[test]
        fn selector() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c.x ~ ~ .d\", \".c\", \".e\")}\n"),
                "a {\
         \n  b: .c.x ~ ~ .d;\
         \n}\n"
            );
        }
    }
    mod trailing {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn extender() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c.x\", \".c\", \".d + +\")}\n"),
                "a {\
         \n  b: .c.x;\
         \n}\n"
            );
        }
        #[test]
        fn selector() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c.x > ~\", \".c\", \".d\")}\n"),
                "a {\
         \n  b: .c.x > ~;\
         \n}\n"
            );
        }
    }
}
mod parent {
    #[allow(unused)]
    use super::runner;

    mod with_grandparent {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn complex() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c .d.x .e\", \".d\", \".f .g\")}\n"),
                "a {\
         \n  b: .c .d.x .e, .c .f .x.g .e, .f .c .x.g .e;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn list() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c .d.x .e\", \".d\", \".f, .g\")}\n"),
                "a {\
         \n  b: .c .d.x .e, .c .x.f .e, .c .x.g .e;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn simple() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c .d.x .e\", \".d\", \".f\")}\n"),
                "a {\
         \n  b: .c .d.x .e, .c .x.f .e;\
         \n}\n"
            );
        }
    }
    mod without_grandparent {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn complex() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c.x .d\", \".c\", \".e .f\")}\n"),
                "a {\
         \n  b: .c.x .d, .e .x.f .d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn list() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c.x .d\", \".c\", \".e, .f\")}\n"),
                "a {\
         \n  b: .c.x .d, .x.e .d, .x.f .d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn simple() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c.x .d\", \".c\", \".e\")}\n"),
                "a {\
         \n  b: .c.x .d, .x.e .d;\
         \n}\n"
            );
        }
    }
}
mod trailing_combinator {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn both() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c.x ~\", \".c\", \".d >\")}\n"),
            "a {\
         \n  b: .c.x ~;\
         \n}\n"
        );
    }
    mod extender {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn child() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c.x .d\", \".c\", \".e >\")}\n"),
                "a {\
         \n  b: .c.x .d, .x.e > .d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn next_sibling() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c.x .d\", \".c\", \".e +\")}\n"),
                "a {\
         \n  b: .c.x .d, .x.e + .d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn sibling() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c.x .d\", \".c\", \".e ~\")}\n"),
                "a {\
         \n  b: .c.x .d, .x.e ~ .d;\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn selector() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c.x +\", \".c\", \".d\")}\n"),
            "a {\
         \n  b: .c.x +, .x.d +;\
         \n}\n"
        );
    }
}
