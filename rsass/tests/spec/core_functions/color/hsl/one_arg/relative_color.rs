//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/one_arg/relative_color.hrx"

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
    #[allow(unused)]
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
    #[allow(unused)]
    use super::runner;

    mod quoted {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn alpha() {
            assert_eq!(
                runner().err("a {b: hsl(\"from\" #aaa h s l / 25%)}\n"),
                "Error: Only 3 elements allowed, but 5 were passed.\
         \n  ,\
         \n1 | a {b: hsl(\"from\" #aaa h s l / 25%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn no_alpha() {
            assert_eq!(
                runner().err("a {b: hsl(\"from\" #aaa h s l)}\n"),
                "Error: Only 3 elements allowed, but 5 were passed.\
         \n  ,\
         \n1 | a {b: hsl(\"from\" #aaa h s l)}\
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
                runner().err("a {b: hsl(c #aaa h s l / 25%)}\n"),
                "Error: Only 3 elements allowed, but 5 were passed.\
         \n  ,\
         \n1 | a {b: hsl(c #aaa h s l / 25%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn no_alpha() {
            assert_eq!(
                runner().err("a {b: hsl(c #aaa h s l)}\n"),
                "Error: Only 3 elements allowed, but 5 were passed.\
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
    #[allow(unused)]
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
    #[allow(unused)]
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
