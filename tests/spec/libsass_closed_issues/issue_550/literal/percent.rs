//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_550/literal/percent.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "#foo {\
            \n  e: 10.0001%;\
            \n  f: 10.01%;\
            \n  g: -10.0001%;\
            \n  h: -10.01%; }\
            \n\
            \n#foo {\
            \n  e: 0.0001%;\
            \n  f: 0.01%;\
            \n  g: -0.0001%;\
            \n  h: -0.01%; }\
            \n\
            \n#foo {\
            \n  e: .0001%;\
            \n  f: .01%;\
            \n  g: -.0001%;\
            \n  h: -.01%; }\
            \n"
        )
        .unwrap(),
        "#foo {\
        \n  e: 10.0001%;\
        \n  f: 10.01%;\
        \n  g: -10.0001%;\
        \n  h: -10.01%;\
        \n}\
        \n#foo {\
        \n  e: 0.0001%;\
        \n  f: 0.01%;\
        \n  g: -0.0001%;\
        \n  h: -0.01%;\
        \n}\
        \n#foo {\
        \n  e: 0.0001%;\
        \n  f: 0.01%;\
        \n  g: -0.0001%;\
        \n  h: -0.01%;\
        \n}\
        \n"
    );
}
