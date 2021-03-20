//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1947.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".a-#{quote(\'\' + b)} {\
            \n  c: d;\
            \n}\
            \n\
            \n.a-#{\'\' + b} {\
            \n  c: d;\
            \n}"
        )
        .unwrap(),
        ".a-b {\
        \n  c: d;\
        \n}\
        \n.a-b {\
        \n  c: d;\
        \n}\
        \n"
    );
}
