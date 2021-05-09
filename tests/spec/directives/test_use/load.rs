//! Tests auto-converted from "sass-spec/spec/directives/use/load.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("explicit_extension/sass/other.css", "a {syntax: css}\n")
        .mock_file(
            "explicit_extension/sass/other.sass",
            "a\n  syntax: sass\n",
        )
        .mock_file("explicit_extension/sass/other.scss", "a {syntax: scss}\n")
        .mock_file("explicit_extension/scss/other.css", "a {syntax: css}\n")
        .mock_file(
            "explicit_extension/scss/other.sass",
            "a\n  syntax: sass\n",
        )
        .mock_file("explicit_extension/scss/other.scss", "a {syntax: scss}\n")
        .mock_file(
            "index/dir_dot_foo/dir.foo/index.scss",
            ".foo {\n  a: b;\n}\n",
        )
        .mock_file("index/partial/dir/_index.scss", ".foo {\n  a: b;\n}\n")
        .mock_file("index/sass/dir/index.sass", ".foo \n  a: b\n")
        .mock_file("index/scss/dir/index.scss", ".foo {\n  a: b;\n}\n")
        .mock_file(
            "precedence/ignores_import_only/other.import.scss",
            "a {import-only: true}\n",
        )
        .mock_file(
            "precedence/ignores_import_only/other.scss",
            "a {import-only: false}\n",
        )
        .mock_file(
            "precedence/normal_before_index/dir.scss",
            "a {index: false}\n",
        )
        .mock_file(
            "precedence/normal_before_index/dir/index.scss",
            "a {index: true}\n",
        )
        .mock_file(
            "precedence/sass_before_css/other.css",
            "a {syntax: css}\n",
        )
        .mock_file(
            "precedence/sass_before_css/other.sass",
            "a\n  syntax: sass\n",
        )
        .mock_file(
            "precedence/scss_before_css/other.css",
            "a {syntax: css}\n",
        )
        .mock_file(
            "precedence/scss_before_css/other.scss",
            "a {syntax: scss}\n",
        )
}

mod explicit_extension {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // unexepected error
    fn sass() {
        assert_eq!(
            runner().ok("@use \"other.sass\"\n"),
            "a {\
         \n  syntax: sass;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn scss() {
        assert_eq!(
            runner().ok("@use \"other.scss\"\n"),
            "a {\
         \n  syntax: scss;\
         \n}\n"
        );
    }
}
mod index {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // unexepected error
    fn dir_dot_foo() {
        assert_eq!(
            runner().ok("@use \"dir.foo\";\n"),
            ".foo {\
         \n  a: b;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
            runner().ok("@use \"dir\";\n"),
            ".foo {\
         \n  a: b;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn sass() {
        assert_eq!(
            runner().ok("@use \"dir\";\n"),
            ".foo {\
         \n  a: b;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn scss() {
        assert_eq!(
            runner().ok("@use \"dir\";\n"),
            ".foo {\
         \n  a: b;\
         \n}\n"
        );
    }
}
mod precedence {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // unexepected error
    fn ignores_import_only() {
        assert_eq!(
            runner().ok("@use \"other\";\n"),
            "a {\
         \n  import-only: false;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn normal_before_index() {
        assert_eq!(
            runner().ok("@use \"dir\";\n"),
            "a {\
         \n  index: false;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn sass_before_css() {
        assert_eq!(
            runner().ok("@use \"other\";\n"),
            "a {\
         \n  syntax: sass;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn scss_before_css() {
        assert_eq!(
            runner().ok("@use \"other\";\n"),
            "a {\
         \n  syntax: scss;\
         \n}\n"
        );
    }
}
