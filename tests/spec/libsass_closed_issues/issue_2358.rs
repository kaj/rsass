//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2358.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".outer {\r\
            \n  @at-root .root {\r\
            \n    .inner {\r\
            \n      .element {\r\
            \n        --modifier#{&}--another-modifier {\r\
            \n          content: \"#{&}\";\r\
            \n        }\r\
            \n        &--modifier#{&}--another-modifier {\r\
            \n          content: \"#{&}\";\r\
            \n        }\r\
            \n      }\r\
            \n    }\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \n@at-root .block {\r\
            \n  &__element {\r\
            \n    #{&} {\r\
            \n      content: \"#{&}\";\r\
            \n    }\r\
            \n    &--modifier {\r\
            \n      content: \"#{&}\";\r\
            \n    }\r\
            \n    --modifier#{&}--another-modifier {\r\
            \n      content: \"#{&}\";\r\
            \n    }\r\
            \n    &--modifier#{&}--another-modifier {\r\
            \n      content: \"#{&}\";\r\
            \n    }\r\
            \n  }\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        ".root .inner .element --modifier.root .inner .element--another-modifier {\
        \n  content: \".root .inner .element --modifier.root .inner .element--another-modifier\";\
        \n}\
        \n.root .inner .element--modifier.root .inner .element--another-modifier {\
        \n  content: \".root .inner .element--modifier.root .inner .element--another-modifier\";\
        \n}\
        \n.block__element .block__element {\
        \n  content: \".block__element .block__element\";\
        \n}\
        \n.block__element--modifier {\
        \n  content: \".block__element--modifier\";\
        \n}\
        \n.block__element --modifier.block__element--another-modifier {\
        \n  content: \".block__element --modifier.block__element--another-modifier\";\
        \n}\
        \n.block__element--modifier.block__element--another-modifier {\
        \n  content: \".block__element--modifier.block__element--another-modifier\";\
        \n}\
        \n"
    );
}
