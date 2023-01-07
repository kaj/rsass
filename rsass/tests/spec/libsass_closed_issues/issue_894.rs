//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_894.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_894")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(
            "a {/**/}\
             \nb {content: \'something so I have a non-empty expected output\'}"
        ),
        "a { /**/ }\
         \nb {\
         \n  content: \"something so I have a non-empty expected output\";\
         \n}\n"
    );
}
