//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/simple/pseudo/selector/non_idempotent.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("non_idempotent")
}

mod has {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn has_in_extender() {
        assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":has(.c)\", \".c\", \":has(.d)\")}\n"
        ),
        "a {\
         \n  b: :has(.c, :has(.d));\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn list() {
        assert_eq!(
            runner().ok(
                "a {b: selector-extend(\":has(.c)\", \".c\", \".d, .e\")}\n"
            ),
            "a {\
         \n  b: :has(.c, .d, .e);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn simple() {
        assert_eq!(
            runner()
                .ok("a {b: selector-extend(\":has(.c)\", \".c\", \".d\")}\n"),
            "a {\
         \n  b: :has(.c, .d);\
         \n}\n"
        );
    }
}
mod host {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn host_in_extender() {
        assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":host(.c)\", \".c\", \":host(.d)\")}\n"
        ),
        "a {\
         \n  b: :host(.c, :host(.d));\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn list() {
        assert_eq!(
            runner().ok(
                "a {b: selector-extend(\":host(.c)\", \".c\", \".d, .e\")}\n"
            ),
            "a {\
         \n  b: :host(.c, .d, .e);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn simple() {
        assert_eq!(
            runner().ok(
                "a {b: selector-extend(\":host(.c)\", \".c\", \".d\")}\n"
            ),
            "a {\
         \n  b: :host(.c, .d);\
         \n}\n"
        );
    }
}
mod host_context {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn host_context_in_extender() {
        assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":host-context(.c)\", \".c\", \":host-context(.d)\")}\n"
        ),
        "a {\
         \n  b: :host-context(.c, :host-context(.d));\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn list() {
        assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":host-context(.c)\", \".c\", \".d, .e\")}\n"
        ),
        "a {\
         \n  b: :host-context(.c, .d, .e);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn simple() {
        assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":host-context(.c)\", \".c\", \".d\")}\n"
        ),
        "a {\
         \n  b: :host-context(.c, .d);\
         \n}\n"
    );
    }
}
mod slotted {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn list() {
        assert_eq!(
        runner().ok(
            "a {b: selector-extend(\"::slotted(.c)\", \".c\", \".d, .e\")}\n"
        ),
        "a {\
         \n  b: ::slotted(.c, .d, .e);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn simple() {
        assert_eq!(
            runner().ok(
                "a {b: selector-extend(\"::slotted(.c)\", \".c\", \".d\")}\n"
            ),
            "a {\
         \n  b: ::slotted(.c, .d);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn slotted_in_extender() {
        assert_eq!(
        runner().ok(
            "a {b: selector-extend(\"::slotted(.c)\", \".c\", \"::slotted(.d)\")}\n"
        ),
        "a {\
         \n  b: ::slotted(.c, ::slotted(.d));\
         \n}\n"
    );
    }
}
