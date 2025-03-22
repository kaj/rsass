//! Tests auto-converted from "sass-spec/spec/css/custom_properties/simple.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("simple")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            ".simple {\
             \n  --single: value;\
             \n  --multiple: value1 value2;\
             \n  --function: foo(bar);\
             \n  --url: url(http://foo.com/bar);\
             \n  --color: #foo;\
             \n  --exponent: 12.6e7;\
             \n  --close-comment: */;\n\
             \n  // The whitespace here DOES count as a token and needs to be preserved.\
             \n  --empty: ;\n\
             \n  // Single-line comments are not supported in variables.\
             \n  --single-line: // (\
             \n    );\n\
             \n  // Extra whitespace isn\'t added.\
             \n  --no-extra-whitespace:value;\
             \n}\n"
        ),
        ".simple {\
         \n  --single: value;\
         \n  --multiple: value1 value2;\
         \n  --function: foo(bar);\
         \n  --url: url(http://foo.com/bar);\
         \n  --color: #foo;\
         \n  --exponent: 12.6e7;\
         \n  --close-comment: */;\
         \n  --empty: ;\
         \n  --single-line: // (\
         \n    );\
         \n  --no-extra-whitespace:value;\
         \n}\n"
    );
}
