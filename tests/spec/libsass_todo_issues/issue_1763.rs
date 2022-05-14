//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_1763.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("issue_1763")
        .mock_file("first.scss", "foo { bar: baz }\n")
        .mock_file("second.scss", "a { b: c }\n")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(
            "@import \"first.css\", \"second.css\" (max-width: 400px);\
             \n@import \"first.scss\", \"second.scss\" (max-width: 400px);\n"
        ),
        "@import \"first.css\";\
         \n@import \"second.css\" (max-width: 400px);\
         \n@import \"second.scss\" (max-width: 400px);\
         \nfoo {\
         \n  bar: baz;\
         \n}\n"
    );
}
