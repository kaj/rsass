//! Tests auto-converted from "sass-spec/spec/css/propset.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("propset")
}

mod comment {
    use super::runner;

    mod after_block {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn loud() {
            assert_eq!(
                runner().ok("a {b: {c: d} /**/}\n"),
                "a {\
         \n  b-c: d; /**/\
         \n}\n"
            );
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("a {\
             \n  b: {c: d} //\
             \n}\n"),
                "a {\
         \n  b-c: d;\
         \n}\n"
            );
        }
    }
    mod before_block {
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(
                runner().ok("a {b: /**/ {c: d}}\n"),
                "a {\
         \n  b-c: d;\
         \n}\n"
            );
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("a {\
             \n  b: //\
             \n    {c: d}\
             \n}\n"),
                "a {\
         \n  b-c: d;\
         \n}\n"
            );
        }
    }
}
#[test]
fn complex() {
    assert_eq!(
        runner().ok("a { b: c { d: e } }\n"),
        "a {\
         \n  b: c;\
         \n  b-d: e;\
         \n}\n"
    );
}
#[test]
fn custom_property_value() {
    assert_eq!(
        runner().ok("a { b: { c: --d } }\n"),
        "a {\
         \n  b-c: --d;\
         \n}\n"
    );
}
mod error {
    use super::runner;

    mod custom_property {
        use super::runner;

        mod nested {
            use super::runner;

            #[test]
            #[ignore] // wrong error
            fn complex() {
                assert_eq!(
        runner().err(
            "a { b: { --d: e {--f: g} } }\n"
        ),
        "Error: Declarations whose names begin with \"--\" may not be nested.\
         \n  ,\
         \n1 | a { b: { --d: e {--f: g} } }\
         \n  |          ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:10  root stylesheet",
    );
            }
            #[test]
            #[ignore] // wrong error
            fn simple() {
                assert_eq!(
        runner().err(
            "a { b: { --d: {e: f} } }\n"
        ),
        "Error: Declarations whose names begin with \"--\" may not be nested.\
         \n  ,\
         \n1 | a { b: { --d: {e: f} } }\
         \n  |          ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:10  root stylesheet",
    );
            }
        }
        #[test]
        #[ignore] // wrong error
        fn simple() {
            assert_eq!(
        runner().err(
            "a { b: { --d: e } }\n"
        ),
        "Error: Declarations whose names begin with \"--\" may not be nested.\
         \n  ,\
         \n1 | a { b: { --d: e } }\
         \n  |          ^^^^^^^\
         \n  \'\
         \n  input.scss 1:10  root stylesheet",
    );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn value_after_propset() {
        assert_eq!(
            runner().err("a { b: { d: e } f }\n"),
            "Error: expected \"{\".\
         \n  ,\
         \n1 | a { b: { d: e } f }\
         \n  |                   ^\
         \n  \'\
         \n  input.scss 1:19  root stylesheet",
        );
    }
}
#[test]
fn nested() {
    assert_eq!(
        runner().ok("a { b: { c: { d: e }; f: g } }\n"),
        "a {\
         \n  b-c-d: e;\
         \n  b-f: g;\
         \n}\n"
    );
}
#[test]
fn simple() {
    assert_eq!(
        runner().ok("a { b: { c: d } }\n"),
        "a {\
         \n  b-c: d;\
         \n}\n"
    );
}
#[test]
fn with_dash_prefix() {
    assert_eq!(
        runner().ok("a { b: { -c: d } }\n"),
        "a {\
         \n  b--c: d;\
         \n}\n"
    );
}
