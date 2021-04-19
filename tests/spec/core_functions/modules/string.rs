//! Tests auto-converted from "sass-spec/spec/core_functions/modules/string.hrx"

mod error {
    #[test]
    fn str_index() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:string\";\
             \na {b: string.str-index(\"c\", \"c\")}\
             \n"
            )
            .unwrap_err(),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: string.str-index(\"c\", \"c\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn str_insert() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:string\";\
             \na {b: string.str-insert(\"c\", 1, \"d\")}\
             \n"
            )
            .unwrap_err(),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: string.str-insert(\"c\", 1, \"d\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn str_length() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:string\";\
             \na {b: string.str-length(\"c\")}\
             \n"
            )
            .unwrap_err(),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: string.str-length(\"c\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn str_slice() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:string\";\
             \na {b: string.str-slice(\"c\", 1, 1)}\
             \n"
            )
            .unwrap_err(),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: string.str-slice(\"c\", 1, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn index() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:string\";\
            \na {b: string.index(\"c\", \"c\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
#[test]
fn insert() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:string\";\
            \na {b: string.insert(\"c\", \"d\", 1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"dc\";\
        \n}\
        \n"
    );
}
#[test]
fn length() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:string\";\
            \na {b: string.length(\"c\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
#[test]
fn quote() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:string\";\
            \na {b: string.quote(c)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"c\";\
        \n}\
        \n"
    );
}
#[test]
fn slice() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:string\";\
            \na {b: string.slice(\"c\", 1, 1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"c\";\
        \n}\
        \n"
    );
}
#[test]
fn to_upper_case() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:string\";\
            \na {b: string.to-upper-case(\"c\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"C\";\
        \n}\
        \n"
    );
}
#[test]
fn unique_id() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:meta\";\
            \n@use \"sass:string\";\
            \na {b: meta.type-of(string.unique-id())}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: string;\
        \n}\
        \n"
    );
}
#[test]
fn unquote() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:string\";\
            \na {b: string.unquote(\"c\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}
