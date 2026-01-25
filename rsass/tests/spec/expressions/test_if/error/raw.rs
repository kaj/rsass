//! Tests auto-converted from "sass-spec/spec/expressions/if/error/raw.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("raw")
}

mod and {
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn test_else() {
        assert_eq!(
            runner().err("a {b: if(css() var(--and) else: c)}\n"),
            "Error: expected \"(\".\
         \n  ,\
         \n1 | a {b: if(css() var(--and) else: c)}\
         \n  |                               ^\
         \n  \'\
         \n  input.scss 1:31  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn not() {
        assert_eq!(
            runner().err("a {b: if(css(1) var(--and) not css(2): c)}\n"),
            "Error: expected \"(\".\
         \n  ,\
         \n1 | a {b: if(css(1) var(--and) not css(2): c)}\
         \n  |                               ^\
         \n  \'\
         \n  input.scss 1:31  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn or() {
        assert_eq!(
        runner().err(
            "a {b: if(css(1) and css(2) var(--and) vss(3) or css(4): c)}\n"
        ),
        "Error: expected \":\".\
         \n  ,\
         \n1 | a {b: if(css(1) and css(2) var(--and) vss(3) or css(4): c)}\
         \n  |                                              ^\
         \n  \'\
         \n  input.scss 1:46  root stylesheet",
    );
    }
}
mod not {
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn test_else() {
        assert_eq!(
            runner().err("a {b: if(var(--not) else: c)}\n"),
            "Error: expected \"(\".\
         \n  ,\
         \n1 | a {b: if(var(--not) else: c)}\
         \n  |                         ^\
         \n  \'\
         \n  input.scss 1:25  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn not() {
        assert_eq!(
            runner().err("a {b: if(not var(--not) css(): c)}\n"),
            "Error: expected \":\".\
         \n  ,\
         \n1 | a {b: if(not var(--not) css(): c)}\
         \n  |                         ^\
         \n  \'\
         \n  input.scss 1:25  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn operator() {
        assert_eq!(
            runner().err("a {b: if(not css(1) var(--and) css(2): c)}\n"),
            "Error: expected \":\".\
         \n  ,\
         \n1 | a {b: if(not css(1) var(--and) css(2): c)}\
         \n  |                     ^\
         \n  \'\
         \n  input.scss 1:21  root stylesheet",
        );
    }
}
mod or {
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn and() {
        assert_eq!(
        runner().err(
            "a {b: if(css(1) or css(2) var(--or) vss(3) and css(4): c)}\n"
        ),
        "Error: expected \":\".\
         \n  ,\
         \n1 | a {b: if(css(1) or css(2) var(--or) vss(3) and css(4): c)}\
         \n  |                                            ^\
         \n  \'\
         \n  input.scss 1:44  root stylesheet",
    );
    }
    #[test]
    #[ignore] // missing error
    fn test_else() {
        assert_eq!(
            runner().err("a {b: if(css() var(--or) else: c)}\n"),
            "Error: expected \"(\".\
         \n  ,\
         \n1 | a {b: if(css() var(--or) else: c)}\
         \n  |                              ^\
         \n  \'\
         \n  input.scss 1:30  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn not() {
        assert_eq!(
            runner().err("a {b: if(css(1) var(--or) not css(2): c)}\n"),
            "Error: expected \"(\".\
         \n  ,\
         \n1 | a {b: if(css(1) var(--or) not css(2): c)}\
         \n  |                              ^\
         \n  \'\
         \n  input.scss 1:30  root stylesheet",
        );
    }
}
mod paren {
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn clause() {
        assert_eq!(
            runner().err("a {b: if(css(1) (var(--and-clause)): c)}\n"),
            "Error: expected \":\".\
         \n  ,\
         \n1 | a {b: if(css(1) (var(--and-clause)): c)}\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn not() {
        assert_eq!(
            runner().err("a {b: if((var(--not)) css(): c)}\n"),
            "Error: expected \":\".\
         \n  ,\
         \n1 | a {b: if((var(--not)) css(): c)}\
         \n  |                       ^\
         \n  \'\
         \n  input.scss 1:23  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn operator() {
        assert_eq!(
            runner().err("a {b: if(css(1) (var(--and)) css(2): c)}\n"),
            "Error: expected \":\".\
         \n  ,\
         \n1 | a {b: if(css(1) (var(--and)) css(2): c)}\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
        );
    }
}
mod with_sass {
    use super::runner;

    mod adjacent {
        use super::runner;

        mod after {
            use super::runner;

            mod t1 {
                use super::runner;

                #[test]
                #[ignore] // missing error
                fn direct() {
                    assert_eq!(
        runner().err(
            "a {b: if(var(--not) sass(true): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if(var(--not) sass(true): c)}\
         \n  |          ^^^^^^^^^^ arbitrary substitution\
         \n  |                     ========== sass() expression\
         \n  \'\
         \n  input.scss 1:10  root stylesheet",
    );
                }
                #[test]
                #[ignore] // missing error
                fn nested() {
                    assert_eq!(
        runner().err(
            "a {b: if(var(--not) (sass(true)): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if(var(--not) (sass(true)): c)}\
         \n  |          ^^^^^^^^^^ arbitrary substitution\
         \n  |                      ========== sass() expression\
         \n  \'\
         \n  input.scss 1:10  root stylesheet",
    );
                }
            }
            mod t2 {
                use super::runner;

                #[test]
                #[ignore] // missing error
                fn direct() {
                    assert_eq!(
        runner().err(
            "a {b: if(var(--clause) var(--and) sass(true): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if(var(--clause) var(--and) sass(true): c)}\
         \n  |          ^^^^^^^^^^^^^ arbitrary substitution\
         \n  |                                   ========== sass() expression\
         \n  \'\
         \n  input.scss 1:10  root stylesheet",
    );
                }
                #[test]
                #[ignore] // missing error
                fn nested() {
                    assert_eq!(
        runner().err(
            "a {b: if(var(--clause) var(--and) (sass(true)): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if(var(--clause) var(--and) (sass(true)): c)}\
         \n  |          ^^^^^^^^^^^^^ arbitrary substitution\
         \n  |                                    ========== sass() expression\
         \n  \'\
         \n  input.scss 1:10  root stylesheet",
    );
                }
            }
        }
        mod before {
            use super::runner;

            #[test]
            #[ignore] // missing error
            fn direct() {
                assert_eq!(
        runner().err(
            "a {b: if(sass(true) var(--and-clause): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if(sass(true) var(--and-clause): c)}\
         \n  |                     ^^^^^^^^^^^^^^^^^ arbitrary substitution\
         \n  |          ========== sass() expression\
         \n  \'\
         \n  input.scss 1:21  root stylesheet",
    );
            }
            #[test]
            #[ignore] // missing error
            fn nested() {
                assert_eq!(
        runner().err(
            "a {b: if((sass(true)) var(--and-clause): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if((sass(true)) var(--and-clause): c)}\
         \n  |                       ^^^^^^^^^^^^^^^^^ arbitrary substitution\
         \n  |           ========== sass() expression\
         \n  \'\
         \n  input.scss 1:23  root stylesheet",
    );
            }
        }
    }
    mod and {
        use super::runner;

        mod after {
            use super::runner;

            mod clause {
                use super::runner;

                #[test]
                #[ignore] // missing error
                fn direct() {
                    assert_eq!(
        runner().err(
            "a {b: if(css(1) and var(--clause-and) sass(true): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if(css(1) and var(--clause-and) sass(true): c)}\
         \n  |                     ^^^^^^^^^^^^^^^^^ arbitrary substitution\
         \n  |                                       ========== sass() expression\
         \n  \'\
         \n  input.scss 1:21  root stylesheet",
    );
                }
                #[test]
                #[ignore] // missing error
                fn nested() {
                    assert_eq!(
        runner().err(
            "a {b: if(css(1) and var(--clause-and) (sass(true)): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if(css(1) and var(--clause-and) (sass(true)): c)}\
         \n  |                     ^^^^^^^^^^^^^^^^^ arbitrary substitution\
         \n  |                                        ========== sass() expression\
         \n  \'\
         \n  input.scss 1:21  root stylesheet",
    );
                }
            }
            mod operator {
                use super::runner;

                #[test]
                #[ignore] // missing error
                fn direct() {
                    assert_eq!(
        runner().err(
            "a {b: if(css(1) and css(2) var(--and) sass(true): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if(css(1) and css(2) var(--and) sass(true): c)}\
         \n  |                            ^^^^^^^^^^ arbitrary substitution\
         \n  |                                       ========== sass() expression\
         \n  \'\
         \n  input.scss 1:28  root stylesheet",
    );
                }
                #[test]
                #[ignore] // missing error
                fn nested() {
                    assert_eq!(
        runner().err(
            "a {b: if(css(1) and css(2) var(--and) (sass(true)): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if(css(1) and css(2) var(--and) (sass(true)): c)}\
         \n  |                            ^^^^^^^^^^ arbitrary substitution\
         \n  |                                        ========== sass() expression\
         \n  \'\
         \n  input.scss 1:28  root stylesheet",
    );
                }
            }
        }
        mod before {
            use super::runner;

            mod clause {
                use super::runner;

                #[test]
                #[ignore] // missing error
                fn direct() {
                    assert_eq!(
        runner().err(
            "a {b: if(sass(true) and css(1) var(--and-clause): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if(sass(true) and css(1) var(--and-clause): c)}\
         \n  |                                ^^^^^^^^^^^^^^^^^ arbitrary substitution\
         \n  |          ========== sass() expression\
         \n  \'\
         \n  input.scss 1:32  root stylesheet",
    );
                }
                #[test]
                #[ignore] // missing error
                fn nested() {
                    assert_eq!(
        runner().err(
            "a {b: if((sass(true)) and css(1) var(--and-clause): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if((sass(true)) and css(1) var(--and-clause): c)}\
         \n  |                                  ^^^^^^^^^^^^^^^^^ arbitrary substitution\
         \n  |           ========== sass() expression\
         \n  \'\
         \n  input.scss 1:34  root stylesheet",
    );
                }
            }
            mod operator {
                use super::runner;

                #[test]
                #[ignore] // missing error
                fn direct() {
                    assert_eq!(
        runner().err(
            "a {b: if(sass(true) and css(1) var(--and) css(2): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if(sass(true) and css(1) var(--and) css(2): c)}\
         \n  |                                ^^^^^^^^^^ arbitrary substitution\
         \n  |          ========== sass() expression\
         \n  \'\
         \n  input.scss 1:32  root stylesheet",
    );
                }
                #[test]
                #[ignore] // missing error
                fn nested() {
                    assert_eq!(
        runner().err(
            "a {b: if((sass(true)) and css(1) var(--and) css(2): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if((sass(true)) and css(1) var(--and) css(2): c)}\
         \n  |                                  ^^^^^^^^^^ arbitrary substitution\
         \n  |           ========== sass() expression\
         \n  \'\
         \n  input.scss 1:34  root stylesheet",
    );
                }
            }
        }
    }
    mod or {
        use super::runner;

        mod after {
            use super::runner;

            mod clause {
                use super::runner;

                #[test]
                #[ignore] // missing error
                fn direct() {
                    assert_eq!(
        runner().err(
            "a {b: if(css(1) or var(--clause-or) sass(true): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if(css(1) or var(--clause-or) sass(true): c)}\
         \n  |                    ^^^^^^^^^^^^^^^^ arbitrary substitution\
         \n  |                                     ========== sass() expression\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
    );
                }
                #[test]
                #[ignore] // missing error
                fn nested() {
                    assert_eq!(
        runner().err(
            "a {b: if(css(1) or var(--clause-or) (sass(true)): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if(css(1) or var(--clause-or) (sass(true)): c)}\
         \n  |                    ^^^^^^^^^^^^^^^^ arbitrary substitution\
         \n  |                                      ========== sass() expression\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
    );
                }
            }
            mod operator {
                use super::runner;

                #[test]
                #[ignore] // missing error
                fn direct() {
                    assert_eq!(
        runner().err(
            "a {b: if(css(1) or css(2) var(--or) sass(true): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if(css(1) or css(2) var(--or) sass(true): c)}\
         \n  |                           ^^^^^^^^^ arbitrary substitution\
         \n  |                                     ========== sass() expression\
         \n  \'\
         \n  input.scss 1:27  root stylesheet",
    );
                }
                #[test]
                #[ignore] // missing error
                fn nested() {
                    assert_eq!(
        runner().err(
            "a {b: if(css(1) or css(2) var(--or) (sass(true)): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if(css(1) or css(2) var(--or) (sass(true)): c)}\
         \n  |                           ^^^^^^^^^ arbitrary substitution\
         \n  |                                      ========== sass() expression\
         \n  \'\
         \n  input.scss 1:27  root stylesheet",
    );
                }
            }
        }
        mod before {
            use super::runner;

            mod clause {
                use super::runner;

                #[test]
                #[ignore] // missing error
                fn direct() {
                    assert_eq!(
        runner().err(
            "a {b: if(sass(true) or css(1) var(--or-clause): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if(sass(true) or css(1) var(--or-clause): c)}\
         \n  |                               ^^^^^^^^^^^^^^^^ arbitrary substitution\
         \n  |          ========== sass() expression\
         \n  \'\
         \n  input.scss 1:31  root stylesheet",
    );
                }
                #[test]
                #[ignore] // missing error
                fn nested() {
                    assert_eq!(
        runner().err(
            "a {b: if((sass(true)) or css(1) var(--or-clause): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if((sass(true)) or css(1) var(--or-clause): c)}\
         \n  |                                 ^^^^^^^^^^^^^^^^ arbitrary substitution\
         \n  |           ========== sass() expression\
         \n  \'\
         \n  input.scss 1:33  root stylesheet",
    );
                }
            }
            mod operator {
                use super::runner;

                #[test]
                #[ignore] // missing error
                fn direct() {
                    assert_eq!(
        runner().err(
            "a {b: if(sass(true) or css(1) var(--or) css(2): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if(sass(true) or css(1) var(--or) css(2): c)}\
         \n  |                               ^^^^^^^^^ arbitrary substitution\
         \n  |          ========== sass() expression\
         \n  \'\
         \n  input.scss 1:31  root stylesheet",
    );
                }
                #[test]
                #[ignore] // missing error
                fn nested() {
                    assert_eq!(
        runner().err(
            "a {b: if((sass(true)) or css(1) var(--or) css(2): c)}\n"
        ),
        "Error: if() conditions with arbitrary substitutions may not contain sass() expressions.\
         \n  ,\
         \n1 | a {b: if((sass(true)) or css(1) var(--or) css(2): c)}\
         \n  |                                 ^^^^^^^^^ arbitrary substitution\
         \n  |           ========== sass() expression\
         \n  \'\
         \n  input.scss 1:33  root stylesheet",
    );
                }
            }
        }
    }
}
