//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2112.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2112")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$color: #1caf9a;\r\
             \n\r\
             \nbody {\r\
             \n  test-01: change-color($color, $hue: -240);\r\
             \n  test-03: change-color($color, $hue: 120);\r\
             \n  test-02: change-color($color, $hue: 480);\r\
             \n}"),
        "body {\
         \n  test-01: #1caf1c;\
         \n  test-03: #1caf1c;\
         \n  test-02: #1caf1c;\
         \n}\n"
    );
}
