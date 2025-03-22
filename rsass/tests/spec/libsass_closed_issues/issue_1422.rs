//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1422.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1422")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  /*foo*/foo/*foo*/: /*foo*/bar/*foo*/;\
             \n  /*foo*/ foo /*foo*/ : /*foo*/ bar /*foo*/;\
             \n}\n"),
        ".foo {\
         \n  /*foo*/\
         \n  foo/*foo*/: bar;\
         \n  /*foo*/\
         \n  foo: bar;\
         \n}\n"
    );
}
