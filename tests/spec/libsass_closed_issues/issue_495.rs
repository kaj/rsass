//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_495.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_495")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "/* Testing to make sure that a trailing comma doesn\'t break the tests */\
             \n$map: (\
             \n  hello: world,\
             \n);\n"
        ),
        "/* Testing to make sure that a trailing comma doesn\'t break the tests */\n"
    );
}
