//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1947.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".a-#{quote(\'\' + b)} {\
             \n  c: d;\
             \n}\n\
             \n.a-#{\'\' + b} {\
             \n  c: d;\
             \n}"),
        ".a-b {\
         \n  c: d;\
         \n}\
         \n.a-b {\
         \n  c: d;\
         \n}\n"
    );
}
