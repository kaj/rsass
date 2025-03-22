//! Tests auto-converted from "sass-spec/spec/css/supports/syntax/anything.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("anything")
}

#[test]
fn ident_only() {
    assert_eq!(
        runner().ok("@supports (a) {@b}\n"),
        "@supports (a) {\
         \n  @b;\
         \n}\n"
    );
}
#[test]
fn idents() {
    assert_eq!(
        runner().ok("@supports (a b) {@c}\n"),
        "@supports (a b) {\
         \n  @c;\
         \n}\n"
    );
}
mod interpolated_any_value {
    use super::runner;

    #[test]
    fn full() {
        assert_eq!(
            runner().ok("@supports (a #{1 + 1}) {@b}\n"),
            "@supports (a 2) {\
         \n  @b;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@supports (a <#{1 + 1}>) {@b}\n"),
            "@supports (a <2>) {\
         \n  @b;\
         \n}\n"
        );
    }
}
mod interpolated_ident {
    use super::runner;

    #[test]
    fn full() {
        assert_eq!(
            runner().ok("@supports (#{\"a\"} b) {@c}\n"),
            "@supports (a b) {\
         \n  @c;\
         \n}\n"
        );
    }
    #[test]
    fn full_before_andlike() {
        assert_eq!(
            runner().ok("@supports (#{\"a\"} andb) {@c}\n"),
            "@supports (a andb) {\
         \n  @c;\
         \n}\n"
        );
    }
    #[test]
    fn partial() {
        assert_eq!(
            runner().ok("@supports (a#{\"b\"}c d) {@e}\n"),
            "@supports (abc d) {\
         \n  @e;\
         \n}\n"
        );
    }
}
#[test]
fn no_space() {
    assert_eq!(
        runner().ok("@supports (a!) {@b}\n"),
        "@supports (a!) {\
         \n  @b;\
         \n}\n"
    );
}
#[test]
fn only_space() {
    assert_eq!(
        runner().ok("@supports (a ) {@b}\n"),
        "@supports (a ) {\
         \n  @b;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn symbols() {
    assert_eq!(
        runner().ok("@supports (a !&$ZH()&;*{&A}_=-+#/><) {@b}\n"),
        "@supports (a !&$ZH()&;*{&A}_=-+#/><) {\
         \n  @b;\
         \n}\n"
    );
}
