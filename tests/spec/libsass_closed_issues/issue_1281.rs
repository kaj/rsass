//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1281.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$quoted: \"green\";\
            \n$unquoted: green;\
            \n\
            \n.test {\
            \n  string: type-of($quoted);\
            \n  color: type-of($unquoted);\
            \n  string: type-of(\"green\");\
            \n  color: type-of(green);\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  string: string;\
        \n  color: color;\
        \n  string: string;\
        \n  color: color;\
        \n}\
        \n"
    );
}
