//! Tests auto-converted from "sass-spec/spec/directives/use/css/order/use_only.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("use_only")
        .mock_file("comment_order/diamond/comment_only/_left.scss", "/* before use in left */\n@use \"shared\";\n/* after use in left */\n")
        .mock_file("comment_order/diamond/comment_only/_right.scss", "/* before use in right */\n@use \"shared\";\n/* after use in right */\n")
        .mock_file("comment_order/diamond/comment_only/_shared.scss", "/* in shared */\n")
        .mock_file("comment_order/sequence/comment_and_css/_midstream.scss", "/* before use in midstream */\n@use \"upstream\";\n/* after use in midstream */\na {in: midstream}\n/* after css in midstream */\n")
        .mock_file("comment_order/sequence/comment_and_css/_upstream.scss", "/* before css in upstream */\na {in: upstream}\n/* after css in upstream */\n")
        .mock_file("comment_order/sequence/comment_css_and_plain_import/_midstream.scss", "/* before use in midstream */\n@use \"upstream\";\n/* after use in midstream */\n@import \"midstream.css\";\n/* after import in input */\na {in: midstream}\n/* after css in midstream */\n")
        .mock_file("comment_order/sequence/comment_css_and_plain_import/_upstream.scss", "/* before css in upstream */\n@import \"upstream.css\";\n/* after import in upstream */\na {in: upstream}\n/* after css in upstream */\n")
        .mock_file("comment_order/sequence/comment_only/_midstream.scss", "/* before use in midstream */\n@use \"upstream\";\n/* after use in midstream */\n")
        .mock_file("comment_order/sequence/comment_only/_upstream.scss", "/* in upstream */\n")
        .mock_file("diamond/left.scss", "@use \"shared\";\n\na {file: left}\n")
        .mock_file("diamond/right.scss", "@use \"shared\";\n\na {file: right}\n")
        .mock_file("diamond/shared.scss", "a {file: shared}\n")
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

mod comment_order {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("comment_order")
    }

    mod diamond {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("diamond")
        }

        #[test]
        fn comment_only() {
            let runner = runner().with_cwd("comment_only");
            assert_eq!(
                runner.ok("/* before use in input */\
             \n@use \"left\";\
             \n/* between use in input */\
             \n@use \"right\";\
             \n/* after use in input */\n"),
                "/* before use in input */\
         \n/* before use in left */\
         \n/* in shared */\
         \n/* after use in left */\
         \n/* between use in input */\
         \n/* before use in right */\
         \n/* after use in right */\
         \n/* after use in input */\n"
            );
        }
    }
    mod sequence {
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("sequence")
        }

        #[test]
        fn comment_and_css() {
            let runner = runner().with_cwd("comment_and_css");
            assert_eq!(
                runner.ok("/* before use in input */\
             \n@use \"midstream\";\
             \n/* after use in input */\
             \na {in: input}\
             \n/* after css in input */\n"),
                "/* before use in input */\
         \n/* before use in midstream */\
         \n/* before css in upstream */\
         \na {\
         \n  in: upstream;\
         \n}\
         \n/* after css in upstream */\
         \n/* after use in midstream */\
         \na {\
         \n  in: midstream;\
         \n}\
         \n/* after css in midstream */\
         \n/* after use in input */\
         \na {\
         \n  in: input;\
         \n}\
         \n/* after css in input */\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn comment_css_and_plain_import() {
            let runner = runner().with_cwd("comment_css_and_plain_import");
            assert_eq!(
                runner.ok("/* before use in input */\
             \n@use \"midstream\";\
             \n/* after use in input */\
             \n@import \"input.css\";\
             \n/* after import in input */\
             \na {in: input}\
             \n/* after css in input */\n"),
                "/* before use in input */\
         \n/* before use in midstream */\
         \n/* before css in upstream */\
         \n@import \"upstream.css\";\
         \n/* after use in midstream */\
         \n@import \"midstream.css\";\
         \n/* after use in input */\
         \n@import \"input.css\";\
         \n/* after import in upstream */\
         \na {\
         \n  in: upstream;\
         \n}\
         \n/* after css in upstream */\
         \n/* after import in input */\
         \na {\
         \n  in: midstream;\
         \n}\
         \n/* after css in midstream */\
         \n/* after import in input */\
         \na {\
         \n  in: input;\
         \n}\
         \n/* after css in input */\n"
            );
        }
        #[test]
        fn comment_only() {
            let runner = runner().with_cwd("comment_only");
            assert_eq!(
                runner.ok("/* before use in input */\
             \n@use \"midstream\";\
             \n/* after use in input */\n"),
                "/* before use in input */\
         \n/* before use in midstream */\
         \n/* in upstream */\
         \n/* after use in midstream */\
         \n/* after use in input */\n"
            );
        }
    }
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
