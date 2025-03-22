//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/one_arg/relative_color.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("relative_color")
}

mod calc {
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
            runner().ok("a {b: rgb(from #aaa calc(r + 2) g b / 25%)}\n"),
            "a {\
         \n  b: rgb(from #aaa calc(r + 2) g b/25%);\
         \n}\n"
        );
    }
    #[test]
    fn no_alpha() {
        assert_eq!(
            runner().ok("a {b: rgb(from #aaa calc(r + 2) g b)}\n"),
            "a {\
         \n  b: rgb(from #aaa calc(r + 2) g b);\
         \n}\n"
        );
    }
}
mod different_case {
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
            runner().ok("a {b: rgb(From #aaa r g b / 25%)}\n"),
            "a {\
         \n  b: rgb(From #aaa r g b/25%);\
         \n}\n"
        );
    }
    #[test]
    fn no_alpha() {
        assert_eq!(
            runner().ok("a {b: rgb(From #aaa r g b)}\n"),
            "a {\
         \n  b: rgb(From #aaa r g b);\
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
            "a {b: rgb(\"from\" #aaa r g b / 25%)}\n"
        ),
        "Error: $channels: Expected red channel to be a number, was \"from\".\
         \n  ,\
         \n1 | a {b: rgb(\"from\" #aaa r g b / 25%)}\
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
            "a {b: rgb(\"from\" #aaa r g b)}\n"
        ),
        "Error: $channels: Expected red channel to be a number, was \"from\".\
         \n  ,\
         \n1 | a {b: rgb(\"from\" #aaa r g b)}\
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
            "a {b: rgb(c #aaa r g b / 25%)}\n"
        ),
        "Error: $channels: Expected red channel to be a number, was c.\
         \n  ,\
         \n1 | a {b: rgb(c #aaa r g b / 25%)}\
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
            "a {b: rgb(c #aaa r g b)}\n"
        ),
        "Error: $channels: Expected red channel to be a number, was c.\
         \n  ,\
         \n1 | a {b: rgb(c #aaa r g b)}\
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
             \na {b: rgb(list.slash(from #aaa r g b, 25%))}\n"),
        "a {\
         \n  b: rgb(from #aaa r g b / 25%);\
         \n}\n"
    );
}
mod test_static {
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
            runner().ok("a {b: rgb(from #aaa r g b / 25%)}\n"),
            "a {\
         \n  b: rgb(from #aaa r g b/25%);\
         \n}\n"
        );
    }
    #[test]
    fn no_alpha() {
        assert_eq!(
            runner().ok("a {b: rgb(from #aaa r g b)}\n"),
            "a {\
         \n  b: rgb(from #aaa r g b);\
         \n}\n"
        );
    }
}
mod var {
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
            runner().ok("a {b: rgb(from var(--c) r g b / 25%)}\n"),
            "a {\
         \n  b: rgb(from var(--c) r g b/25%);\
         \n}\n"
        );
    }
    #[test]
    fn no_alpha() {
        assert_eq!(
            runner().ok("a {b: rgb(from var(--c) r g b)}\n"),
            "a {\
         \n  b: rgb(from var(--c) r g b);\
         \n}\n"
        );
    }
}
