//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_550/literal/dimension.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("dimension")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("#foo {\
             \n  i: 10.0001px;\
             \n  j: 10.01px;\
             \n  k: -10.0001px;\
             \n  l: -10.01px; }\n\
             \n#foo {\
             \n  i: 0.0001px;\
             \n  j: 0.01px;\
             \n  k: -0.0001px;\
             \n  l: -0.01px; }\n\
             \n#foo {\
             \n  i: .0001px;\
             \n  j: .01px;\
             \n  k: -.0001px;\
             \n  l: -.01px; }\n"),
        "#foo {\
         \n  i: 10.0001px;\
         \n  j: 10.01px;\
         \n  k: -10.0001px;\
         \n  l: -10.01px;\
         \n}\
         \n#foo {\
         \n  i: 0.0001px;\
         \n  j: 0.01px;\
         \n  k: -0.0001px;\
         \n  l: -0.01px;\
         \n}\
         \n#foo {\
         \n  i: 0.0001px;\
         \n  j: 0.01px;\
         \n  k: -0.0001px;\
         \n  l: -0.01px;\
         \n}\n"
    );
}
