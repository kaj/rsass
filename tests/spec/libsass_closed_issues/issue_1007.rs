//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1007.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "/* start */ foo /* foo */ baz /* bar */ {\
            \n    /* before */ margin /* X */: /* Y */ 0 /* */; /* after */\
            \n} /* end */"
        )
        .unwrap(),
        "/* start */\
        \nfoo baz {\
        \n  /* before */\
        \n  margin: 0;\
        \n  /* after */\
        \n}\
        \n/* end */\
        \n"
    );
}
