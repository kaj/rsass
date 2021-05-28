//! Tests auto-converted from "sass-spec/spec/directives/use/css/order.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("diamond/left.scss", "@use \"shared\";\n\na {file: left}\n")
        .mock_file("diamond/right.scss", "@use \"shared\";\n\na {file: right}\n")
        .mock_file("diamond/shared.scss", "a {file: shared}\n")
        .mock_file("import_order/comments_and_imports/midstream.scss", "/* midstream comment before use */\n@use \"upstream\";\n\n/* midstream comment before first import */\n@import \"midstream1.css\";\n\n/* midstream comment before second import */\n@import \"midstream2.css\";\n\n/* midstream comment after imports */\n\na {file: midstream}\n")
        .mock_file("import_order/comments_and_imports/upstream.scss", "/* upstream comment before import */\n@import \"upstream.css\";\n\n/* upstream comment after import */\n")
        .mock_file("import_order/import_into_use/css_import_above_rule/_imported.scss", "@use \"used\";\n\n@import \"imported.css\";\n\na {file: imported}\n")
        .mock_file("import_order/import_into_use/css_import_above_rule/_used.scss", "@import \"used.css\";\n\na {file: used}\n")
        .mock_file("import_order/import_into_use/css_import_below_rule/_imported.scss", "@use \"used\";\n\na {file: imported}\n\n@import \"imported.css\";\n")
        .mock_file("import_order/import_into_use/css_import_below_rule/_used.scss", "a {file: used}\n\n@import \"used.css\";\n")
        .mock_file("import_order/import_into_use/sass_import_below_css_import/_imported.scss", "@use \"used\";\n\n@import \"imported.css\";\n")
        .mock_file("import_order/import_into_use/sass_import_below_css_import/_used.scss", "@import \"used.css\";\n")
        .mock_file("import_order/use_into_import/css_import_above_rule/_imported.scss", "@import \"imported.css\";\n\na {file: imported}\n")
        .mock_file("import_order/use_into_import/css_import_above_rule/_used.scss", "@import \"imported\";\n\n@import \"used.css\";\n\na {file: used}\n")
        .mock_file("import_order/use_into_import/css_import_below_rule/_imported.scss", "a {file: imported}\n\n@import \"imported.css\";\n")
        .mock_file("import_order/use_into_import/css_import_below_rule/_used.scss", "@import \"imported\";\n\na {file: used}\n\n@import \"used.css\";\n")
        .mock_file("import_order/use_into_import/sass_import_below_css_import/_imported.scss", "@import \"imported.css\";\n")
        .mock_file("import_order/use_into_import/sass_import_below_css_import/_used.scss", "@import \"used.css\";\n\n@import \"imported\";\n")
        .mock_file("import_order/use_into_use/import_above_rule/_midstream.scss", "@use \"upstream\";\n\n@import \"midstream.css\";\n\na {file: midstream}\n")
        .mock_file("import_order/use_into_use/import_above_rule/_upstream.scss", "@import \"upstream.css\";\n\na {file: upstream}\n")
        .mock_file("import_order/use_into_use/import_below_rule/_midstream.scss", "@use \"upstream\";\n\na {file: midstream}\n\n@import \"midstream.css\";\n")
        .mock_file("import_order/use_into_use/import_below_rule/_upstream.scss", "a {file: upstream}\n\n@import \"upstream.css\";\n")
        .mock_file("once/_other.scss", "a {b: c}\n")
        .mock_file("triangle/midstream.scss", "@use \"upstream\";\n\na {file: midstream}\n")
        .mock_file("triangle/upstream.scss", "a {file: upstream}\n")
        .mock_file("unrelated_branches/left_midstream.scss", "@use \"left_upstream\";\n\na {file: left midstream}\n")
        .mock_file("unrelated_branches/left_upstream.scss", "a {file: left upstream}\n")
        .mock_file("unrelated_branches/right_midstream.scss", "@use \"right_upstream\";\n\na {file: right midstream}\n")
        .mock_file("unrelated_branches/right_upstream.scss", "a {file: right upstream}\n")
        .mock_file("use_into_use/midstream.scss", "@use \"upstream\";\n\na {file: midstream}\n")
        .mock_file("use_into_use/upstream.scss", "a {file: upstream}\n")
        .mock_file("use_order/other1.scss", "a {file: other1}\n")
        .mock_file("use_order/other2.scss", "a {file: other2}\n")
        .mock_file("use_order/other3.scss", "a {file: other3}\n")
}

#[test]
fn diamond() {
    let runner = runner().with_cwd("diamond");
    assert_eq!(
        runner.ok("@use \"left\";\
             \n@use \"right\";\n\
             \na {file: input}\n"),
        "a {\
         \n  file: shared;\
         \n}\
         \na {\
         \n  file: left;\
         \n}\
         \na {\
         \n  file: right;\
         \n}\
         \na {\
         \n  file: input;\
         \n}\n"
    );
}
mod import_order {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("import_order")
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
            "/* upstream comment before import */\
         \n@import \"upstream.css\";\
         \n/* midstream comment before use */\
         \n/* midstream comment before first import */\
         \n@import \"midstream1.css\";\
         \n/* midstream comment before second import */\
         \n@import \"midstream2.css\";\
         \n/* input comment before use */\
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
        #[allow(unused)]
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
        #[allow(unused)]
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
        #[allow(unused)]
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
}
#[test]
fn once() {
    let runner = runner().with_cwd("once");
    assert_eq!(
        runner.ok("@use \"other\" as o1;\
             \n@use \"other\" as o2;\
             \n@use \"other\" as o3;\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
fn triangle() {
    let runner = runner().with_cwd("triangle");
    assert_eq!(
        runner.ok("@use \"midstream\";\
             \n@use \"upstream\";\n\
             \na {file: input}\n"),
        "a {\
         \n  file: upstream;\
         \n}\
         \na {\
         \n  file: midstream;\
         \n}\
         \na {\
         \n  file: input;\
         \n}\n"
    );
}
#[test]
fn unrelated_branches() {
    let runner = runner().with_cwd("unrelated_branches");
    assert_eq!(
        runner.ok("@use \"left_midstream\";\
             \n@use \"right_midstream\";\n\
             \na {file: input}\n"),
        "a {\
         \n  file: left upstream;\
         \n}\
         \na {\
         \n  file: left midstream;\
         \n}\
         \na {\
         \n  file: right upstream;\
         \n}\
         \na {\
         \n  file: right midstream;\
         \n}\
         \na {\
         \n  file: input;\
         \n}\n"
    );
}
#[test]
fn use_into_use() {
    let runner = runner().with_cwd("use_into_use");
    assert_eq!(
        runner.ok("@use \"midstream\";\n\
             \na {file: input}\n"),
        "a {\
         \n  file: upstream;\
         \n}\
         \na {\
         \n  file: midstream;\
         \n}\
         \na {\
         \n  file: input;\
         \n}\n"
    );
}
#[test]
fn use_order() {
    let runner = runner().with_cwd("use_order");
    assert_eq!(
        runner.ok("@use \"other1\";\
             \n@use \"other2\";\
             \n@use \"other3\";\n\
             \na {file: input}\n"),
        "a {\
         \n  file: other1;\
         \n}\
         \na {\
         \n  file: other2;\
         \n}\
         \na {\
         \n  file: other3;\
         \n}\
         \na {\
         \n  file: input;\
         \n}\n"
    );
}
