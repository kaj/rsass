//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_820.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_820")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@charset \"UTF-8\";\
             \n/*!  Force output of above line by adding a unicode character. ♫ */\
             \nhtml, body {\
             \n  height: 100%; }\n"
        ),
        "@charset \"UTF-8\";\
         \n/*!  Force output of above line by adding a unicode character. ♫ */\
         \nhtml, body {\
         \n  height: 100%;\
         \n}\n"
    );
}
