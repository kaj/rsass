//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1294.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1294")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("/*------------------------------------*\\\
             \n  #BUTTONS\
             \n\\*------------------------------------*/\n\
             \nfoo {\
             \n  display: inline-block; /* [1] */\
             \n}\n"),
        "/*------------------------------------*\\\
         \n  #BUTTONS\
         \n\\*------------------------------------*/\
         \nfoo {\
         \n  display: inline-block; /* [1] */\
         \n}\n"
    );
}
