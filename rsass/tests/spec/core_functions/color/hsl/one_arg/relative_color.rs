//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/one_arg/relative_color.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("relative_color")
}

mod calc {
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
            runner().ok("a {b: hsl(from #aaa calc(h + 180deg) s l / 25%)}\n"),
            "a {\
         \n  b: hsl(from #aaa calc(h + 180deg) s l/25%);\
         \n}\n"
        );
    }
    #[test]
    fn no_alpha() {
        assert_eq!(
            runner().ok("a {b: hsl(from #aaa calc(h + 180deg) s l)}\n"),
            "a {\
         \n  b: hsl(from #aaa calc(h + 180deg) s l);\
         \n}\n"
        );
    }
}
mod different_case {
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
            runner().ok("a {b: hsl(From #aaa h s l / 25%)}\n"),
            "a {\
         \n  b: hsl(From #aaa h s l/25%);\
         \n}\n"
        );
    }
    #[test]
    fn no_alpha() {
        assert_eq!(
            runner().ok("a {b: hsl(From #aaa h s l)}\n"),
            "a {\
         \n  b: hsl(From #aaa h s l);\
         \n}\n"
        );
    }
}
mod error {
    use super::runner;

    mod quoted {
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn alpha() {
            assert_eq!(
        runner().err(
            "a {b: hsl(\"from\" #aaa h s l / 25%)}\n"
        ),
        "Error: $channels: Expected hue channel to be a number, was \"from\".\
         \n  ,\
         \n1 | a {b: hsl(\"from\" #aaa h s l / 25%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn no_alpha() {
            assert_eq!(
        runner().err(
            "a {b: hsl(\"from\" #aaa h s l)}\n"
        ),
        "Error: $channels: Expected hue channel to be a number, was \"from\".\
         \n  ,\
         \n1 | a {b: hsl(\"from\" #aaa h s l)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
    }
    mod wrong_keyword {
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn alpha() {
            assert_eq!(
        runner().err(
            "a {b: hsl(c #aaa h s l / 25%)}\n"
        ),
        "Error: $channels: Expected hue channel to be a number, was c.\
         \n  ,\
         \n1 | a {b: hsl(c #aaa h s l / 25%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn no_alpha() {
            assert_eq!(
        runner().err(
            "a {b: hsl(c #aaa h s l)}\n"
        ),
        "Error: $channels: Expected hue channel to be a number, was c.\
         \n  ,\
         \n1 | a {b: hsl(c #aaa h s l)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
    }
}
#[test]
fn slash_list_alpha() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \na {b: hsl(list.slash(from #aaa h s l, 25%))}\n"),
        "a {\
         \n  b: hsl(from #aaa h s l / 25%);\
         \n}\n"
    );
}
mod test_static {
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
            runner().ok("a {b: hsl(from #aaa h s l / 25%)}\n"),
            "a {\
         \n  b: hsl(from #aaa h s l/25%);\
         \n}\n"
        );
    }
    #[test]
    fn no_alpha() {
        assert_eq!(
            runner().ok("a {b: hsl(from #aaa h s l)}\n"),
            "a {\
         \n  b: hsl(from #aaa h s l);\
         \n}\n"
        );
    }
}
mod var {
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
            runner().ok("a {b: hsl(from var(--c) h s l / 25%)}\n"),
            "a {\
         \n  b: hsl(from var(--c) h s l/25%);\
         \n}\n"
        );
    }
    #[test]
    fn no_alpha() {
        assert_eq!(
            runner().ok("a {b: hsl(from var(--c) h s l)}"),
            "a {\
         \n  b: hsl(from var(--c) h s l);\
         \n}\n"
        );
    }
}
