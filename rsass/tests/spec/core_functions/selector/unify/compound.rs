//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/compound.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("compound")
}

#[test]
fn full_overlap() {
    assert_eq!(
        runner().ok("a {b: selector-unify(\".c.d\", \".c.d\")}\n"),
        "a {\
         \n  b: .c.d;\
         \n}\n"
    );
}
#[test]
fn no_overlap() {
    assert_eq!(
        runner().ok("a {b: selector-unify(\".c.d\", \".e.f\")}\n"),
        "a {\
         \n  b: .c.d.e.f;\
         \n}\n"
    );
}
mod order {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn element_at_start() {
        assert_eq!(
            runner().ok("a {b: selector-unify(\".c\", \"d\")}\n"),
            "a {\
         \n  b: d.c;\
         \n}\n"
        );
    }
    #[test]
    fn preserved_by_default() {
        assert_eq!(
            runner().ok("a {b: selector-unify(\".c.d\", \".e.f\")}\n"),
            "a {\
         \n  b: .c.d.e.f;\
         \n}\n"
        );
    }
    #[test]
    fn pseudo_class_at_end() {
        assert_eq!(
            runner().ok("a {b: selector-unify(\":c\", \".d\")}\n"),
            "a {\
         \n  b: .d:c;\
         \n}\n"
        );
    }
    mod pseudo_element_after_pseudo_class {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn class_first() {
            assert_eq!(
                runner().ok("a {b: selector-unify(\":c\", \"::d\")}\n"),
                "a {\
         \n  b: :c::d;\
         \n}\n"
            );
        }
        #[test]
        fn element_first() {
            assert_eq!(
                runner().ok("a {b: selector-unify(\"::c\", \":d\")}\n"),
                "a {\
         \n  b: :d::c;\
         \n}\n"
            );
        }
    }
    #[test]
    fn pseudo_element_at_end() {
        assert_eq!(
            runner().ok("a {b: selector-unify(\"::c\", \".d\")}\n"),
            "a {\
         \n  b: .d::c;\
         \n}\n"
        );
    }
}
#[test]
fn partial_overlap() {
    assert_eq!(
        runner().ok("a {b: selector-unify(\".c.d\", \".d.e\")}\n"),
        "a {\
         \n  b: .c.d.e;\
         \n}\n"
    );
}
