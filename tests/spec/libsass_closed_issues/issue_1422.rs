//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1422.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  /*foo*/foo/*foo*/: /*foo*/bar/*foo*/;\
            \n  /*foo*/ foo /*foo*/ : /*foo*/ bar /*foo*/;\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  /*foo*/\
        \n  foo/*foo*/: bar;\
        \n  /*foo*/\
        \n  foo: bar;\
        \n}\
        \n"
    );
}
