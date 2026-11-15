//! Tests auto-converted from "sass-spec/spec/callable/arguments/error.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod keyword {
    use super::runner;

    mod escape {
        use super::runner;

        mod rest {
            use super::runner;

            mod quoted {
                use super::runner;

                #[test]
                fn dash_insensitive() {
                    assert_eq!(
                        runner().err(
                            "@function a($b-c) {@return $b-c}\
             \nd {e: a((\"b\\\\5F c\": f)...)}\n"
                        ),
                        "Error: Missing argument $b-c.\
         \n  ,\
         \n1 | @function a($b-c) {@return $b-c}\
         \n  |           ======= declaration\
         \n2 | d {e: a((\"b\\\\5F c\": f)...)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  input.scss 2:7  a()\
         \n  input.scss 2:7  root stylesheet",
                    );
                }
                #[test]
                #[ignore] // wrong error
                fn non_normalized() {
                    assert_eq!(
                        runner().err(
                            "@function a($\\$) {@return $\\$}\
             \nb {c: a((\"\\\\24\": d)...)}\n"
                        ),
                        "Error: Missing argument $\\$.\
         \n  ,\
         \n1 | @function a($\\$) {@return $\\$}\
         \n  |           ====== declaration\
         \n2 | b {c: a((\"\\\\24\": d)...)}\
         \n  |       ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  input.scss 2:7  a()\
         \n  input.scss 2:7  root stylesheet",
                    );
                }
                #[test]
                #[ignore] // wrong error
                fn semantic() {
                    assert_eq!(
                        runner().err(
                            "@function a($\\$) {@return $\\$}\
             \nb {c: a((\"$\": d)...)}\n"
                        ),
                        "Error: Missing argument $\\$.\
         \n  ,\
         \n1 | @function a($\\$) {@return $\\$}\
         \n  |           ====== declaration\
         \n2 | b {c: a((\"$\": d)...)}\
         \n  |       ^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  input.scss 2:7  a()\
         \n  input.scss 2:7  root stylesheet",
                    );
                }
            }
            mod unquoted {
                use super::runner;

                #[test]
                fn dash_insensitive() {
                    assert_eq!(
                        runner().err(
                            "@use \'sass:string\';\
             \n@function a($b-c) {@return $b-c}\
             \nd {e: a((string.unquote(\"b\\\\5f c\"): f)...)}\n"
                        ),
                        "Error: Missing argument $b-c.\
         \n  ,\
         \n2 | @function a($b-c) {@return $b-c}\
         \n  |           ======= declaration\
         \n3 | d {e: a((string.unquote(\"b\\\\5f c\"): f)...)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  input.scss 3:7  a()\
         \n  input.scss 3:7  root stylesheet",
                    );
                }
                #[test]
                #[ignore] // wrong error
                fn non_normalized() {
                    assert_eq!(
                        runner().err(
                            "@use \'sass:string\';\
             \n@function a($\\$) {@return $\\$}\
             \nb {c: a((string.unquote(\"\\\\24\"): d)...)}\n"
                        ),
                        "Error: Missing argument $\\$.\
         \n  ,\
         \n2 | @function a($\\$) {@return $\\$}\
         \n  |           ====== declaration\
         \n3 | b {c: a((string.unquote(\"\\\\24\"): d)...)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  input.scss 3:7  a()\
         \n  input.scss 3:7  root stylesheet",
                    );
                }
                #[test]
                #[ignore] // wrong error
                fn semantic() {
                    assert_eq!(
                        runner().err(
                            "@use \'sass:string\';\
             \n@function a($\\$) {@return $\\$}\
             \nb {c: a((string.unquote(\"$\"): d)...)}\n"
                        ),
                        "Error: Missing argument $\\$.\
         \n  ,\
         \n2 | @function a($\\$) {@return $\\$}\
         \n  |           ====== declaration\
         \n3 | b {c: a((string.unquote(\"$\"): d)...)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  input.scss 3:7  a()\
         \n  input.scss 3:7  root stylesheet",
                    );
                }
            }
        }
    }
}
