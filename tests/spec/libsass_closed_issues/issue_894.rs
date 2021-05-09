//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_894.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "a {/**/}\
             \nb {content: \'something so I have a non-empty expected output\'}"
        ),
        "a {\
         \n  /**/\
         \n}\
         \nb {\
         \n  content: \"something so I have a non-empty expected output\";\
         \n}\n"
    );
}
