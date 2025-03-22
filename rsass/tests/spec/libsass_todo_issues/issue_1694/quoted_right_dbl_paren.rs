//! Tests auto-converted from "sass-spec/spec/libsass-todo-issues/issue_1694/quoted-right-dbl-paren.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("quoted-right-dbl-paren")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "test {\
             \n  filter: progid:DXImageTransform.Microsoft.Alpha(opacity=20\\));\
             \n}\n"
        ),
        "test {\
         \n  filter: progid:DXImageTransform.Microsoft.Alpha(opacity=20\\));\
         \n}\n"
    );
}
