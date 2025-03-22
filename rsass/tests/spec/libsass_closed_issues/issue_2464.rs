//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2464.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2464")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(":host(:not(.foo)) {\r\
             \n    left: 0;\r\
             \n}\r\
             \n\r\
             \nfoobar {\r\
             \n    :host(:not(.foo)) {\r\
             \n        left: 0;\r\
             \n    }\r\
             \n}"),
        ":host(:not(.foo)) {\
         \n  left: 0;\
         \n}\
         \nfoobar :host(:not(.foo)) {\
         \n  left: 0;\
         \n}\n"
    );
}
