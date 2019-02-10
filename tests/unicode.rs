use rsass::{compile_scss, OutputStyle};

#[test]
fn unicode_in_expanded() {
    // Blåbärsöl is a proper swedish word.  Translates to blueberry beer.
    check(
        "a {\n  \
         content: \"Blåbärsöl\";\n\
         }",
        "@charset \"UTF-8\";\n\
         a {\n  content: \"Blåbärsöl\";\n}\n",
        OutputStyle::Expanded,
    )
}

#[test]
fn unicode_in_compressed() {
    // Blåbärsöl is a proper swedish word.  Translates to blueberry beer.
    // The charset declaration is replaced with a byte order mark.
    check(
        "a {\n  \
         content: \"Blåbärsöl\";\n\
         }",
        "\u{feff}a{content:\"Blåbärsöl\"}\n",
        OutputStyle::Compressed,
    )
}

fn check(input: &str, expected: &str, style: OutputStyle) {
    assert_eq!(
        compile_scss(input.as_bytes(), style)
            .and_then(|s| Ok(String::from_utf8(s)?))
            .unwrap(),
        expected
    );
}
