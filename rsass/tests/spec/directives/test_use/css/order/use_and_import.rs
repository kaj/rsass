//! Tests auto-converted from "sass-spec/spec/directives/use/css/order/use_and_import.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("use_and_import")
        .mock_file("comments_and_imports/midstream.scss", "/* midstream comment before use */\n@use \"upstream\";\n\n/* midstream comment before first import */\n@import \"midstream1.css\";\n\n/* midstream comment before second import */\n@import \"midstream2.css\";\n\n/* midstream comment after imports */\n\na {file: midstream}\n")
        .mock_file("comments_and_imports/upstream.scss", "/* upstream comment before import */\n@import \"upstream.css\";\n\n/* upstream comment after import */\n")
        .mock_file("import_into_use/css_import_above_rule/_imported.scss", "@use \"used\";\n\n@import \"imported.css\";\n\na {file: imported}\n")
        .mock_file("import_into_use/css_import_above_rule/_used.scss", "@import \"used.css\";\n\na {file: used}\n")
        .mock_file("import_into_use/css_import_below_rule/_imported.scss", "@use \"used\";\n\na {file: imported}\n\n@import \"imported.css\";\n")
        .mock_file("import_into_use/css_import_below_rule/_used.scss", "a {file: used}\n\n@import \"used.css\";\n")
        .mock_file("import_into_use/sass_import_below_css_import/_imported.scss", "@use \"used\";\n\n@import \"imported.css\";\n")
        .mock_file("import_into_use/sass_import_below_css_import/_used.scss", "@import \"used.css\";\n")
        .mock_file("use_into_import/css_import_above_rule/_imported.scss", "@import \"imported.css\";\n\na {file: imported}\n")
        .mock_file("use_into_import/css_import_above_rule/_used.scss", "@import \"imported\";\n\n@import \"used.css\";\n\na {file: used}\n")
        .mock_file("use_into_import/css_import_below_rule/_imported.scss", "a {file: imported}\n\n@import \"imported.css\";\n")
        .mock_file("use_into_import/css_import_below_rule/_used.scss", "@import \"imported\";\n\na {file: used}\n\n@import \"used.css\";\n")
        .mock_file("use_into_import/sass_import_below_css_import/_imported.scss", "@import \"imported.css\";\n")
        .mock_file("use_into_import/sass_import_below_css_import/_used.scss", "@import \"used.css\";\n\n@import \"imported\";\n")
        .mock_file("use_into_use/import_above_rule/_midstream.scss", "@use \"upstream\";\n\n@import \"midstream.css\";\n\na {file: midstream}\n")
        .mock_file("use_into_use/import_above_rule/_upstream.scss", "@import \"upstream.css\";\n\na {file: upstream}\n")
        .mock_file("use_into_use/import_below_rule/_midstream.scss", "@use \"upstream\";\n\na {file: midstream}\n\n@import \"midstream.css\";\n")
        .mock_file("use_into_use/import_below_rule/_upstream.scss", "a {file: upstream}\n\n@import \"upstream.css\";\n")
}

#[test]
#[ignore] // wrong result
fn comments_and_imports() {
    let runner = runner().with_cwd("comments_and_imports");
    assert_eq!(
        runner.ok("/* input comment before use */\
             \n@use \"midstream\";\n\
             \n/* input comment before import */\
             \n@import \"input.css\";\n\
             \n/* input comment after import */\n"),
        "/* input comment before use */\
         \n/* midstream comment before use */\
         \n/* upstream comment before import */\
         \n@import \"upstream.css\";\
         \n/* midstream comment before first import */\
         \n@import \"midstream1.css\";\
         \n/* midstream comment before second import */\
         \n@import \"midstream2.css\";\
         \n/* input comment before import */\
         \n@import \"input.css\";\
         \n/* upstream comment after import */\
         \n/* midstream comment after imports */\
         \na {\
         \n  file: midstream;\
         \n}\
         \n/* input comment after import */\n"
    );
}
mod import_into_use {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("import_into_use")
    }

    #[test]
    fn css_import_above_rule() {
        let runner = runner().with_cwd("css_import_above_rule");
        assert_eq!(
            runner.ok("@import \"imported\";\n\
             \n@import \"input.css\";\n"),
            "@import \"used.css\";\
         \n@import \"imported.css\";\
         \n@import \"input.css\";\
         \na {\
         \n  file: used;\
         \n}\
         \na {\
         \n  file: imported;\
         \n}\n"
        );
    }
    #[test]
    fn css_import_below_rule() {
        let runner = runner().with_cwd("css_import_below_rule");
        assert_eq!(
            runner.ok("@import \"imported\";\n\
             \n@import \"input.css\";\n"),
            "@import \"used.css\";\
         \n@import \"imported.css\";\
         \n@import \"input.css\";\
         \na {\
         \n  file: used;\
         \n}\
         \na {\
         \n  file: imported;\
         \n}\n"
        );
    }
    #[test]
    fn sass_import_below_css_import() {
        let runner = runner().with_cwd("sass_import_below_css_import");
        assert_eq!(
            runner.ok("@import \"input.css\";\n\
             \n@import \"imported\";\n"),
            "@import \"input.css\";\
         \n@import \"used.css\";\
         \n@import \"imported.css\";\n"
        );
    }
}
mod use_into_import {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("use_into_import")
    }

    #[test]
    fn css_import_above_rule() {
        let runner = runner().with_cwd("css_import_above_rule");
        assert_eq!(
            runner.ok("@use \"used\";\n\
             \n@import \"input.css\";\n"),
            "@import \"imported.css\";\
         \n@import \"used.css\";\
         \n@import \"input.css\";\
         \na {\
         \n  file: imported;\
         \n}\
         \na {\
         \n  file: used;\
         \n}\n"
        );
    }
    #[test]
    fn css_import_below_rule() {
        let runner = runner().with_cwd("css_import_below_rule");
        assert_eq!(
            runner.ok("@use \"used\";\n\
             \n@import \"input.css\";\n"),
            "@import \"imported.css\";\
         \n@import \"used.css\";\
         \n@import \"input.css\";\
         \na {\
         \n  file: imported;\
         \n}\
         \na {\
         \n  file: used;\
         \n}\n"
        );
    }
    #[test]
    fn sass_import_below_css_import() {
        let runner = runner().with_cwd("sass_import_below_css_import");
        assert_eq!(
            runner.ok("@use \"used\";\n\
             \n@import \"input.css\";\n"),
            "@import \"used.css\";\
         \n@import \"imported.css\";\
         \n@import \"input.css\";\n"
        );
    }
}
mod use_into_use {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("use_into_use")
    }

    #[test]
    fn import_above_rule() {
        let runner = runner().with_cwd("import_above_rule");
        assert_eq!(
            runner.ok("@use \"midstream\";\n\
             \n@import \"input.css\";\n"),
            "@import \"upstream.css\";\
         \n@import \"midstream.css\";\
         \n@import \"input.css\";\
         \na {\
         \n  file: upstream;\
         \n}\
         \na {\
         \n  file: midstream;\
         \n}\n"
        );
    }
    #[test]
    fn import_below_rule() {
        let runner = runner().with_cwd("import_below_rule");
        assert_eq!(
            runner.ok("@use \"midstream\";\n\
             \n@import \"input.css\";\n"),
            "@import \"upstream.css\";\
         \n@import \"midstream.css\";\
         \n@import \"input.css\";\
         \na {\
         \n  file: upstream;\
         \n}\
         \na {\
         \n  file: midstream;\
         \n}\n"
        );
    }
}
