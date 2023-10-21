//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/one_arg/relative_color.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("relative_color")
}

mod calc {
    #[allow(unused)]
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
    #[allow(unused)]
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
    #[allow(unused)]
    use super::runner;

    mod quoted {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn alpha() {
            assert_eq!(
                runner().err("a {b: rgb(\"from\" #aaa r g b / 25%)}\n"),
                "Error: Only 3 elements allowed, but 5 were passed.\
         \n  ,\
         \n1 | a {b: rgb(\"from\" #aaa r g b / 25%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn no_alpha() {
            assert_eq!(
                runner().err("a {b: rgb(\"from\" #aaa r g b)}\n"),
                "Error: Only 3 elements allowed, but 5 were passed.\
         \n  ,\
         \n1 | a {b: rgb(\"from\" #aaa r g b)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
    mod wrong_keyword {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn alpha() {
            assert_eq!(
                runner().err("a {b: rgb(c #aaa r g b / 25%)}\n"),
                "Error: Only 3 elements allowed, but 5 were passed.\
         \n  ,\
         \n1 | a {b: rgb(c #aaa r g b / 25%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn no_alpha() {
            assert_eq!(
                runner().err("a {b: rgb(c #aaa r g b)}\n"),
                "Error: Only 3 elements allowed, but 5 were passed.\
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
             \na {b: rgb(list.slash(from #aaa h s l, 25%))}\n"),
        "a {\
         \n  b: rgb(from #aaa h s l / 25%);\
         \n}\n"
    );
}
mod test_static {
    #[allow(unused)]
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
    #[allow(unused)]
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
