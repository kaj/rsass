//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_602.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_602")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("#foo.\\bar {\
             \n  color: red;\
             \n}\n\
             \n#foo.b\\ar {\
             \n  color: red;\
             \n}\n\
             \n#foo\\.bar {\
             \n  color: red;\
             \n}\n\
             \n#foo\\bar {\
             \n  color: red;\
             \n}\n\
             \n#fo\\o.bar {\
             \n  color: red;\
             \n}\n"),
        "@charset \"UTF-8\";\
         \n#foo.ºr {\
         \n  color: red;\
         \n}\
         \n#foo.b\\a r {\
         \n  color: red;\
         \n}\
         \n#foo\\.bar {\
         \n  color: red;\
         \n}\
         \n#fooºr {\
         \n  color: red;\
         \n}\
         \n#foo.bar {\
         \n  color: red;\
         \n}\n"
    );
}
