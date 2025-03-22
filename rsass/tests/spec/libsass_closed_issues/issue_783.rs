//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_783.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_783")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "// $a: 12px / 1em;\
             \n// $b: 6px / 1em;\
             \n// $c: 10em;\
             \n// $x: -9999em;\
             \n// $aa: 1px * 1px;\n\
             \na {\
             \n  $foo: 2em;\
             \n  $bar: 2em;\n\
             \n  foo: $foo;          // 2em  ✔\
             \n  bar: $bar;          // 2em  ✔\
             \n  // a: $foo * $bar;     // 4em*em isn\'t a valid CSS value.  ✔\
             \n  a: $foo / $bar;     // 1  ✔\
             \n  a: $foo + $bar;     // 4em  ✔\
             \n  a: $foo - $bar;     // 0em  ✔\n\n\
             \n  $foo: 2px;\
             \n  $bar: 2em;\n\
             \n  foo: $foo;          // 2px  ✔\
             \n  bar: $bar;          // 2em  ✔\
             \n  // a: $foo * $bar;     // 4em*px isn\'t a valid CSS value.  ✔\
             \n  // a: $foo / $bar;     // 1px/em isn\'t a valid CSS value.  ✔\
             \n  // a: $foo + $bar;     // Incompatible units: \'em\' and \'px\'.  ✔\
             \n  // a: $foo - $bar;     // Incompatible units: \'em\' and \'px\'.  ✔\n\n\
             \n  $foo: 2em;\
             \n  $bar: 2px;\n\
             \n  foo: $foo;          // 2em  ✔\
             \n  bar: $bar;          // 2px  ✔\
             \n  // a: $foo * $bar;     // 4em*px isn\'t a valid CSS value.  ✔\
             \n  // a: $foo / $bar;     // 1em/px isn\'t a valid CSS value.  ✔\
             \n  // a: $foo + $bar;     // Incompatible units: \'px\' and \'em\'.  ✔\
             \n  // a: $foo - $bar;     // Incompatible units: \'px\' and \'em\'.  ✔\n\n\
             \n  $foo: 2px / 2em;\
             \n  $bar: 2px;\n\
             \n  // foo: $foo;          // 1px/em isn\'t a valid CSS value.  ✔\
             \n  bar: $bar;          // 2px  ✔\
             \n  // a: $foo * $bar;     // 2px*px/em isn\'t a valid CSS value.  ✔\
             \n  // a: $foo / $bar;     // 0.5/em isn\'t a valid CSS value.  ✔\
             \n  // a: $foo + $bar;     // Incompatible units: \'\' and \'em\'.\
             \n  // a: $foo - $bar;     // Incompatible units: \'\' and \'em\'.\n\n\
             \n  $foo: 2em / 2px;\
             \n  $bar: 2px;\n\
             \n  // foo: $foo;          // 1em/px isn\'t a valid CSS value.  ✔\
             \n  bar: $bar;          // 2px  ✔\
             \n  a: $foo * $bar;     // 2em  ✔\
             \n  // a: $foo / $bar;     // 0.5em/px*px isn\'t a valid CSS value.  ✔\
             \n  // a: $foo + $bar;     // Incompatible units: \'px\' and \'em\'.\
             \n  // a: $foo - $bar;     // Incompatible units: \'px\' and \'em\'.\n\n\
             \n  $foo: 2em / 2px;\
             \n  $bar: 2em / 2px;\n\
             \n  // foo: $foo;          // 1em/px isn\'t a valid CSS value.  ✔\
             \n  // bar: $bar;          // 1em/px isn\'t a valid CSS value.  ✔\
             \n  // a: $foo * $bar;     // 1em*em/px*px isn\'t a valid CSS value.  ✔\
             \n  a: $foo / $bar;     // 1  ✔\
             \n  // a: $foo + $bar;     // 2em/px isn\'t a valid CSS value.  ✔\
             \n  // a: $foo - $bar;     // 0em/px isn\'t a valid CSS value.  ✔\n\n\
             \n  $foo: 2px / 2em;\
             \n  $bar: 2em / 2px;\n\
             \n  // foo: $foo;          // 1px/em isn\'t a valid CSS value.  ✔\
             \n  // bar: $bar;          // 1em/px isn\'t a valid CSS value.  ✔\
             \n  a: $foo * $bar;     // 1  ✔\
             \n  // a: $foo / $bar;     // 1px*px/em*em isn\'t a valid CSS value.  ✔\
             \n  // a: $foo + $bar;     // Incompatible units: \'em\' and \'px\'.\
             \n  // a: $foo - $bar;     // Incompatible units: \'em\' and \'px\'.\n\n\
             \n  $foo: 2px;\
             \n  $bar: 2px / 2em;\n\
             \n  foo: $foo;          // 2px  ✔\
             \n  // bar: $bar;          // 1px/em isn\'t a valid CSS value.  ✔\
             \n  // a: $foo * $bar;     // 2px*px/em isn\'t a valid CSS value.  ✔\
             \n  a: $foo / $bar;     // 2em  ✔\
             \n  // a: $foo + $bar;     // Incompatible units: \'em\' and \'\'.\
             \n  // a: $foo - $bar;     // Incompatible units: \'em\' and \'\'.\n\n\
             \n  $foo: 2px;\
             \n  $bar: 2em / 2px;\n\
             \n  foo: $foo;          // 2px  ✔\
             \n  // bar: $bar;          // 1em/px isn\'t a valid CSS value.  ✔\
             \n  a: $foo * $bar;     // 2em  ✔\
             \n  // a: $foo / $bar;     // 2px*px/em isn\'t a valid CSS value.  ✔\
             \n  // a: $foo + $bar;     // Incompatible units: \'em\' and \'px\'.\
             \n  // a: $foo - $bar;     // Incompatible units: \'em\' and \'px\'.\
             \n}\n"
        ),
        "a {\
         \n  foo: 2em;\
         \n  bar: 2em;\
         \n  a: 1;\
         \n  a: 4em;\
         \n  a: 0em;\
         \n  foo: 2px;\
         \n  bar: 2em;\
         \n  foo: 2em;\
         \n  bar: 2px;\
         \n  bar: 2px;\
         \n  bar: 2px;\
         \n  a: 2em;\
         \n  a: 1;\
         \n  a: 1;\
         \n  foo: 2px;\
         \n  a: 2em;\
         \n  foo: 2px;\
         \n  a: 2em;\
         \n}\n"
    );
}
