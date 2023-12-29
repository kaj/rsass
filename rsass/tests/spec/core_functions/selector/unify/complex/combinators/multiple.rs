//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/complex/combinators/multiple.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("multiple")
}

mod in_a_row {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn different() {
        assert_eq!(
        runner().ok(
            "a {b: inspect(selector-unify(\".c + ~ > .d\", \".e + > ~ ~ .f\"))}\n"
        ),
        "a {\
         \n  b: null;\
         \n}\n"
    );
    }
    #[test]
    fn same() {
        assert_eq!(
            runner().ok(
                "a {b: selector-unify(\".c + ~ > .d\", \".e + ~ > .f\")}\n"
            ),
            ""
        );
    }
    mod supersequence {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn contiguous() {
            assert_eq!(
        runner().ok(
            "a {b: selector-unify(\".c + ~ > .d\", \".e > + ~ > > .f\")}\n"
        ),
        ""
    );
        }
        #[test]
        fn non_contiguous() {
            assert_eq!(
        runner().ok(
            "a {b: selector-unify(\".c + ~ > .d\", \".e + > ~ ~ > .f\")}\n"
        ),
        ""
    );
        }
    }
}
#[test]
fn isolated() {
    assert_eq!(
        runner()
            .ok("a {b: selector-unify(\".c > .d + .e\", \".f .g ~ .h\")}\n"),
        "a {\
         \n  b: .f .c > .g ~ .d + .e.h, .f .c > .d.g + .e.h;\
         \n}\n"
    );
}
