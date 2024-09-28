//! Tests auto-converted from "sass-spec/spec/directives/import/comment.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("comment")
}

mod after_comma {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn loud() {
        assert_eq!(
            runner().ok("@import \"a.css\", /**/ \"b.css\"\n"),
            "@import \"a.css\";\
         \n@import \"b.css\";\n"
        );
    }
    #[test]
    fn silent() {
        assert_eq!(
            runner().ok("@import \"a.css\", //\
             \n  \"b.css\"\n"),
            "@import \"a.css\";\
         \n@import \"b.css\";\n"
        );
    }
}
mod after_url {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn loud() {
        assert_eq!(
            runner().ok("@import \"a.css\" /**/\n"),
            "@import \"a.css\";\n"
        );
    }
    #[test]
    fn silent() {
        assert_eq!(
            runner().ok("@import \"a.css\" //\n"),
            "@import \"a.css\";\n"
        );
    }
}
mod before_comma {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn loud() {
        assert_eq!(
            runner().ok("@import \"a.css\" /**/, \"b.css\"\n"),
            "@import \"a.css\";\
         \n@import \"b.css\";\n"
        );
    }
    #[test]
    fn silent() {
        assert_eq!(
            runner().ok("@import \"a.css\" //\
             \n  , \"b.css\"\n"),
            "@import \"a.css\";\
         \n@import \"b.css\";\n"
        );
    }
}
mod before_url {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn loud() {
        assert_eq!(
            runner().ok("@import /**/ \"a.css\"\n"),
            "@import \"a.css\";\n"
        );
    }
    #[test]
    fn silent() {
        assert_eq!(
            runner().ok("@import //\
             \n  \"a.css\"\n"),
            "@import \"a.css\";\n"
        );
    }
}
mod modifier {
    #[allow(unused)]
    use super::runner;

    mod args {
        #[allow(unused)]
        use super::runner;

        mod after {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("@import \"a.css\" b(c) /**/\n"),
                    "@import \"a.css\" b(c);\n"
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@import \"a.css\" b(c) //\n"),
                    "@import \"a.css\" b(c);\n"
                );
            }
        }
        mod after_open_paren {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("@import \"a.css\" b(/**/ c)\n"),
                    "@import \"a.css\" b(/**/ c);\n"
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@import \"a.css\" b(c)\n"),
                    "@import \"a.css\" b(c);\n"
                );
            }
        }
        mod before {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("@import \"a.css\" /**/ b(c)\n"),
                    "@import \"a.css\" b(c);\n"
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@import \"a.css\" //\
             \n  b(c)\n"),
                    "@import \"a.css\" b(c);\n"
                );
            }
        }
        mod before_close_paren {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn loud() {
                assert_eq!(
                    runner().ok("@import \"a.css\" b(c /**/)\n"),
                    "@import \"a.css\" b(c /**/);\n"
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@import \"a.css\" b(c)\n"),
                    "@import \"a.css\" b(c);\n"
                );
            }
        }
    }
    mod no_args {
        #[allow(unused)]
        use super::runner;

        mod after {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("@import \"a.css\" b /**/\n"),
                    "@import \"a.css\" b;\n"
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@import \"a.css\" b //\n"),
                    "@import \"a.css\" b;\n"
                );
            }
        }
        mod before {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("@import \"a.css\" /**/ b\n"),
                    "@import \"a.css\" b;\n"
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@import \"a.css\" //\
             \n  b\n"),
                    "@import \"a.css\" b;\n"
                );
            }
        }
    }
}
