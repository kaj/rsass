//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/error.hrx"

mod from_other {
    #[test]
    #[ignore] // wrong error
    fn extend() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\
             \n"
            )
            .unwrap_err(),
            "Error: The target selector was not found.\
         \nUse \"@extend missing !optional\" to avoid this error.\
         \n  ,\
         \n1 | a {@extend missing}\
         \n  |    ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  _other.scss 1:4  load-css()\
         \n  input.scss 2:1   root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn runtime() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\
             \n"
            )
            .unwrap_err(),
            "Error: 1px and 1em have incompatible units.\
         \n  ,\
         \n1 | a {b: 1px + 1em}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  _other.scss 1:7  load-css()\
         \n  input.scss 2:1   root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn syntax() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\
             \n"
            )
            .unwrap_err(),
            "Error: Expected expression.\
         \n  ,\
         \n1 | a {b: }\
         \n  |       ^\
         \n  \'\
         \n  _other.scss 1:7  load-css()\
         \n  input.scss 2:1   root stylesheet\
         \n",
        );
    }
}
mod load {
    #[test]
    #[ignore] // wrong error
    fn test_loop() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\
             \n"
            )
            .unwrap_err(),
            "Error: Module loop: input.scss is already being loaded.\
         \n  ,\
         \n2 | @include meta.load-css(\"input\");\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  _other.scss 2:1  load-css()\
         \n  input.scss 2:1   root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn missing() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\
             \n"
            )
            .unwrap_err(),
            "Error: Can\'t find stylesheet to import.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\");\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet\
         \n",
        );
    }
}
mod member {
    #[test]
    #[ignore] // wrong error
    fn global() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n@meta.load-css(\"other\");\
             \n\
             \na {b: $c}\
             \n"
            )
            .unwrap_err(),
            "Error: Undefined variable.\
         \n  ,\
         \n4 | a {b: $c}\
         \n  |       ^^\
         \n  \'\
         \n  input.scss 4:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn namespace() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n@meta.load-css(\"other\");\
             \n\
             \na {b: other.$c}\
             \n"
            )
            .unwrap_err(),
            "Error: There is no module with the namespace \"other\".\
         \n  ,\
         \n4 | a {b: other.$c}\
         \n  |       ^^^^^^^^\
         \n  \'\
         \n  input.scss 4:7  root stylesheet\
         \n",
        );
    }
}
#[test]
#[ignore] // wrong error
fn too_few_args() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
             \n@include meta.load-css();\
             \n"
        )
        .unwrap_err(),
        "Error: Missing argument $url.\
         \n  ,--> input.scss\
         \n2 | @include meta.load-css();\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @mixin load-css($url, $with: null) {\
         \n  |        =========================== declaration\
         \n  \'\
         \n  input.scss 2:1  root stylesheet\
         \n",
    );
}
#[test]
#[ignore] // wrong error
fn too_many_args() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", (), \"a\");\
             \n"
        )
        .unwrap_err(),
        "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n2 | @include meta.load-css(\"other\", (), \"a\");\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @mixin load-css($url, $with: null) {\
         \n  |        =========================== declaration\
         \n  \'\
         \n  input.scss 2:1  root stylesheet\
         \n",
    );
}
mod test_type {
    #[test]
    #[ignore] // wrong error
    fn url() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n@include meta.load-css(1);\
             \n"
            )
            .unwrap_err(),
            "Error: $url: 1 is not a string.\
         \n  ,\
         \n2 | @include meta.load-css(1);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet\
         \n",
        );
    }
    mod with {
        #[test]
        #[ignore] // wrong error
        fn key() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (1: null));\
             \n"
                )
                .unwrap_err(),
                "Error: $with key: 1 is not a string.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (1: null));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet\
         \n",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn map() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: 1);\
             \n"
                )
                .unwrap_err(),
                "Error: $with: 1 is not a map.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: 1);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet\
         \n",
            );
        }
    }
}
mod with {
    #[test]
    #[ignore] // wrong error
    fn conflict() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"midstream\", $with: (a: b));\
             \n"
            )
            .unwrap_err(),
            "Error: This variable is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"left\" as *;\
         \n    | ================ includes variable\
         \n2   | @use \"right\" as *;\
         \n    | ================= includes variable\
         \n... |\
         \n4   | $a: c !default;\
         \n    | ^^^^^^^^^^^^^^ variable use\
         \n    \'\
         \n  _midstream.scss 4:1  load-css()\
         \n  input.scss 2:1       root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn core_module() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"sass:color\", $with: (a: b));\
             \n"
            )
            .unwrap_err(),
            "Error: Built-in module sass:color can\'t be configured.\
         \n  ,\
         \n2 | @include meta.load-css(\"sass:color\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet\
         \n",
        );
    }
    mod multi_configuration {
        mod double_load {
            #[test]
            #[ignore] // wrong error
            fn both_configured() {
                assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a: b));\
             \n@include meta.load-css(\"other\", $with: (a: b));\
             \n"
        ).unwrap_err(),
        "Error: _other.scss was already loaded, so it can\'t be configured using \"with\".\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ============================================== original load\
         \n3 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  input.scss 3:1  root stylesheet\
         \n",
    );
            }
            #[test]
            #[ignore] // wrong error
            fn through_forward() {
                assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"forwarded\");\
             \n@include meta.load-css(\"midstream\", $with: (a: b));\
             \n"
        ).unwrap_err(),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,--> _midstream.scss\
         \n1 | @forward \"forwarded\";\
         \n  | ^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  ,--> input.scss\
         \n2 | @include meta.load-css(\"forwarded\");\
         \n  | =================================== original load\
         \n3 | @include meta.load-css(\"midstream\", $with: (a: b));\
         \n  | ================================================== configuration\
         \n  \'\
         \n  _midstream.scss 1:1  load-css()\
         \n  input.scss 3:1       root stylesheet\
         \n",
    );
            }
            #[test]
            #[ignore] // wrong error
            fn unconfigured_first() {
                assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\");\
             \n@include meta.load-css(\"other\", $with: (a: b));\
             \n"
        ).unwrap_err(),
        "Error: _other.scss was already loaded, so it can\'t be configured using \"with\".\
         \n  ,\
         \n2 | @include meta.load-css(\"other\");\
         \n  | =============================== original load\
         \n3 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  input.scss 3:1  root stylesheet\
         \n",
    );
            }
        }
        mod use_and_load {
            #[test]
            #[ignore] // wrong error
            fn both_configured() {
                assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
             \n@use \"other\" with ($a: b);\
             \n@include meta.load-css(\"other\", $with: (a: b));\
             \n"
        ).unwrap_err(),
        "Error: _other.scss was already loaded, so it can\'t be configured using \"with\".\
         \n  ,\
         \n2 | @use \"other\" with ($a: b);\
         \n  | ========================= original load\
         \n3 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  input.scss 3:1  root stylesheet\
         \n",
    );
            }
            #[test]
            #[ignore] // wrong error
            fn load_first() {
                assert_eq!(
        crate::rsass(
            "// This indirection is necessary so that we can execute `meta.load-css()` before\
             \n// we begin loading `used`.\
             \n@use \"loads\";\
             \n@use \"other\" with ($a: b);\
             \n"
        ).unwrap_err(),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,--> input.scss\
         \n4 | @use \"other\" with ($a: b);\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  ,--> _loads.scss\
         \n2 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ============================================== original load\
         \n  \'\
         \n  input.scss 4:1  root stylesheet\
         \n",
    );
            }
            #[test]
            #[ignore] // wrong error
            fn through_forward() {
                assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
             \n@use \"forwarded\";\
             \n@include meta.load-css(\"midstream\", $with: (a: b));\
             \n"
        ).unwrap_err(),
        "Error: This module was already loaded, so it can\'t be configured using \"with\".\
         \n  ,--> _midstream.scss\
         \n1 | @forward \"forwarded\";\
         \n  | ^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  ,--> input.scss\
         \n2 | @use \"forwarded\";\
         \n  | ================ original load\
         \n3 | @include meta.load-css(\"midstream\", $with: (a: b));\
         \n  | ================================================== configuration\
         \n  \'\
         \n  _midstream.scss 1:1  load-css()\
         \n  input.scss 3:1       root stylesheet\
         \n",
    );
            }
            #[test]
            #[ignore] // wrong error
            fn unconfigured_first() {
                assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
             \n@use \"other\";\
             \n@include meta.load-css(\"other\", $with: (a: b));\
             \n"
        ).unwrap_err(),
        "Error: _other.scss was already loaded, so it can\'t be configured using \"with\".\
         \n  ,\
         \n2 | @use \"other\";\
         \n  | ============ original load\
         \n3 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ new load\
         \n  \'\
         \n  input.scss 3:1  root stylesheet\
         \n",
    );
            }
        }
    }
    #[test]
    #[ignore] // wrong error
    fn namespace() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"midstream\", $with: (a: b));\
             \n"
            )
            .unwrap_err(),
            "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"midstream\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn nested() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a: b));\
             \n"
            )
            .unwrap_err(),
            "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn not_default() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a: b));\
             \n"
            )
            .unwrap_err(),
            "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn repeated_variable() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a-b: c, a_b: c));\
             \n"
            )
            .unwrap_err(),
            "Error: The variable $a-b was configured twice.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (a-b: c, a_b: c));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet\
         \n",
        );
    }
    mod through_forward {
        #[test]
        #[ignore] // wrong error
        fn test_as() {
            assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"used\", $with: (a: b));\
             \n"
        ).unwrap_err(),
        "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"used\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet\
         \n",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn hide() {
            assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"used\", $with: (a: b));\
             \n"
        ).unwrap_err(),
        "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"used\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet\
         \n",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn show() {
            assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"used\", $with: (a: b));\
             \n"
        ).unwrap_err(),
        "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"used\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet\
         \n",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn with() {
            assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"used\", $with: (a: b));\
             \n"
        ).unwrap_err(),
        "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"used\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet\
         \n",
    );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn undefined() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (a: b));\
             \n"
            )
            .unwrap_err(),
            "Error: $a was not declared with !default in the @used module.\
         \n  ,\
         \n2 | @include meta.load-css(\"other\", $with: (a: b));\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:1  root stylesheet\
         \n",
        );
    }
}
