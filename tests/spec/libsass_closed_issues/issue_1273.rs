//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1273.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "test {\
            \n  src: url(test.eot#{if(true, \'?#{42}\', \'\')});\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  src: url(test.eot?42);\
        \n}\
        \n"
    );
}
