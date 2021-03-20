//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_733.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function getter() {\
            \n  @return 42px;\
            \n}\
            \n\
            \ntest {\
            \n  content: getter()-1;\
            \n  content: getter()- 1;\
            \n  content: getter() -1;\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  content: 41px;\
        \n  content: 41px;\
        \n  content: 42px -1;\
        \n}\
        \n"
    );
}
