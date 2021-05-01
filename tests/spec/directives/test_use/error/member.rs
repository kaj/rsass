//! Tests auto-converted from "sass-spec/spec/directives/use/error/member.hrx"

mod before_use {
    #[test]
    #[ignore] // wrong error
    fn function() {
        assert_eq!(
            crate::rsass(
                "$variable: other.member();\
             \n@use \"other\";\
             \n"
            )
            .unwrap_err(),
            "Error: There is no module with the namespace \"other\".\
         \n  ,\
         \n1 | $variable: other.member();\
         \n  |            ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn variable_declaration() {
        assert_eq!(
            crate::rsass(
                "other.$member: value;\
             \n@use \"other\";\
             \n"
            )
            .unwrap_err(),
            "Error: There is no module with the namespace \"other\".\
         \n  ,\
         \n1 | other.$member: value;\
         \n  | ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn variable_declaration_without_namespace() {
        assert_eq!(
        crate::rsass(
            "$member: from input;\
             \n\
             \n@use \"other\" as *;\
             \n\
             \na {b: $member}\
             \n"
        ).unwrap_err(),
        "Error: This module and the new module both define a variable named \"$member\".\
         \n  ,\
         \n3 | @use \"other\" as *;\
         \n  | ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn variable_use() {
        assert_eq!(
            crate::rsass(
                "$variable: other.$member;\
             \n@use \"other\";\
             \n"
            )
            .unwrap_err(),
            "Error: There is no module with the namespace \"other\".\
         \n  ,\
         \n1 | $variable: other.$member;\
         \n  |            ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
}
mod conflict {
    #[test]
    #[ignore] // wrong error
    fn function() {
        assert_eq!(
            crate::rsass(
                "@use \"other1\" as *;\
             \n@use \"other2\" as *;\
             \n\
             \na {b: member()}\
             \n"
            )
            .unwrap_err(),
            "Error: This function is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"other1\" as *;\
         \n    | ================== includes function\
         \n2   | @use \"other2\" as *;\
         \n    | ================== includes function\
         \n... |\
         \n4   | a {b: member()}\
         \n    |       ^^^^^^^^ function use\
         \n    \'\
         \n  input.scss 4:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn mixin() {
        assert_eq!(
            crate::rsass(
                "@use \"other1\" as *;\
             \n@use \"other2\" as *;\
             \n\
             \na {@include member}\
             \n"
            )
            .unwrap_err(),
            "Error: This mixin is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"other1\" as *;\
         \n    | ================== includes mixin\
         \n2   | @use \"other2\" as *;\
         \n    | ================== includes mixin\
         \n... |\
         \n4   | a {@include member}\
         \n    |    ^^^^^^^^^^^^^^^ mixin use\
         \n    \'\
         \n  input.scss 4:4  root stylesheet",
        );
    }
    mod same_value {
        #[test]
        #[ignore] // wrong error
        fn function() {
            assert_eq!(
        crate::rsass(
            "@use \"other1\" as *;\
             \n@use \"other2\" as *;\
             \n\
             \na {b: c()}\
             \n"
        ).unwrap_err(),
        "Error: This function is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"other1\" as *;\
         \n    | ================== includes function\
         \n2   | @use \"other2\" as *;\
         \n    | ================== includes function\
         \n... |\
         \n4   | a {b: c()}\
         \n    |       ^^^ function use\
         \n    \'\
         \n  input.scss 4:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn mixin() {
            assert_eq!(
        crate::rsass(
            "@use \"other1\" as *;\
             \n@use \"other2\" as *;\
             \n\
             \na {@include b}\
             \n"
        ).unwrap_err(),
        "Error: This mixin is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"other1\" as *;\
         \n    | ================== includes mixin\
         \n2   | @use \"other2\" as *;\
         \n    | ================== includes mixin\
         \n... |\
         \n4   | a {@include b}\
         \n    |    ^^^^^^^^^^ mixin use\
         \n    \'\
         \n  input.scss 4:4  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn variable() {
            assert_eq!(
        crate::rsass(
            "@use \"other1\" as *;\
             \n@use \"other2\" as *;\
             \n\
             \na {b: $c}\
             \n"
        ).unwrap_err(),
        "Error: This variable is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"other1\" as *;\
         \n    | ================== includes variable\
         \n2   | @use \"other2\" as *;\
         \n    | ================== includes variable\
         \n... |\
         \n4   | a {b: $c}\
         \n    |       ^^ variable use\
         \n    \'\
         \n  input.scss 4:7  root stylesheet",
    );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn variable() {
        assert_eq!(
            crate::rsass(
                "@use \"other1\" as *;\
             \n@use \"other2\" as *;\
             \n\
             \na {b: $member}\
             \n"
            )
            .unwrap_err(),
            "Error: This variable is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"other1\" as *;\
         \n    | ================== includes variable\
         \n2   | @use \"other2\" as *;\
         \n    | ================== includes variable\
         \n... |\
         \n4   | a {b: $member}\
         \n    |       ^^^^^^^ variable use\
         \n    \'\
         \n  input.scss 4:7  root stylesheet",
        );
    }
}
mod inaccessible {
    mod private {
        #[test]
        #[ignore] // unexepected error
        fn function() {
            assert_eq!(
        crate::rsass(
            "@use \"other\" as *;\
            \n\
            \n// This is technically not a compile error, since `-member()` is treated as\
            \n// plain CSS, but it\'s included here for consistency with the other specs.\
            \na {b: -member()};\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: -member();\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // wrong error
        fn mixin() {
            assert_eq!(
                crate::rsass(
                    "@use \"other\" as *;\
             \n\
             \n@include -member;\
             \n"
                )
                .unwrap_err(),
                "Error: Undefined mixin.\
         \n  ,\
         \n3 | @include -member;\
         \n  | ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn variable() {
            assert_eq!(
                crate::rsass(
                    "@use \"other\" as *;\
             \n\
             \na {b: $-member};\
             \n"
                )
                .unwrap_err(),
                "Error: Undefined variable.\
         \n  ,\
         \n3 | a {b: $-member};\
         \n  |       ^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
            );
        }
    }
    mod transitive {
        #[test]
        #[ignore] // unexepected error
        fn function() {
            assert_eq!(
        crate::rsass(
            "@use \"midstream\" as *;\
            \n\
            \n// This is technically not a compile error, since `-member()` is treated as\
            \n// plain CSS, but it\'s included here for consistency with the other specs.\
            \na {b: upstream()};\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: upstream();\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // wrong error
        fn mixin() {
            assert_eq!(
                crate::rsass(
                    "@use \"midstream\" as *;\
             \n\
             \n@include upstream;\
             \n"
                )
                .unwrap_err(),
                "Error: Undefined mixin.\
         \n  ,\
         \n3 | @include upstream;\
         \n  | ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn variable() {
            assert_eq!(
                crate::rsass(
                    "@use \"midstream\" as *;\
             \n\
             \na {b: $upstream};\
             \n"
                )
                .unwrap_err(),
                "Error: Undefined variable.\
         \n  ,\
         \n3 | a {b: $upstream};\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
            );
        }
    }
    mod transitive_from_import {
        #[test]
        #[ignore] // wrong result
        fn function() {
            assert_eq!(
        crate::rsass(
            "@import \"midstream\";\
            \n\
            \n// This is technically not a compile error, since `upstream()` is treated as\
            \n// plain CSS, but it\'s included here for consistency with the other specs.\
            \na {b: upstream()};\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: upstream();\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // wrong error
        fn mixin() {
            assert_eq!(
                crate::rsass(
                    "@import \"midstream\";\
             \n\
             \n@include upstream;\
             \n"
                )
                .unwrap_err(),
                "Error: Undefined mixin.\
         \n  ,\
         \n3 | @include upstream;\
         \n  | ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn variable() {
            assert_eq!(
                crate::rsass(
                    "@import \"midstream\";\
             \n\
             \na {b: $upstream};\
             \n"
                )
                .unwrap_err(),
                "Error: Undefined variable.\
         \n  ,\
         \n3 | a {b: $upstream};\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
            );
        }
    }
}
mod missing {
    mod global {
        #[test]
        #[ignore] // wrong error
        fn mixin() {
            assert_eq!(
                crate::rsass(
                    "@use \"other\";\
             \n\
             \n@include member;\
             \n"
                )
                .unwrap_err(),
                "Error: Undefined mixin.\
         \n  ,\
         \n3 | @include member;\
         \n  | ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn variable() {
            assert_eq!(
                crate::rsass(
                    "@use \"other\";\
             \n\
             \na {b: $member}\
             \n"
                )
                .unwrap_err(),
                "Error: Undefined variable.\
         \n  ,\
         \n3 | a {b: $member}\
         \n  |       ^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
            );
        }
    }
    mod namespaced {
        #[test]
        #[ignore] // wrong error
        fn function() {
            assert_eq!(
                crate::rsass(
                    "@use \"other\";\
             \n\
             \na {b: other.member()}\
             \n"
                )
                .unwrap_err(),
                "Error: Undefined function.\
         \n  ,\
         \n3 | a {b: other.member()}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn mixin() {
            assert_eq!(
                crate::rsass(
                    "@use \"other\";\
             \n\
             \n@include other.member;\
             \n"
                )
                .unwrap_err(),
                "Error: Undefined mixin.\
         \n  ,\
         \n3 | @include other.member;\
         \n  | ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn variable_declaration() {
            assert_eq!(
                crate::rsass(
                    "@use \"other\";\
             \n\
             \nother.$member: value;\
             \n"
                )
                .unwrap_err(),
                "Error: Undefined variable.\
         \n  ,\
         \n3 | other.$member: value;\
         \n  | ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn variable_use() {
            assert_eq!(
                crate::rsass(
                    "@use \"other\";\
             \n\
             \na {b: other.$member}\
             \n"
                )
                .unwrap_err(),
                "Error: Undefined variable.\
         \n  ,\
         \n3 | a {b: other.$member}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:7  root stylesheet",
            );
        }
    }
}
