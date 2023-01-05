//! Tests auto-converted from "sass-spec/spec/css/supports/syntax/lone_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lone_interpolation")
}

mod parens {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn after_operator() {
        assert_eq!(
            runner().ok("@supports ((c: 1 + 1) and #{\"(a: b)\"})  {@d}\n"),
            "@supports (c: 2) and (a: b) {\
         \n  @d;\
         \n}\n"
        );
    }
    #[test]
    fn alone() {
        assert_eq!(
            runner().ok("@supports (#{\"(a: b)\"}) {@c}\n"),
            "@supports ((a: b)) {\
         \n  @c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn before_operator() {
        assert_eq!(
            runner().ok("@supports (#{\"(a: b)\"} and (c: 1 + 1)) {@d}\n"),
            "@supports (a: b) and (c: 2) {\
         \n  @d;\
         \n}\n"
        );
    }
}
mod top_level {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn after_operator() {
        assert_eq!(
            runner().ok("@supports (c: 1 + 1) and #{\"(a: b)\"}  {@d}\n"),
            "@supports (c: 2) and (a: b) {\
         \n  @d;\
         \n}\n"
        );
    }
    #[test]
    fn alone() {
        assert_eq!(
            runner().ok("@supports #{\"(a: b)\"} {@c}\n"),
            "@supports (a: b) {\
         \n  @c;\
         \n}\n"
        );
    }
    #[test]
    fn before_operator() {
        assert_eq!(
            runner().ok("@supports #{\"(a: b)\"} and (c: 1 + 1) {@d}\n"),
            "@supports (a: b) and (c: 2) {\
         \n  @d;\
         \n}\n"
        );
    }
}
