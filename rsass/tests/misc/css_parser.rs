use rsass::input::{FsContext, SourceFile, SourceName};

#[test]
fn error_in_right_place() {
    assert_eq!(
        parse_css(
            "@font-face {\
             \n  font-family: 'Open Sans';\
             \n  font-style: normal;\
             \n  font-weight: 300;\
             \n  error here\
             \n  font-display: swap;\
             \n}\n"
        ),
        // Note: The error message should be better, but this is a good place for it.
        // Specifically, the marker should _not_ indicate the opening brace.
        Err(String::from(
            "Error: Parse error: Tag\
             \n  ,\
             \n5 |   error here\
             \n  |   ^\
             \n  '\
             \n  - 5:3  root stylesheet"
        ))
    );
}

#[test]
fn url_as_function() {
    assert_eq!(
        parse_css("x { src: url(../fonts/memGs12-U_rX-h.woff2); }\n"),
        Ok("x {\n  src: url(../fonts/memGs12-U_rX-h.woff2);\n}\n".into())
    );
}

#[test]
#[ignore]
fn uncode_range() {
    todo!();
}

fn parse_css(data: &str) -> Result<String, String> {
    FsContext::for_cwd()
        .with_format(Default::default())
        .transform(SourceFile::css_bytes(data, SourceName::root("-")))
        .map(|s| String::from_utf8_lossy(&s).to_string())
        .map_err(|e| {
            // The unescaped printed error is easier to read in case of error.
            println!("{e}");
            e.to_string()
        })
}
