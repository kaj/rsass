//! Tests auto-converted from "sass-spec/spec/css/selector/parent.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("parent")
}

mod alone {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn first() {
        assert_eq!(
            runner().ok("& {a: b}\n"),
            "& {\
         \n  a: b;\
         \n}\n"
        );
    }
    #[test]
    fn second() {
        assert_eq!(
            runner().ok("a {\
             \n  & {b: c}\
             \n}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
}
mod complex {
    use super::runner;

    #[test]
    fn complex_parent() {
        assert_eq!(
            runner().ok("a b {\
             \n  c &.d {e: f}\
             \n}\n"),
            "c a b.d {\
         \n  e: f;\
         \n}\n"
        );
    }
    #[test]
    fn simple_parent() {
        assert_eq!(
            runner().ok("a {\
             \n  b &.c {d: e}\
             \n}\n"),
            "b a.c {\
         \n  d: e;\
         \n}\n"
        );
    }
}
#[test]
fn compound() {
    assert_eq!(
        runner().ok("a {\
             \n  &.b {c: d}\
             \n}\n"),
        "a.b {\
         \n  c: d;\
         \n}\n"
    );
}
mod error {
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn first_arg_suffix() {
        assert_eq!(
            runner().err("&a {b: c}\n"),
            "Error: A top-level selector may not contain a parent selector with a suffix.\
         \n  ,\
         \n1 | &a {b: c}\
         \n  | ^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn in_at_rule_suffix() {
        assert_eq!(
            runner().err(
                "@a {\
             \n  &b {c: d}\
             \n}\n"
            ),
            "Error: A top-level selector may not contain a parent selector with a suffix.\
         \n  ,\
         \n2 |   &b {c: d}\
         \n  |   ^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn non_initial() {
        assert_eq!(
            runner().err(
                "a {\
             \n  [b]& {c: d}\
             \n}\n"
            ),
            "Error: \"&\" may only used at the beginning of a compound selector.\
         \n  ,\
         \n2 |   [b]& {c: d}\
         \n  |      ^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn prefix() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b& {c: d}\
             \n}\n"
            ),
            "Error: \"&\" may only used at the beginning of a compound selector.\
         \n  ,\
         \n2 |   b& {c: d}\
         \n  |    ^\
         \n  \'\
         \n  input.scss 2:4  root stylesheet",
        );
    }
}
#[test]
#[ignore] // wrong result
fn in_at_rule() {
    assert_eq!(
        runner().ok("@a {\
             \n  & {b: c}\
             \n}\n"),
        "@a {\
         \n  & {\
         \n    b: c;\
         \n  }\
         \n}\n"
    );
}
#[test]
fn in_one_complex() {
    assert_eq!(
        runner().ok("a {\
             \n  &.b, c {d: e}\
             \n}\n"),
        "a.b, a c {\
         \n  d: e;\
         \n}\n"
    );
}
#[test]
fn multiple() {
    assert_eq!(
        runner().ok("a {\
             \n  &.b &.c {d: e}\
             \n}\n"),
        "a.b a.c {\
         \n  d: e;\
         \n}\n"
    );
}
mod selector_pseudo {
    use super::runner;

    mod complex_parent {
        use super::runner;

        #[test]
        fn is() {
            assert_eq!(
                runner().ok("a b {\
             \n  :is(&) {c: d}\
             \n}\n"),
                ":is(a b) {\
         \n  c: d;\
         \n}\n"
            );
        }
        #[test]
        fn matches() {
            assert_eq!(
                runner().ok("a b {\
             \n  :where(&) {c: d}\
             \n}\n"),
                ":where(a b) {\
         \n  c: d;\
         \n}\n"
            );
        }
        #[test]
        fn test_where() {
            assert_eq!(
                runner().ok("a b {\
             \n  :where(&) {c: d}\
             \n}\n"),
                ":where(a b) {\
         \n  c: d;\
         \n}\n"
            );
        }
    }
    mod simple_parent {
        use super::runner;

        #[test]
        fn is() {
            assert_eq!(
                runner().ok("a {\
             \n  :is(&) {b: c}\
             \n}\n"),
                ":is(a) {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        fn matches() {
            assert_eq!(
                runner().ok("a {\
             \n  :matches(&) {b: c}\
             \n}\n"),
                ":matches(a) {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        fn test_where() {
            assert_eq!(
                runner().ok("a {\
             \n  :where(&) {b: c}\
             \n}\n"),
                ":where(a) {\
         \n  b: c;\
         \n}\n"
            );
        }
    }
}
#[test]
fn suffix() {
    assert_eq!(
        runner().ok("a {\
             \n  &b {c: d}\
             \n}\n"),
        "ab {\
         \n  c: d;\
         \n}\n"
    );
}
