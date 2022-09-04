//! Tests auto-converted from "sass-spec/spec/css/supports/syntax/function.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("function")
}

#[test]
fn after_not() {
    assert_eq!(
        runner().ok("@supports not a() {@b}\n"),
        "@supports not a() {\
         \n  @b;\
         \n}\n"
    );
}
mod interpolated_name {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn full() {
        assert_eq!(
            runner().ok("@supports #{\"a\"}(b) {@c}\n"),
            "@supports a(b) {\
         \n  @c;\
         \n}\n"
        );
    }
    #[test]
    fn partial() {
        assert_eq!(
            runner().ok("@supports a#{\"b\"}c(d) {@e}\n"),
            "@supports abc(d) {\
         \n  @e;\
         \n}\n"
        );
    }
}
mod interpolated_value {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn full() {
        assert_eq!(
            runner().ok("@supports a(#{1 + 1}) {@c}\n"),
            "@supports a(2) {\
         \n  @c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@supports a(<#{1 + 1}>) {@c}\n"),
            "@supports a(<2>) {\
         \n  @c;\
         \n}\n"
        );
    }
}
#[test]
fn no_arg() {
    assert_eq!(
        runner().ok("@supports a() {@b}\n"),
        "@supports a() {\
         \n  @b;\
         \n}\n"
    );
}
#[test]
fn plain() {
    assert_eq!(
        runner().ok("@supports a(b) {@c}\n"),
        "@supports a(b) {\
         \n  @c;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn space() {
    assert_eq!(
        runner().ok("@supports a( ) {@b}\n"),
        "@supports a( ) {\
         \n  @b;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn symbols() {
    assert_eq!(
        runner().ok("@supports a(!&$ZH()&;*{&A}_=-+#/>:<) {@b}\n"),
        "@supports a(!&$ZH()&;*{&A}_=-+#/>:<) {\
         \n  @b;\
         \n}\n"
    );
}
