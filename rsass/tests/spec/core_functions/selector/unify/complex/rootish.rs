//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/complex/rootish.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rootish")
}

#[test]
fn host() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":host .c\", \".d .e\")}\n"),
        "a {\
         \n  b: :host .d .c.e;\
         \n}\n"
    );
}
#[test]
fn host_context() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.unify(\":host-context(f g) .c\", \".d .e\")}\n"
        ),
        "a {\
         \n  b: :host-context(f g) .d .c.e;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn mixed() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":root .c .d\", \":scope .e .f\")}\n"),
        "a {\
         \n  b: :root:scope .c .e .d.f, :root:scope .e .c .d.f;\
         \n}\n"
    );
}
mod root {
    #[allow(unused)]
    use super::runner;

    mod in_both {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn can_unify() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c:root .d\", \".e:root .f\")}\n"),
                "a {\
         \n  b: .c.e:root .d.f;\
         \n}\n"
            );
        }
        #[test]
        fn cant_unify() {
            assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"c:root .d\", \"e:root .f\"))}\n"
        ),
        "a {\
         \n  b: null;\
         \n}\n"
    );
        }
        #[test]
        fn superselector() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"c:root .d\", \":root .e\")}\n"),
                "a {\
         \n  b: c:root .d.e;\
         \n}\n"
            );
        }
    }
    mod in_one {
        #[allow(unused)]
        use super::runner;

        mod selector1 {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn three_layer() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":root .c .d\", \".e .f\")}\n"),
                    "a {\
         \n  b: :root .c .e .d.f, :root .e .c .d.f;\
         \n}\n"
                );
            }
            #[test]
            fn two_layer() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":root .c\", \".d .e\")}\n"),
                    "a {\
         \n  b: :root .d .c.e;\
         \n}\n"
                );
            }
        }
        mod selector2 {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn three_layer() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c .d\", \":root .e .f\")}\n"),
                    "a {\
         \n  b: :root .c .e .d.f, :root .e .c .d.f;\
         \n}\n"
                );
            }
            #[test]
            fn two_layer() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c .d\", \":root .e\")}\n"),
                    "a {\
         \n  b: :root .c .d.e;\
         \n}\n"
                );
            }
        }
    }
}
#[test]
fn scope() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\":scope .c\", \".d .e\")}\n"),
        "a {\
         \n  b: :scope .d .c.e;\
         \n}\n"
    );
}
