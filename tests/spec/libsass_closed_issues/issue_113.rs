//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_113.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "// Input\
            \nsection {\
            \n    $w: null, 10px;\
            \n    width: $w;\
            \n}"
        )
        .unwrap(),
        "section {\
        \n  width: 10px;\
        \n}\
        \n"
    );
}
