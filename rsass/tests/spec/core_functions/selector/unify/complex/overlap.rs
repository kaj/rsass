//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/complex/overlap.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("overlap")
}

#[test]
fn class() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \n// Because neither compound selector containing `.c` is a superselector of the\
             \n// other, they aren\'t unified.\
             \na {b: selector.unify(\".c.s1-1 .s1-2\", \".c.s2-1 .s2-2\")}\n"
        ),
        "a {\
         \n  b: .c.s1-1 .c.s2-1 .s1-2.s2-2, .c.s2-1 .c.s1-1 .s1-2.s2-2;\
         \n}\n"
    );
}
mod id {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn forced_unification() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \n// Even though neither selector containing `#c` is a superselector of the other,\
             \n// they\'re still unified because the selector can\'t meainingfully contain two\
             \n// instances of `#c`.\
             \na {b: selector.unify(\"#c.s1-1 .s1-2\", \"#c.s2-1 .s2-2\")}\n"
        ),
        "a {\
         \n  b: #c.s2-1.s1-1 .s1-2.s2-2;\
         \n}\n"
    );
    }
    #[test]
    fn no_unification() {
        assert_eq!(
            runner().ok(
                "@use \"sass:selector\";\
             \na {b: selector.unify(\"#s1-1.c .s1-2\", \"#s2-1.c .s2-2\")}\n"
            ),
            "a {\
         \n  b: #s1-1.c #s2-1.c .s1-2.s2-2, #s2-1.c #s1-1.c .s1-2.s2-2;\
         \n}\n"
        );
    }
}
mod pseudo_element {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn forced_unification() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.unify(\".s1-1::c .s1-2\", \".s2-1::c .s2-2\")}\n"
        ),
        "a {\
         \n  b: .s2-1.s1-1::c .s1-2.s2-2;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn no_unification() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.unify(\"::s1-1.c .s1-2\", \"::s2-1.c .s2-2\")}\n"
        ),
        "a {\
         \n  b: ::s1-1.c ::s2-1.c .s1-2.s2-2, ::s2-1.c ::s1-1.c .s1-2.s2-2;\
         \n}\n"
    );
    }
}
