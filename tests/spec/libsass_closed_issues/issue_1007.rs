//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1007.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1007")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("/* start */ foo /* foo */ baz /* bar */ {\
             \n    /* before */ margin /* X */: /* Y */ 0 /* */; /* after */\
             \n} /* end */"),
        "/* start */\
         \nfoo baz {\
         \n  /* before */\
         \n  margin: 0;\
         \n  /* after */\
         \n}\
         \n/* end */\n"
    );
}
