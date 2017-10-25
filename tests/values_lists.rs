//! These are from the `values/lists` directory in the sass specification.
extern crate rsass;
use rsass::{OutputStyle, compile_scss};

#[test]
fn brackets() {
    check(".bracketed-lists {
  empty: [];
  single: [foo];
  multiple: [foo, bar];
  nested: [[foo], [[bar, baz]]];
  space-separated: [foo bar baz];
  trailing-comma: [foo, bar,];

  // List functions treat it like a list.
  nth-comma: nth([foo, bar], 2);
  nth-space: nth([foo bar], 2);
  comma-separator: list-separator([foo, bar]);
  space-separator: list-separator([foo bar]);

  // List functions preserve bracketedness.
  set-nth: set-nth([foo, bar, baz], 2, qux);
  append: append([foo, bar], baz);
  append-with-separator: append([foo, bar], baz, $separator: space);

  // Inspection produces valid input. This also verifies that nested lists of
  // various sorts are parsed properly.
  inspect-empty: inspect([]);
  inspect-simple: inspect([foo, bar]);
  inspect-nested-bracketed: inspect([[foo]]);
  inspect-nested-unbracketed: inspect([(foo bar)]);
  inspect-nested-unbracketed-comma: inspect([foo bar,]);
  inspect-nested-unbracketed-singleton: inspect([(foo,)]);
}
",
          ".bracketed-lists {
  empty: [];
  single: [foo];
  multiple: [foo, bar];
  nested: [[foo], [[bar, baz]]];
  space-separated: [foo bar baz];
  trailing-comma: [foo, bar];
  nth-comma: bar;
  nth-space: bar;
  comma-separator: comma;
  space-separator: space;
  set-nth: [foo, qux, baz];
  append: [foo, bar, baz];
  append-with-separator: [foo bar baz];
  inspect-empty: [];
  inspect-simple: [foo, bar];
  inspect-nested-bracketed: [[foo]];
  inspect-nested-unbracketed: [(foo bar)];
  inspect-nested-unbracketed-comma: [foo bar,];
  inspect-nested-unbracketed-singleton: [(foo,)];
}
")
}

#[test]
fn equality() {
    check("a {\n  \
           @if [foo bar]==[foo bar] {\n    t1: t;\n  } \
           @else {\n    f1: f;\n  }\n\n\
           @if [foo bar]==[foo, bar] {\n    t2: t;\n  } \
           @else {\n    f2: f;\n  }\n\n\
           @if [foo bar]==(foo bar) {\n    t3: t;\n  } \
           @else {\n    f3: f;\n  }\n\n\
           @if [] == [] {\n    t4: t;\n  } @else {\n    f4: f;\n  }\n\n\
           @if [] == () {\n    t5: t;\n  } @else {\n    f5: f;\n  }\n}\n",
          "a {\n  t1: t;\n  f2: f;\n  f3: f;\n  t4: t;\n  f5: f;\n}\n")
}

fn check(input: &str, expected: &str) {
    assert_eq!(compile_scss(input.as_bytes(), OutputStyle::Normal)
                   .and_then(|s| Ok(String::from_utf8(s)?))
                   .unwrap(),
               expected);
}
