//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_289.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@import url(http://fonts.googleapis.com/css?family=Titillium+Web:400,300,200,600);"
        )
        .unwrap(),
        "@import url(http://fonts.googleapis.com/css?family=Titillium+Web:400,300,200,600);\
        \n"
    );
}
