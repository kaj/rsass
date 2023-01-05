//! Tests auto-converted from "sass-spec/spec/css/supports/syntax/declaration.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("declaration")
}

mod custom_prop {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn comma() {
        assert_eq!(
            runner().ok("@supports (--a: ,) {@c}\n"),
            "@supports (--a: ,) {\
         \n  @c;\
         \n}\n"
        );
    }
    #[test]
    fn value() {
        assert_eq!(
            runner().ok("@supports (--a: b) {@c}\n"),
            "@supports (--a: b) {\
         \n  @c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn whitespace() {
        assert_eq!(
            runner().ok("@supports (--a: ) {@c}\n"),
            "@supports (--a: ) {\
         \n  @c;\
         \n}\n"
        );
    }
}
mod dynamic {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn lhs() {
        assert_eq!(
            runner().ok("@supports (1 + 1: b) {@c}\n"),
            "@supports (2: b) {\
         \n  @c;\
         \n}\n"
        );
    }
    #[test]
    fn rhs() {
        assert_eq!(
            runner().ok("@supports (a: 1 + 1) {@c}\n"),
            "@supports (a: 2) {\
         \n  @c;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn nested() {
    assert_eq!(
        runner().ok("@supports ((((a: b)))) {@c}\n"),
        "@supports (a: b) {\
         \n  @c;\
         \n}\n"
    );
}
mod plain {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn ident() {
        assert_eq!(
            runner().ok("@supports (a: b) {@c}\n"),
            "@supports (a: b) {\
         \n  @c;\
         \n}\n"
        );
    }
    #[test]
    fn quoted_rhs() {
        assert_eq!(
            runner().ok("@supports (a: \"b\") {@c}\n"),
            "@supports (a: \"b\") {\
         \n  @c;\
         \n}\n"
        );
    }
}
