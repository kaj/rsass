//! Tests auto-converted from "sass-spec/spec/libsass/selectors/variables/nested/bare.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo a,\
            \n.bar p {\
            \n\
            \n  .baz {\
            \n    $bar: &;\
            \n    content: $bar;\
            \n  }\
            \n\
            \n}"
        )
        .unwrap(),
        ".foo a .baz,\
        \n.bar p .baz {\
        \n  content: .foo a .baz, .bar p .baz;\
        \n}\
        \n"
    );
}
