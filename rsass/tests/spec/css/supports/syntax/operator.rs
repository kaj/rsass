//! Tests auto-converted from "sass-spec/spec/css/supports/syntax/operator.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("operator")
}

#[test]
fn and() {
    assert_eq!(
        runner().ok("@supports (a: b) and (c: d) and (e: f) {@g}\n"),
        "@supports (a: b) and (c: d) and (e: f) {\
         \n  @g;\
         \n}\n"
    );
}
mod mixed {
    use super::runner;

    #[test]
    fn and_in_not() {
        assert_eq!(
            runner().ok("@supports not ((a: b) and (c: d)) {@e}\n"),
            "@supports not ((a: b) and (c: d)) {\
         \n  @e;\
         \n}\n"
        );
    }
    #[test]
    fn and_in_or() {
        assert_eq!(
            runner().ok("@supports ((a: b) and (c: d)) or (e: f) {@g}\n"),
            "@supports ((a: b) and (c: d)) or (e: f) {\
         \n  @g;\
         \n}\n"
        );
    }
    #[test]
    fn or_in_and() {
        assert_eq!(
            runner().ok("@supports (a: b) and ((c: d) or (e: f)) {@g}\n"),
            "@supports (a: b) and ((c: d) or (e: f)) {\
         \n  @g;\
         \n}\n"
        );
    }
}
#[test]
fn not() {
    assert_eq!(
        runner().ok("@supports not (a: b) {@c}\n"),
        "@supports not (a: b) {\
         \n  @c;\
         \n}\n"
    );
}
#[test]
fn or() {
    assert_eq!(
        runner().ok("@supports (a: b) or (c: d) or (e: f) {@g}\n"),
        "@supports (a: b) or (c: d) or (e: f) {\
         \n  @g;\
         \n}\n"
    );
}
