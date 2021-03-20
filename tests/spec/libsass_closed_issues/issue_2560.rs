//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2560.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$x: 10px / 5px;\r\
            \n\r\
            \ntest {\r\
            \n    font-size: $x;\r\
            \n    font-size: #{$x};\r\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  font-size: 2;\
        \n  font-size: 2;\
        \n}\
        \n"
    );
}
