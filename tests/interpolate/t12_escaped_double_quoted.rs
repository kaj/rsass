//! Tests from `spec/parser/interpolate/12_escaped_double_quoted`

use super::check;

#[test]
fn t01_inline() {
    check(".result {
  output: \"l\\\\ite\\ral\";
  output: #{\"l\\\\ite\\ral\"};
  output: \"[#{\"l\\\\ite\\ral\"}]\";
  output: \"#{\"l\\\\ite\\ral\"}\";
  output: '#{\"l\\\\ite\\ral\"}';
  output: \"['#{\"l\\\\ite\\ral\"}']\";
}
",
          ".result {
  output: \"l\\\\ite\\ral\";
  output: l\\iteral;
  output: \"[l\\\\iteral]\";
  output: \"l\\\\iteral\";
  output: \"l\\\\iteral\";
  output: \"['l\\\\iteral']\";
}
")
}

#[test]
fn t02_variable() {
    check("$input: \"l\\\\ite\\ral\";
.result {
  output: $input;
  output: #{$input};
  output: \"[#{$input}]\";
  output: \"#{$input}\";
  output: '#{$input}';
  output: \"['#{$input}']\";
}
",
          ".result {
  output: \"l\\\\iteral\";
  output: l\\iteral;
  output: \"[l\\\\iteral]\";
  output: \"l\\\\iteral\";
  output: \"l\\\\iteral\";
  output: \"['l\\\\iteral']\";
}
")
}

#[test]
fn t03_inline_double() {
    check(".result {
  output: #{#{\"l\\\\ite\\ral\"}};
  output: #{\"[#{\"l\\\\ite\\ral\"}]\"};
  output: #{\"#{\"l\\\\ite\\ral\"}\"};
  output: #{'#{\"l\\\\ite\\ral\"}'};
  output: #{\"['#{\"l\\\\ite\\ral\"}']\"};
}
",
          ".result {
  output: l\\iteral;
  output: [l\\iteral];
  output: l\\iteral;
  output: l\\iteral;
  output: ['l\\iteral'];
}
")
}
