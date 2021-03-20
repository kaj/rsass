//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2376.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".test {\r\
            \n\tbackground:url(//img12.360buyimg.com/..);\r\
            \n\t.a{\r\
            \n\t\theight: 100px;\r\
            \n\t}\r\
            \n}"
        )
        .unwrap(),
        ".test {\
        \n  background: url(//img12.360buyimg.com/..);\
        \n}\
        \n.test .a {\
        \n  height: 100px;\
        \n}\
        \n"
    );
}
