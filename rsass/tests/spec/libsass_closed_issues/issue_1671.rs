//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1671.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1671")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$foo: 5px;\
             \na {\
             \n    background: url(\'img.png\') no-repeat 6px 0 / #{$foo};\
             \n    background: url(\'img.png\') no-repeat 6px 1 / #{$foo};\
             \n    background: url(\'img.png\') no-repeat 6px 1px / #{$foo};\
             \n    background: url(\'img.png\') no-repeat 6px #{$foo} / 0;\
             \n    background: url(\'img.png\') no-repeat 6px #{$foo} / 1;\
             \n    background: url(\'img.png\') no-repeat 6px #{$foo} / 1px;\
             \n}\n"),
        "a {\
         \n  background: url(\"img.png\") no-repeat 6px 0/5px;\
         \n  background: url(\"img.png\") no-repeat 6px 1/5px;\
         \n  background: url(\"img.png\") no-repeat 6px 1px/5px;\
         \n  background: url(\"img.png\") no-repeat 6px 5px/0;\
         \n  background: url(\"img.png\") no-repeat 6px 5px/1;\
         \n  background: url(\"img.png\") no-repeat 6px 5px/1px;\
         \n}\n"
    );
}
