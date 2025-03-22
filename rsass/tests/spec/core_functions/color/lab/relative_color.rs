//! Tests auto-converted from "sass-spec/spec/core_functions/color/lab/relative_color.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("relative_color")
}

mod calc {
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
            runner().ok("a {b: lab(from #aaa calc(l + 0.2) a b / 25%)}\n"),
            "a {\
         \n  b: lab(from #aaa calc(l + 0.2) a b/25%);\
         \n}\n"
        );
    }
    #[test]
    fn no_alpha() {
        assert_eq!(
            runner().ok("a {b: lab(from #aaa calc(l + 0.2) a b)}\n"),
            "a {\
         \n  b: lab(from #aaa calc(l + 0.2) a b);\
         \n}\n"
        );
    }
}
mod different_case {
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
            runner().ok("a {b: lab(From #aaa l a b / 25%)}\n"),
            "a {\
         \n  b: lab(From #aaa l a b/25%);\
         \n}\n"
        );
    }
    #[test]
    fn no_alpha() {
        assert_eq!(
            runner().ok("a {b: lab(From #aaa l a b)}\n"),
            "a {\
         \n  b: lab(From #aaa l a b);\
         \n}\n"
        );
    }
}
mod error {
    use super::runner;

    mod quoted {
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn alpha() {
            assert_eq!(
        runner().err(
            "a {b: lab(\"from\" #aaa l a b / 25%)}\n"
        ),
        "Error: $channels: Expected lightness channel to be a number, was \"from\".\
         \n  ,\
         \n1 | a {b: lab(\"from\" #aaa l a b / 25%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn no_alpha() {
            assert_eq!(
        runner().err(
            "a {b: lab(\"from\" #aaa l a b)}\n"
        ),
        "Error: $channels: Expected lightness channel to be a number, was \"from\".\
         \n  ,\
         \n1 | a {b: lab(\"from\" #aaa l a b)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
    }
    mod wrong_keyword {
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn alpha() {
            assert_eq!(
        runner().err(
            "a {b: lab(c #aaa l a b / 25%)}\n"
        ),
        "Error: $channels: Expected lightness channel to be a number, was c.\
         \n  ,\
         \n1 | a {b: lab(c #aaa l a b / 25%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn no_alpha() {
            assert_eq!(
        runner().err(
            "a {b: lab(c #aaa l a b)}\n"
        ),
        "Error: $channels: Expected lightness channel to be a number, was c.\
         \n  ,\
         \n1 | a {b: lab(c #aaa l a b)}\
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
             \na {b: lab(list.slash(from #aaa r g b, 25%))}\n"),
        "a {\
         \n  b: lab(from #aaa r g b / 25%);\
         \n}\n"
    );
}
mod test_static {
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
            runner().ok("a {b: lab(from #aaa l a b / 25%)}\n"),
            "a {\
         \n  b: lab(from #aaa l a b/25%);\
         \n}\n"
        );
    }
    #[test]
    fn no_alpha() {
        assert_eq!(
            runner().ok("a {b: lab(from #aaa l a b)}\n"),
            "a {\
         \n  b: lab(from #aaa l a b);\
         \n}\n"
        );
    }
}
mod var {
    use super::runner;

    #[test]
    fn alpha() {
        assert_eq!(
            runner().ok("a {b: lab(from var(--c) l a b / 25%)}\n"),
            "a {\
         \n  b: lab(from var(--c) l a b/25%);\
         \n}\n"
        );
    }
    #[test]
    fn no_alpha() {
        assert_eq!(
            runner().ok("a {b: lab(from var(--c) l a b)}\n"),
            "a {\
         \n  b: lab(from var(--c) l a b);\
         \n}\n"
        );
    }
}
