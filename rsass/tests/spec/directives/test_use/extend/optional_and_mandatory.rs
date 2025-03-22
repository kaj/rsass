//! Tests auto-converted from "sass-spec/spec/directives/use/extend/optional_and_mandatory.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("optional_and_mandatory")
        .mock_file(
            "different_files/mandatory_first/_mandatory.scss",
            "@use \"shared\";\n\ndownstream {@extend in-other};\n",
        )
        .mock_file(
            "different_files/mandatory_first/_optional.scss",
            "@use \"shared\";\n\ndownstream {@extend in-other !optional};\n",
        )
        .mock_file(
            "different_files/mandatory_first/_shared.scss",
            "in-other {x: y}\n",
        )
        .mock_file(
            "different_files/optional_first/_mandatory.scss",
            "@use \"shared\";\n\ndownstream {@extend in-other};\n",
        )
        .mock_file(
            "different_files/optional_first/_optional.scss",
            "@use \"shared\";\n\ndownstream {@extend in-other !optional};\n",
        )
        .mock_file(
            "different_files/optional_first/_shared.scss",
            "in-other {x: y}\n",
        )
        .mock_file("same_file/_other.scss", "in-other {x: y}\n")
}

mod different_files {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("different_files")
    }

    #[test]
    #[ignore] // unexepected error
    fn mandatory_first() {
        let runner = runner().with_cwd("mandatory_first");
        assert_eq!(
            runner.ok("@use \"mandatory\";\
             \n@use \"optional\";\n"),
            "in-other, downstream {\
         \n  x: y;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn optional_first() {
        let runner = runner().with_cwd("optional_first");
        assert_eq!(
            runner.ok("@use \"optional\";\
             \n@use \"mandatory\";\n"),
            "in-other, downstream {\
         \n  x: y;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn same_file() {
    let runner = runner().with_cwd("same_file");
    assert_eq!(
        runner.ok("@use \"other\";\n\
             \nin-input {\
             \n  @extend in-other !optional;\
             \n  @extend in-other;\
             \n}\n"),
        "in-other, in-input {\
         \n  x: y;\
         \n}\n"
    );
}
