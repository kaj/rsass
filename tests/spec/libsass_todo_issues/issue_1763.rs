//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_1763.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \"first.css\", \"second.css\" (max-width: 400px);\
            \n@import \"first.scss\", \"second.scss\" (max-width: 400px);\
            \n"
        )
        .unwrap(),
        "@import \"first.css\";\
        \n@import \"second.css\" (max-width: 400px);\
        \n@import \"second.scss\" (max-width: 400px);\
        \nfoo {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}
