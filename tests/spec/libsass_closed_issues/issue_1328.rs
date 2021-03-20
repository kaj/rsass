//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1328.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "#{bar},\
            \n[foo=\"#{bar}\"],\
            \n[foo=\"#{bar}\"] {\
            \n    content: \"foo\";\
            \n}\
            \n"
        )
        .unwrap(),
        "bar,\
        \n[foo=bar],\
        \n[foo=bar] {\
        \n  content: \"foo\";\
        \n}\
        \n"
    );
}
