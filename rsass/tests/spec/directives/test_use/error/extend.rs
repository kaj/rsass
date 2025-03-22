//! Tests auto-converted from "sass-spec/spec/directives/use/error/extend.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("extend")
        .mock_file("optional_and_mandatory/different_files/_mandatory.scss", "@use \"shared\";\n\ndownstream {@extend %-in-other};\n")
        .mock_file("optional_and_mandatory/different_files/_optional.scss", "@use \"shared\";\n\ndownstream {@extend %-in-other !optional};\n")
        .mock_file("optional_and_mandatory/different_files/_shared.scss", "%-in-other {x: y}\n")
        .mock_file("optional_and_mandatory/same_file/_other.scss", "%-in-other {x: y}\n")
        .mock_file("scope/diamond/_left.scss", "@use \"shared\";\n\nleft-extendee {@extend in-shared}\nleft-extender {@extend right-extendee}\n")
        .mock_file("scope/diamond/_right.scss", "@use \"shared\";\n\nright-extendee {@extend in-shared}\n")
        .mock_file("scope/diamond/_shared.scss", "in-shared {x: y}\n")
        .mock_file("scope/downstream/_other.scss", "in-other {@extend in-input}\n")
        .mock_file("scope/private/_other.scss", "%-in-other {x: y}\n\nin-other {@extend %-in-other}\n")
        .mock_file("scope/sibling/_left.scss", "left-extendee {in: left}\nleft-extender {@extend right-extendee}\n")
        .mock_file("scope/sibling/_right.scss", "right-extendee {in: right}\n")
}

mod optional_and_mandatory {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("optional_and_mandatory")
    }

    #[test]
    #[ignore] // wrong error
    fn different_files() {
        let runner = runner().with_cwd("different_files");
        assert_eq!(
            runner.err(
                "@use \"optional\";\
             \n@use \"mandatory\";\n"
            ),
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
        let runner = runner().with_cwd("same_file");
        assert_eq!(
            runner.err(
                "@use \"other\";\n\
             \nin-input {\
             \n  @extend %-in-other !optional;\
             \n  @extend %-in-other;\
             \n}\n"
            ),
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
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("scope")
    }

    #[test]
    #[ignore] // wrong error
    fn diamond() {
        let runner = runner().with_cwd("diamond");
        assert_eq!(
        runner.err(
            "// Even though left-extendee and right-extendee both end up in the style rule\
             \n// defined in _shared.scss, they aren\'t extended by the other file because those\
             \n// files don\'t use one another.\
             \n@use \"left\";\
             \n@use \"right\";\n"
        ),
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
        let runner = runner().with_cwd("downstream");
        assert_eq!(
            runner.err(
                "@use \"other\";\n\
             \nin-input {x: y}\n"
            ),
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
        let runner = runner().with_cwd("private");
        assert_eq!(
            runner.err(
                "@use \"other\";\n\
             \nin-input {@extend %-in-other}\n"
            ),
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
        let runner = runner().with_cwd("sibling");
        assert_eq!(
            runner.err(
                "@use \"left\";\
             \n@use \"right\";\n"
            ),
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
