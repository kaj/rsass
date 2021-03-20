//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1259.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin dummy($a, $b, $c, $d, $e: true) {\
            \n  content: $a $b $c $d $e;\
            \n}\
            \n\
            \n.foo {\
            \n  @include dummy( (\'a\', \'b\', \'c\', \'e\')..., $e: false );\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  content: \"a\" \"b\" \"c\" \"e\" false;\
        \n}\
        \n"
    );
}
