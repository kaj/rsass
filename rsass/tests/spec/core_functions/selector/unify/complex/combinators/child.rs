//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/complex/combinators/child.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("child")
}

mod and_child {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn conflict() {
        assert_eq!(
        runner().ok(
            "a {b: inspect(selector-unify(\"#s1-1 > .s1-2\", \"#s2-1 > .s2-2\"))}\n"
        ),
        "a {\
         \n  b: null;\
         \n}\n"
    );
    }
    #[test]
    fn distinct() {
        assert_eq!(
            runner().ok("a {b: selector-unify(\".c > .d\", \".e > .f\")}\n"),
            "a {\
         \n  b: .e.c > .d.f;\
         \n}\n"
        );
    }
    #[test]
    fn overlap() {
        assert_eq!(
        runner().ok(
            "a {b: selector-unify(\".c.s1-1 > .s1-2\", \".c.s2-1 > .s2-2\")}\n"
        ),
        "a {\
         \n  b: .c.s2-1.s1-1 > .s1-2.s2-2;\
         \n}\n"
    );
    }
    #[test]
    fn superselector() {
        assert_eq!(
            runner().ok(
                "a {b: selector-unify(\".c.s1-1 > .s1-2\", \".c > .s2\")}\n"
            ),
            "a {\
         \n  b: .c.s1-1 > .s1-2.s2;\
         \n}\n"
        );
    }
}
mod and_descendant {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn distinct() {
        assert_eq!(
            runner().ok("a {b: selector-unify(\".c > .d\", \".e .f\")}\n"),
            "a {\
         \n  b: .e .c > .d.f;\
         \n}\n"
        );
    }
    #[test]
    fn identical() {
        assert_eq!(
            runner().ok("a {b: selector-unify(\".c > .s1\", \".c .s2\")}\n"),
            "a {\
         \n  b: .c > .s1.s2;\
         \n}\n"
        );
    }
    #[test]
    fn overlap() {
        assert_eq!(
        runner().ok(
            "a {b: selector-unify(\".c.s1-1 > .s1-2\", \".c.s2-1 .s2-2\")}\n"
        ),
        "a {\
         \n  b: .c.s2-1 .c.s1-1 > .s1-2.s2-2;\
         \n}\n"
    );
    }
    #[test]
    fn superselector() {
        assert_eq!(
            runner().ok(
                "a {b: selector-unify(\".c.s1-1 > .s1-2\", \".c .s2\")}\n"
            ),
            "a {\
         \n  b: .c.s1-1 > .s1-2.s2;\
         \n}\n"
        );
    }
}
#[test]
fn and_next_sibling() {
    assert_eq!(
        runner().ok("a {b: selector-unify(\".c > .s1\", \".c + .s2\")}\n"),
        "a {\
         \n  b: .c > .c + .s1.s2;\
         \n}\n"
    );
}
#[test]
fn and_sibling() {
    assert_eq!(
        runner().ok("a {b: selector-unify(\".c > .s1\", \".c ~ .s2\")}\n"),
        "a {\
         \n  b: .c > .c ~ .s1.s2;\
         \n}\n"
    );
}
