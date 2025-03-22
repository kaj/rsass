//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2140.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2140")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$red: red;\
             \n$foo: red;\n\
             \n:root {\
             \n  --red: #f00;\
             \n  // --red: $red;\
             \n  // --red: $foo;\
             \n}\n\
             \na { content: var(--red) }\
             \na { content: var(--$red) }\
             \na { content: var(--$foo) }\n\
             \nb { content: (- red) }\
             \nb { content: (- $red) }\
             \nb { content: (- $foo) }\n\
             \nc { content: (+- red) }\
             \nc { content: (+- $red) }\
             \nc { content: (+- $foo) }\n\
             \nd { content: (-red) }\
             \nd { content: (-$red) }\
             \nd { content: (-$foo) }\n\
             \ne { content: (+ red) }\
             \ne { content: (+ $red) }\
             \ne { content: (+ $foo) }"),
        ":root {\
         \n  --red: #f00;\
         \n}\
         \na {\
         \n  content: var(--red);\
         \n}\
         \na {\
         \n  content: var(-- red);\
         \n}\
         \na {\
         \n  content: var(-- red);\
         \n}\
         \nb {\
         \n  content: -red;\
         \n}\
         \nb {\
         \n  content: -red;\
         \n}\
         \nb {\
         \n  content: -red;\
         \n}\
         \nc {\
         \n  content: +-red;\
         \n}\
         \nc {\
         \n  content: +-red;\
         \n}\
         \nc {\
         \n  content: +-red;\
         \n}\
         \nd {\
         \n  content: -red;\
         \n}\
         \nd {\
         \n  content: -red;\
         \n}\
         \nd {\
         \n  content: -red;\
         \n}\
         \ne {\
         \n  content: +red;\
         \n}\
         \ne {\
         \n  content: +red;\
         \n}\
         \ne {\
         \n  content: +red;\
         \n}\n"
    );
}
