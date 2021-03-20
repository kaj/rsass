//! Tests auto-converted from "sass-spec/spec/css/custom_properties/nesting_characters.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".nesting-characters {\
            \n  --parens: (foo; (bar: baz;) bang!);\
            \n  --curly: {foo; {bar: baz;} bang!};\
            \n  --square: [foo; [bar: baz;] bang!];\
            \n  --multiple: [({{([])}})];\
            \n\
            \n  // Nested properties aren\'t supported in custom properties.\
            \n  --nested-props: {foo: bar;};\
            \n\
            \n  // A property that\'s ambiguous with a nested selector is interpreted as a\
            \n  // custom property.\
            \n  --ambiguous:foo {bar: baz;};\
            \n}\
            \n"
        )
        .unwrap(),
        ".nesting-characters {\
        \n  --parens: (foo; (bar: baz;) bang!);\
        \n  --curly: {foo; {bar: baz;} bang!};\
        \n  --square: [foo; [bar: baz;] bang!];\
        \n  --multiple: [({{([])}})];\
        \n  --nested-props: {foo: bar;};\
        \n  --ambiguous:foo {bar: baz;};\
        \n}\
        \n"
    );
}
