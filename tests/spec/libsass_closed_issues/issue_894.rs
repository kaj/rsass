//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_894.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a {/**/}\
            \nb {content: \'something so I have a non-empty expected output\'}"
        )
        .unwrap(),
        "a {\
        \n  /**/\
        \n}\
        \nb {\
        \n  content: \"something so I have a non-empty expected output\";\
        \n}\
        \n"
    );
}
