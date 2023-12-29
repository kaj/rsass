//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/complex/combinators/initial.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("initial")
}

#[test]
fn different() {
    assert_eq!(
        runner().ok(
            "a {b: inspect(selector-unify(\"+ ~ > .c\", \"+ > ~ ~ .d\"))}\n"
        ),
        "a {\
         \n  b: null;\
         \n}\n"
    );
}
mod only_one {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn selector1() {
        assert_eq!(
            runner().ok("a {b: selector-unify(\"> .c\", \".d\")}\n"),
            "a {\
         \n  b: > .c.d;\
         \n}\n"
        );
    }
    #[test]
    fn selector2() {
        assert_eq!(
            runner().ok("a {b: selector-unify(\".c\", \"~ .d\")}\n"),
            "a {\
         \n  b: ~ .c.d;\
         \n}\n"
        );
    }
}
#[test]
fn same() {
    assert_eq!(
        runner().ok("a {b: selector-unify(\"+ .c\", \"+ .d\")}\n"),
        "a {\
         \n  b: + .c.d;\
         \n}\n"
    );
}
mod supersequence {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn contiguous() {
        assert_eq!(
            runner().ok(
                "a {b: selector-unify(\"+ ~ > .c\", \"> + ~ > > .d\")}\n"
            ),
            ""
        );
    }
    #[test]
    fn non_contiguous() {
        assert_eq!(
            runner().ok(
                "a {b: selector-unify(\"+ ~ > .c\", \"+ > ~ ~ > .d\")}\n"
            ),
            ""
        );
    }
}
