//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_289.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@import url(http://fonts.googleapis.com/css?family=Titillium+Web:400,300,200,600);"
        ),
        "@import url(http://fonts.googleapis.com/css?family=Titillium+Web:400,300,200,600);\n"
    );
}
