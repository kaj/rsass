//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/complex/combinators/next_sibling.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("next_sibling")
}

#[test]
fn and_child() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c + .s1\", \".c > .s2\")}\n"),
        "a {\
         \n  b: .c > .c + .s1.s2;\
         \n}\n"
    );
}
#[test]
fn and_descendant() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c + .s1\", \".c .s2\")}\n"),
        "a {\
         \n  b: .c .c + .s1.s2;\
         \n}\n"
    );
}
mod and_next_sibling {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn conflict() {
        assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"#s1-1 + .s1-2\", \"#s2-1 + .s2-2\"))}\n"
        ),
        "a {\
         \n  b: null;\
         \n}\n"
    );
    }
    #[test]
    fn distinct() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c + .d\", \".e + .f\")}\n"),
            "a {\
         \n  b: .e.c + .d.f;\
         \n}\n"
        );
    }
    #[test]
    fn overlap() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.unify(\".c.s1-1 + .s1-2\", \".c.s2-1 + .s2-2\")}\n"
        ),
        "a {\
         \n  b: .c.s2-1.s1-1 + .s1-2.s2-2;\
         \n}\n"
    );
    }
    #[test]
    fn superselector() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c.s1-1 + .s1-2\", \".c + .s2\")}\n"),
            "a {\
         \n  b: .c.s1-1 + .s1-2.s2;\
         \n}\n"
        );
    }
}
mod and_sibling {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn conflict() {
        assert_eq!(
            runner().ok(
                "@use \"sass:selector\";\
             \na {b: selector.unify(\"#s1-1 + .s1-2\", \"#s2-1 ~ .s2-2\")}\n"
            ),
            "a {\
         \n  b: #s2-1 ~ #s1-1 + .s1-2.s2-2;\
         \n}\n"
        );
    }
    #[test]
    fn distinct() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c + .d\", \".e ~ .f\")}\n"),
            "a {\
         \n  b: .e ~ .c + .d.f, .c.e + .d.f;\
         \n}\n"
        );
    }
    #[test]
    fn identical() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c + .s1\", \".c ~ .s2\")}\n"),
            "a {\
         \n  b: .c + .s1.s2;\
         \n}\n"
        );
    }
    #[test]
    fn overlap() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.unify(\".c.s1-1 + .s1-2\", \".c.s2-1 ~ .s2-2\")}\n"
        ),
        "a {\
         \n  b: .c.s2-1 ~ .c.s1-1 + .s1-2.s2-2, .c.s1-1.s2-1 + .s1-2.s2-2;\
         \n}\n"
    );
    }
    #[test]
    fn superselector() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c.s1-1 + .s1-2\", \".c ~ .s2\")}\n"),
            "a {\
         \n  b: .c.s1-1 + .s1-2.s2;\
         \n}\n"
        );
    }
}
