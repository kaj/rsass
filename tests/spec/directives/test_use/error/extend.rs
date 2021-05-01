//! Tests auto-converted from "sass-spec/spec/directives/use/error/extend.hrx"

mod optional_and_mandatory {
    #[test]
    #[ignore] // wrong error
    fn different_files() {
        assert_eq!(
            crate::rsass(
                "@use \"optional\";\
             \n@use \"mandatory\";\
             \n"
            )
            .unwrap_err(),
            "Error: The target selector was not found.\
         \nUse \"@extend %-in-other !optional\" to avoid this error.\
         \n  ,\
         \n3 | downstream {@extend %-in-other};\
         \n  |             ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  _mandatory.scss 3:13  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn same_file() {
        assert_eq!(
            crate::rsass(
                "@use \"other\";\
             \n\
             \nin-input {\
             \n  @extend %-in-other !optional;\
             \n  @extend %-in-other;\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: The target selector was not found.\
         \nUse \"@extend %-in-other !optional\" to avoid this error.\
         \n  ,\
         \n5 |   @extend %-in-other;\
         \n  |   ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 5:3  root stylesheet",
        );
    }
}
mod scope {
    #[test]
    #[ignore] // wrong error
    fn diamond() {
        assert_eq!(
        crate::rsass(
            "// Even though left-extendee and right-extendee both end up in the style rule\
             \n// defined in _shared.scss, they aren\'t extended by the other file because those\
             \n// files don\'t use one another.\
             \n@use \"left\";\
             \n@use \"right\";\
             \n"
        ).unwrap_err(),
        "Error: The target selector was not found.\
         \nUse \"@extend right-extendee !optional\" to avoid this error.\
         \n  ,\
         \n4 | left-extender {@extend right-extendee}\
         \n  |                ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  _left.scss 4:16  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn downstream() {
        assert_eq!(
            crate::rsass(
                "@use \"other\";\
             \n\
             \nin-input {x: y}\
             \n"
            )
            .unwrap_err(),
            "Error: The target selector was not found.\
         \nUse \"@extend in-input !optional\" to avoid this error.\
         \n  ,\
         \n1 | in-other {@extend in-input}\
         \n  |           ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  _other.scss 1:11  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn private() {
        assert_eq!(
            crate::rsass(
                "@use \"other\";\
             \n\
             \nin-input {@extend %-in-other}\
             \n"
            )
            .unwrap_err(),
            "Error: The target selector was not found.\
         \nUse \"@extend %-in-other !optional\" to avoid this error.\
         \n  ,\
         \n3 | in-input {@extend %-in-other}\
         \n  |           ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:11  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn sibling() {
        assert_eq!(
            crate::rsass(
                "@use \"left\";\
             \n@use \"right\";\
             \n"
            )
            .unwrap_err(),
            "Error: The target selector was not found.\
         \nUse \"@extend right-extendee !optional\" to avoid this error.\
         \n  ,\
         \n2 | left-extender {@extend right-extendee}\
         \n  |                ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  _left.scss 2:16  root stylesheet",
        );
    }
}
