//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hsla")
}

#[test]
fn four_args() {
    assert_eq!(
        runner().ok("a {b: hsla(180, 60%, 50%, 0.4)}\n"),
        "a {\
         \n  b: hsla(180, 60%, 50%, 0.4);\
         \n}\n"
    );
}
mod one_arg {
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
            runner().ok("a {b: hsla(180 60% 50% / 0.4)}\n"),
            "a {\
         \n  b: hsla(180, 60%, 50%, 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn no_alpha() {
        assert_eq!(
            runner().ok("a {b: hsla(180 60% 50%)}\n"),
            "a {\
         \n  b: hsl(180, 60%, 50%);\
         \n}\n"
        );
    }
}
#[test]
fn three_args() {
    assert_eq!(
        runner().ok("a {b: hsla(180, 60%, 50%)}\n"),
        "a {\
         \n  b: hsl(180, 60%, 50%);\
         \n}\n"
    );
}
