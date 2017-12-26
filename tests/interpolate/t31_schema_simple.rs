//! Tests from `spec/parser/interpolate/31_schema_simple`

use super::check;

#[test]
fn t01_inline() {
    check(".result {
  output: \"[\"'foo'\"]\";
  output: #{\"[\"'foo'\"]\"};
  output: \"[#{\"[\"'foo'\"]\"}]\";
  output: \"#{\"[\"'foo'\"]\"}\";
  output: '#{\"[\"'foo'\"]\"}';
  output: \"['#{\"[\"'foo'\"]\"}']\";
}
",
          ".result {
  output: \"[\" \"foo\" \"]\";
  output: [ foo ];
  output: \"[[ foo ]]\";
  output: \"[ foo ]\";
  output: \"[ foo ]\";
  output: \"['[ foo ]']\";
}
")
}
