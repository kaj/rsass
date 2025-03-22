//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/simple/pseudo/selector/idempotent/not.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("not")
}

#[test]
#[ignore] // wrong result
fn complex() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\":not(.c .d)\", \".d\", \".e .f\")}\n"),
        "a {\
         \n  b: :not(.c .d):not(.c .e .f):not(.e .c .f);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn component() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\":not(.c.d)\", \".c\", \".e\")}\n"),
        "a {\
         \n  b: :not(.c.d):not(.d.e);\
         \n}\n"
    );
}
mod is {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn in_compound() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.extend(\":not(.c)\", \".c\", \".d:is(.e, .f)\")}\n"
        ),
        "a {\
         \n  b: :not(.c):not(.d:is(.e, .f));\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn list() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.extend(\":not(.c)\", \".c\", \":is(.d, .e)\")}\n"
        ),
        "a {\
         \n  b: :not(.c):not(.d):not(.e);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn list_of_complex() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.extend(\":not(.c)\", \".c\", \":is(.d .e, .f .g)\")}\n"
        ),
        "a {\
         \n  b: :not(.c):not(.d .e):not(.f .g);\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // wrong result
fn list() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\":not(.c)\", \".c\", \".d, .e\")}\n"),
        "a {\
         \n  b: :not(.c):not(.d):not(.e);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn list_in_not() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \n// If the original :not() already contains a selector list, we add new selectors\
             \n// to that list because there\'s no risk of breaking additional browsers.\
             \na {b: selector.extend(\":not(.c, .d)\", \".c\", \".e\")}\n"
        ),
        "a {\
         \n  b: :not(.c, .e, .d);\
         \n}\n"
    );
}
mod matches {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn in_compound() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.extend(\":not(.c)\", \".c\", \".d:matches(.e, .f)\")}\n"
        ),
        "a {\
         \n  b: :not(.c):not(.d:matches(.e, .f));\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn list() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.extend(\":not(.c)\", \".c\", \":matches(.d, .e)\")}\n"
        ),
        "a {\
         \n  b: :not(.c):not(.d):not(.e);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn list_of_complex() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.extend(\":not(.c)\", \".c\", \":matches(.d .e, .f .g)\")}\n"
        ),
        "a {\
         \n  b: :not(.c):not(.d .e):not(.f .g);\
         \n}\n"
    );
    }
}
#[test]
fn not_in_extender() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \n// Ideally, this should emit `.d, :not(.c)`, but that would be the only\
             \n// situation where extending a pseudo selector could produce a full-on selector\
             \n// list. For the sake of simplicity of the `@extend` algorithm, we just ignore\
             \n// nested `:not()`s instead.\
             \na {b: selector.extend(\":not(.c)\", \".c\", \":not(.d)\")}\n"
        ),
        "a {\
         \n  b: :not(.c);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn simple() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\":not(.c)\", \".c\", \".d\")}\n"),
        "a {\
         \n  b: :not(.c):not(.d);\
         \n}\n"
    );
}
mod test_where {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn in_compound() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.extend(\":not(.c)\", \".c\", \".d:where(.e, .f)\")}\n"
        ),
        "a {\
         \n  b: :not(.c):not(.d:where(.e, .f));\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn list() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.extend(\":not(.c)\", \".c\", \":where(.d, .e)\")}\n"
        ),
        "a {\
         \n  b: :not(.c):not(.d):not(.e);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn list_of_complex() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.extend(\":not(.c)\", \".c\", \":where(.d .e, .f .g)\")}\n"
        ),
        "a {\
         \n  b: :not(.c):not(.d .e):not(.f .g);\
         \n}\n"
    );
    }
}
