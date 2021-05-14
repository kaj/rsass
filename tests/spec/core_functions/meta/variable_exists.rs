//! Tests auto-converted from "sass-spec/spec/core_functions/meta/variable_exists.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("conflict/other1.scss", "$member: from other1;\n")
        .mock_file("conflict/other2.scss", "$member: from other2;\n")
        .mock_file(
            "through_forward/as/_midstream.scss",
            "@forward \"upstream\" as b-*;\n",
        )
        .mock_file("through_forward/as/_upstream.scss", "$c: null;\n")
        .mock_file(
            "through_forward/hide/_midstream.scss",
            "@forward \"upstream\" hide $b;\n",
        )
        .mock_file(
            "through_forward/hide/_upstream.scss",
            "$b: null;\n$c: null;\n",
        )
        .mock_file(
            "through_forward/show/_midstream.scss",
            "@forward \"upstream\" show $b;\n",
        )
        .mock_file(
            "through_forward/show/_upstream.scss",
            "$b: null;\n$c: null;\n",
        )
        .mock_file("through_import/other.scss", "$global-variable: null;\n")
        .mock_file("through_use/other.scss", "$global-variable: null;\n")
}

#[test]
#[ignore] // missing error
fn conflict() {
    let runner = runner().with_cwd("conflict");
    assert_eq!(
        runner.err(
            "@use \"other1\" as *;\
             \n@use \"other2\" as *;\n\
             \na {b: variable-exists(member)}\n"
        ),
        "Error: This variable is available from multiple global modules.\
         \n    ,\
         \n1   | @use \"other1\" as *;\
         \n    | ================== includes variable\
         \n2   | @use \"other2\" as *;\
         \n    | ================== includes variable\
         \n... |\
         \n4   | a {b: variable-exists(member)}\
         \n    |       ^^^^^^^^^^^^^^^^^^^^^^^ variable use\
         \n    \'\
         \n  input.scss 4:7  root stylesheet",
    );
}
mod dash_insensitive {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("dash_insensitive")
    }

    #[test]
    fn dash_to_underscore() {
        let runner = runner().with_cwd("dash_to_underscore");
        assert_eq!(
            runner.ok("$a_b: null;\n\
             \nc {d: variable-exists(a-b)}\n"),
            "c {\
         \n  d: true;\
         \n}\n"
        );
    }
    #[test]
    fn underscore_to_dash() {
        let runner = runner().with_cwd("underscore_to_dash");
        assert_eq!(
            runner.ok("$a-b: null;\n\
             \nc {d: variable-exists(a_b)}\n"),
            "c {\
         \n  d: true;\
         \n}\n"
        );
    }
}
mod error {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("error")
    }

    mod argument {
        #[allow(unused)]
        fn runner() -> crate::TestRunner {
            super::runner().with_cwd("argument")
        }

        #[test]
        fn too_few() {
            let runner = runner().with_cwd("too_few");
            assert_eq!(
                runner.err("a {b: variable-exists()}\n"),
                "Error: Missing argument $name.\
         \n  ,--> input.scss\
         \n1 | a {b: variable-exists()}\
         \n  |       ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function variable-exists($name) {\
         \n  |           ====================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn too_many() {
            let runner = runner().with_cwd("too_many");
            assert_eq!(
                runner.err("a {b: variable-exists(foo, bar)}\n"),
                "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: variable-exists(foo, bar)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function variable-exists($name) {\
         \n  |           ====================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn test_type() {
            let runner = runner().with_cwd("type");
            assert_eq!(
                runner.err("a {b: variable-exists(12px)}\n"),
                "Error: $name: 12px is not a string.\
         \n  ,\
         \n1 | a {b: variable-exists(12px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
#[test]
fn global() {
    let runner = runner().with_cwd("global");
    assert_eq!(
        runner.ok("$global-variable: null;\n\
             \na {b: variable-exists(global-variable)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn keyword() {
    let runner = runner().with_cwd("keyword");
    assert_eq!(
        runner.ok("a {b: variable-exists($name: foo)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn local() {
    let runner = runner().with_cwd("local");
    assert_eq!(
        runner.ok("a {\
             \n  $local-variable: null;\
             \n  b: variable-exists(local-variable);\
             \n}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn non_existent() {
    let runner = runner().with_cwd("non_existent");
    assert_eq!(
        runner.ok("a {\
             \n  b: variable-exists(non-existent);\
             \n}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
mod through_forward {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("through_forward")
    }

    #[test]
    #[ignore] // wrong result
    fn test_as() {
        let runner = runner().with_cwd("as");
        assert_eq!(
            runner.ok("@use \"midstream\" as *;\
             \na {\
             \n  with-prefix: variable-exists(b-c);\
             \n  without-prefix: variable-exists(c);\
             \n}\n"),
            "a {\
         \n  with-prefix: true;\
         \n  without-prefix: false;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn hide() {
        let runner = runner().with_cwd("hide");
        assert_eq!(
            runner.ok("@use \"midstream\" as *;\
             \na {\
             \n  hidden: variable-exists(b);\
             \n  not-hidden: variable-exists(c);\
             \n}\n"),
            "a {\
         \n  hidden: false;\
         \n  not-hidden: true;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn show() {
        let runner = runner().with_cwd("show");
        assert_eq!(
            runner.ok("@use \"midstream\" as *;\
             \na {\
             \n  shown: variable-exists(b);\
             \n  not-shown: variable-exists(c);\
             \n}\n"),
            "a {\
         \n  shown: true;\
         \n  not-shown: false;\
         \n}\n"
        );
    }
}
#[test]
fn through_import() {
    let runner = runner().with_cwd("through_import");
    assert_eq!(
        runner.ok("@import \"other\";\
             \na {b: variable-exists(global-variable)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn through_use() {
    let runner = runner().with_cwd("through_use");
    assert_eq!(
        runner.ok("@use \"other\" as *;\
             \na {b: variable-exists(global-variable)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
