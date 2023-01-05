//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1259.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1259")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@mixin dummy($a, $b, $c, $d, $e: true) {\
             \n  content: $a $b $c $d $e;\
             \n}\n\
             \n.foo {\
             \n  @include dummy( (\'a\', \'b\', \'c\', \'e\')..., $e: false );\
             \n}"
        ),
        ".foo {\
         \n  content: \"a\" \"b\" \"c\" \"e\" false;\
         \n}\n"
    );
}
