//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/simple/pseudo/arg.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("arg")
}

mod class {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn equal() {
        assert_eq!(
            runner().ok(
                "a {b: selector-extend(\":c(@#$)\", \":c(@#$)\", \"e\")}\n"
            ),
            "a {\
         \n  b: :c(@#$), e;\
         \n}\n"
        );
    }
    mod unequal {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn argument() {
            assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":c(@#$)\", \":c(*&^)\", \"e\")}\n"
        ),
        "a {\
         \n  b: :c(@#$);\
         \n}\n"
    );
        }
        #[test]
        fn has_argument() {
            assert_eq!(
                runner().ok(
                    "a {b: selector-extend(\":c(@#$)\", \":c\", \"e\")}\n"
                ),
                "a {\
         \n  b: :c(@#$);\
         \n}\n"
            );
        }
        #[test]
        fn name() {
            assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":c(@#$)\", \":d(@#$)\", \"e\")}\n"
        ),
        "a {\
         \n  b: :c(@#$);\
         \n}\n"
    );
        }
    }
}
mod element {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn equal() {
        assert_eq!(
            runner().ok(
                "a {b: selector-extend(\"::c(@#$)\", \"::c(@#$)\", \"e\")}\n"
            ),
            "a {\
         \n  b: ::c(@#$), e;\
         \n}\n"
        );
    }
    mod unequal {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn argument() {
            assert_eq!(
        runner().ok(
            "a {b: selector-extend(\"::c(@#$)\", \"::c(*&^)\", \"e\")}\n"
        ),
        "a {\
         \n  b: ::c(@#$);\
         \n}\n"
    );
        }
        #[test]
        fn has_argument() {
            assert_eq!(
                runner().ok(
                    "a {b: selector-extend(\"::c(@#$)\", \"::c\", \"e\")}\n"
                ),
                "a {\
         \n  b: ::c(@#$);\
         \n}\n"
            );
        }
        #[test]
        fn name() {
            assert_eq!(
        runner().ok(
            "a {b: selector-extend(\"::c(@#$)\", \":d(@#$)\", \"e\")}\n"
        ),
        "a {\
         \n  b: ::c(@#$);\
         \n}\n"
    );
        }
    }
}
