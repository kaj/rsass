//! Tests auto-converted from "sass-spec/spec/directives/import/load.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("load")
        .mock_file("explicit_extension/sass/other.css", "a {syntax: css}\n")
        .mock_file("explicit_extension/sass/other.sass", "a\n  syntax: sass\n")
        .mock_file("explicit_extension/sass/other.scss", "a {syntax: scss}\n")
        .mock_file("explicit_extension/scss/other.css", "a {syntax: css}\n")
        .mock_file("explicit_extension/scss/other.sass", "a\n  syntax: sass\n")
        .mock_file("explicit_extension/scss/other.scss", "a {syntax: scss}\n")
        .mock_file("index/dir_dot_foo/dir.foo/index.scss", ".foo {\n  a: b;\n}\n")
        .mock_file("index/dir_dot_scss/dir.scss/index.scss", ".foo {\n  a: b;\n}\n")
        .mock_file("index/partial/dir/_index.scss", ".foo {\n  a: b;\n}\n")
        .mock_file("index/sass/dir/index.sass", ".foo \n  a: b\n")
        .mock_file("index/scss/dir/index.scss", ".foo {\n  a: b;\n}\n")
        .mock_file("precedence/import_only/before_index/other.import.scss", "a {import-only: true}\n")
        .mock_file("precedence/import_only/before_index/other/index.scss", "a {import-only: false}\n")
        .mock_file("precedence/import_only/explicit_extension/other.import.scss", "a {import-only: true}\n")
        .mock_file("precedence/import_only/explicit_extension/other.scss", "a {import-only: false}\n")
        .mock_file("precedence/import_only/implicit_extension/other.import.sass", "a\n  import-only: true\n")
        .mock_file("precedence/import_only/implicit_extension/other.scss", "a {import-only: false}\n")
        .mock_file("precedence/import_only/index/other/index.import.scss", "a {import-only: true}\n")
        .mock_file("precedence/import_only/index/other/index.scss", "a {import-only: false}\n")
        .mock_file("precedence/import_only/index_after_normal/other.scss", "a {import-only: false}\n")
        .mock_file("precedence/import_only/index_after_normal/other/index.import.scss", "a {import-only: true}\n")
        .mock_file("precedence/import_only/normal_before_partial/_other.scss", "a {import-only: false}\n")
        .mock_file("precedence/import_only/normal_before_partial/other.import.scss", "a {import-only: true}\n")
        .mock_file("precedence/import_only/partial_before_normal/_other.import.scss", "a {import-only: true}\n")
        .mock_file("precedence/import_only/partial_before_normal/other.scss", "a {import-only: false}\n")
        .mock_file("precedence/normal_before_index/dir.scss", "a {index: false}\n")
        .mock_file("precedence/normal_before_index/dir/index.scss", "a {index: true}\n")
        .mock_file("precedence/sass_before_css/other.css", "a {syntax: css}\n")
        .mock_file("precedence/sass_before_css/other.sass", "a\n  syntax: sass\n")
        .mock_file("precedence/scss_before_css/other.css", "a {syntax: css}\n")
        .mock_file("precedence/scss_before_css/other.scss", "a {syntax: scss}\n")
}

mod explicit_extension {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("explicit_extension")
    }

    #[test]
    #[ignore] // unexepected error
    fn sass() {
        let runner = runner().with_cwd("sass");
        assert_eq!(
            runner.ok("@import \"other.sass\"\n"),
            "a {\
         \n  syntax: sass;\
         \n}\n"
        );
    }
    #[test]
    fn scss() {
        let runner = runner().with_cwd("scss");
        assert_eq!(
            runner.ok("@import \"other.scss\"\n"),
            "a {\
         \n  syntax: scss;\
         \n}\n"
        );
    }
}
mod index {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("index")
    }

    #[test]
    fn dir_dot_foo() {
        let runner = runner().with_cwd("dir_dot_foo");
        assert_eq!(
            runner.ok("@import \"dir.foo\";\n"),
            ".foo {\
         \n  a: b;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // missing error
    fn dir_dot_scss() {
        let runner = runner().with_cwd("dir_dot_scss");
        assert_eq!(
            runner.err("@import \"dir.scss\";\n"),
            "Error: Can\'t find stylesheet to import.\
         \n  ,\
         \n1 | @import \"dir.scss\";\
         \n  |         ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    fn partial() {
        let runner = runner().with_cwd("partial");
        assert_eq!(
            runner.ok("@import \"dir\";\n"),
            ".foo {\
         \n  a: b;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn sass() {
        let runner = runner().with_cwd("sass");
        assert_eq!(
            runner.ok("@import \"dir\";\n"),
            ".foo {\
         \n  a: b;\
         \n}\n"
        );
    }
    #[test]
    fn scss() {
        let runner = runner().with_cwd("scss");
        assert_eq!(
            runner.ok("@import \"dir\";\n"),
            ".foo {\
         \n  a: b;\
         \n}\n"
        );
    }
}
mod precedence {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("precedence")
    }

    mod import_only {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("import_only")
        }

        #[test]
        fn before_index() {
            let runner = runner().with_cwd("before_index");
            assert_eq!(
        runner.ok(
            "// A non-index import-only file takes precedence over an index file.\
             \n@import \"other\";\n"
        ),
        "a {\
         \n  import-only: true;\
         \n}\n"
    );
        }
        #[test]
        fn explicit_extension() {
            let runner = runner().with_cwd("explicit_extension");
            assert_eq!(
                runner.ok("@import \"other\";\n"),
                "a {\
         \n  import-only: true;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn implicit_extension() {
            let runner = runner().with_cwd("implicit_extension");
            assert_eq!(
        runner.ok(
            "// The extension of the import-only file doesn\'t need to match the extension of\
             \n// the use-only file.\
             \n@import \"other\";\n"
        ),
        "a {\
         \n  import-only: true;\
         \n}\n"
    );
        }
        #[test]
        fn index() {
            let runner = runner().with_cwd("index");
            assert_eq!(
        runner.ok(
            "// A import-only index file takes precedence over a normal index file.\
             \n@import \"other\";\n"
        ),
        "a {\
         \n  import-only: true;\
         \n}\n"
    );
        }
        #[test]
        fn index_after_normal() {
            let runner = runner().with_cwd("index_after_normal");
            assert_eq!(
        runner.ok(
            "// Index files, even import-only ones, always come after non-index files.\
             \n@import \"other\";\n"
        ),
        "a {\
         \n  import-only: false;\
         \n}\n"
    );
        }
        #[test]
        fn normal_before_partial() {
            let runner = runner().with_cwd("normal_before_partial");
            assert_eq!(
        runner.ok(
            "// A normal import-only file takes precedence over a non-import-only partial.\
             \n@import \"other\";\n"
        ),
        "a {\
         \n  import-only: true;\
         \n}\n"
    );
        }
        #[test]
        fn partial_before_normal() {
            let runner = runner().with_cwd("partial_before_normal");
            assert_eq!(
        runner.ok(
            "// An import-only partial takes precedence over a normal non-import-only file.\
             \n@import \"other\";\n"
        ),
        "a {\
         \n  import-only: true;\
         \n}\n"
    );
        }
    }
    #[test]
    fn normal_before_index() {
        let runner = runner().with_cwd("normal_before_index");
        assert_eq!(
            runner.ok("@import \"dir\";\n"),
            "a {\
         \n  index: false;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn sass_before_css() {
        let runner = runner().with_cwd("sass_before_css");
        assert_eq!(
            runner.ok("@import \"other\";\n"),
            "a {\
         \n  syntax: sass;\
         \n}\n"
        );
    }
    #[test]
    fn scss_before_css() {
        let runner = runner().with_cwd("scss_before_css");
        assert_eq!(
            runner.ok("@import \"other\";\n"),
            "a {\
         \n  syntax: scss;\
         \n}\n"
        );
    }
}
