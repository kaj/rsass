//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgba.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("rgba")
}

#[test]
fn four_args() {
    assert_eq!(
        runner().ok("a {b: rgba(0, 255, 127, 0.4)}\n"),
        "a {\
         \n  b: rgba(0, 255, 127, 0.4);\
         \n}\n"
    );
}
mod one_arg {
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
            runner().ok("a {b: rgba(0 255 127 / 0.4)}\n"),
            "a {\
         \n  b: rgba(0, 255, 127, 0.4);\
         \n}\n"
        );
    }
    #[test]
    fn no_alpha() {
        assert_eq!(
            runner().ok("a {b: rgba(190 173 237)}\n"),
            "a {\
         \n  b: rgb(190, 173, 237);\
         \n}\n"
        );
    }
}
#[test]
fn three_args() {
    assert_eq!(
        runner().ok("a {b: rgba(190, 173, 237)}\n"),
        "a {\
         \n  b: rgb(190, 173, 237);\
         \n}\n"
    );
}
