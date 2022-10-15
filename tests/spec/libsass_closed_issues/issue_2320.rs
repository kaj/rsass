//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2320.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2320")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$char-f: \'\\66\';\r\
             \n$char-g: \'\\67\';\r\
             \n\r\
             \n.test-1 {\r\
             \n  content: \'#{$char-f}\\feff\';\r\
             \n}\r\
             \n\r\
             \n.test-2 {\r\
             \n  content: \'#{$char-g}\\feff\';\r\
             \n}\r\
             \n\r\
             \n// this is broken\r\
             \n.test-3 {\r\
             \n  content: \'\\feff#{$char-f}\';\r\
             \n}\r\
             \n\r\
             \n.test-4 {\r\
             \n  content: \'\\feff#{$char-g}\';\r\
             \n}"),
        "@charset \"UTF-8\";\
         \n.test-1 {\
         \n  content: \"f\u{feff}\";\
         \n}\
         \n.test-2 {\
         \n  content: \"g\u{feff}\";\
         \n}\
         \n.test-3 {\
         \n  content: \"\u{feff}f\";\
         \n}\
         \n.test-4 {\
         \n  content: \"\u{feff}g\";\
         \n}\n"
    );
}
