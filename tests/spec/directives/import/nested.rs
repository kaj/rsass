//! Tests auto-converted from "sass-spec/spec/directives/import/nested.hrx"

mod at_rule {
    #[test]
    #[ignore] // unexepected error
    fn childless() {
        assert_eq!(
            crate::rsass(
                "a {@import \"other\"}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  @b c;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn declaration_child() {
        assert_eq!(
            crate::rsass(
                "a {@import \"other\"}\
            \n"
            )
            .unwrap(),
            "@b {\
        \n  a {\
        \n    c: d;\
        \n  }\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn keyframes() {
        assert_eq!(
            crate::rsass(
                "a {@import \"other\"}\
            \n"
            )
            .unwrap(),
            "@keyframes b {\
        \n  0% {\
        \n    c: d;\
        \n  }\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn rule_child() {
        assert_eq!(
            crate::rsass(
                "a {@import \"other\"}\
            \n"
            )
            .unwrap(),
            "@b {\
        \n  a c {\
        \n    d: e;\
        \n  }\
        \n}\
        \n"
        );
    }
}
mod scope {
    #[test]
    #[ignore] // wrong result
    fn function() {
        assert_eq!(
        crate::rsass(
            ".parent {\
            \n  // This should be visible to the imported stylesheet. There\'s not really a\
            \n  // good reason for this, but it\'s the historical behavior so whatever.\
            \n  @function local() {\
            \n    @return value;\
            \n  }\
            \n\
            \n  @import \'other\';\
            \n}\
            \n"
        )
        .unwrap(),
        ".parent x {\
        \n  function: value;\
        \n}\
        \n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn mixin() {
        assert_eq!(
        crate::rsass(
            ".parent {\
            \n  // This should be visible to the imported stylesheet. There\'s not really a\
            \n  // good reason for this, but it\'s the historical behavior so whatever.\
            \n  @mixin local {\
            \n    x {y: z}\
            \n  }\
            \n\
            \n  @import \'other\';\
            \n}\
            \n"
        )
        .unwrap(),
        ".parent x {\
        \n  y: z;\
        \n}\
        \n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn variable() {
        assert_eq!(
        crate::rsass(
            ".parent {\
            \n  // This should be visible to the imported stylesheet. There\'s not really a\
            \n  // good reason for this, but it\'s the historical behavior so whatever.\
            \n  $var: value;\
            \n  @import \'other\';\
            \n}\
            \n"
        )
        .unwrap(),
        ".parent x {\
        \n  var: value;\
        \n}\
        \n"
    );
    }
}
