//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2863.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("issue_2863")
        .mock_file("input.scss ", "$squoted: \"'dquoted'\";\n$dquoted: \"\\\"squoted\\\"\";\n\ntest {\n    str-slice-single: str-slice( $squoted, 1, 2 );\n    str-slice-double: str-slice( $dquoted, 1, 2 );\n    str-index-single: str-index( $squoted, \"q\" );\n    str-index-double: str-index( $dquoted, \"q\" );\n    str-insert-single: str-insert( $squoted, \"p\", 2 );\n    str-insert-double: str-insert( $dquoted, \"p\", 2 );\n}")
        .mock_file("output.css ", "test {\n  str-slice-single: \"'d\";\n  str-slice-double: '\"s';\n  str-index-single: 3;\n  str-index-double: 3;\n  str-insert-single: \"'pdquoted'\";\n  str-insert-double: '\"psquoted\"';\n}\n")
}
