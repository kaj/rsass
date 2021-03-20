//! Tests auto-converted from "sass-spec/spec/libsass/selectors/interpolation.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  content: #{&};\
            \n}\
            \n\
            \n.bar a {\
            \n  content: #{&};\
            \n}\
            \n\
            \n.bar,\
            \n.baz {\
            \n  content: #{&};\
            \n}\
            \n\
            \n.qux {\
            \n  &.waldo {\
            \n    .where & {\
            \n      .final {\
            \n        content: #{&};\
            \n      }\
            \n    }\
            \n  }\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  content: .foo;\
        \n}\
        \n.bar a {\
        \n  content: .bar a;\
        \n}\
        \n.bar,\
        \n.baz {\
        \n  content: .bar, .baz;\
        \n}\
        \n.where .qux.waldo .final {\
        \n  content: .where .qux.waldo .final;\
        \n}\
        \n"
    );
}
