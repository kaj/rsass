macro_rules! name {
    ($name:ident) => {
        crate::sass::Name::from_static(stringify!($name))
    };
    () => {
        // an empty name
        crate::sass::Name::from_static("")
    };
}

macro_rules! one_arg {
    ($name:ident) => {
        (stringify!($name).into(), $crate::sass::Value::Null)
    };
    ($name:ident = $value:expr) => {{
        use $crate::parser::parse_value_data;
        (
            stringify!($name).into(),
            parse_value_data($value).expect(stringify!($name)),
        )
    }};
}

macro_rules! func {
    ($module:expr, $name:expr, ( $($arg:ident $( = $value:expr )* ),* ), $body:expr) => {{
        use crate::SourcePos;
        use crate::sass::Function;
        use std::sync::Arc;
        Function::builtin(vec![ $( one_arg!($arg $( = $value)* ) ),* ],
                          false,
                          SourcePos::mock_function($name, &[$(name!($arg)),*], $module),
                          Arc::new($body))
    }};
}
macro_rules! func_va {
    (( $($arg:ident $( = $value:expr )* ),* ), $body:expr) => {{
        use crate::parser::code_span;
        use crate::sass::Function;
        use std::sync::Arc;
        Function::builtin(vec![ $( one_arg!($arg $( = $value)* ) ),* ],
                          true,
                          // FIXME
                          code_span(b"@function").into(),
                          Arc::new($body))
    }};
}
macro_rules! def {
    ($f:expr, $name:ident( $($arg:ident$(=$val:expr)* ),* ), $body:expr) => {
        let name = name!($name);
        $f.define_function(
            name.clone(),
            func!(&$f.get_name(), name, ($($arg $( = $val )* ),*), $body),
        )
    }
}
macro_rules! def_va {
    ($f:expr, $name:ident( $($arg:ident$(=$value:expr)* ),*), $body:expr) => {
        $f.define_function(name!($name),
                  func_va!(($($arg $( = $value )* ),*), $body))
    }
}

macro_rules! dep_warn {
    ($first: expr, $($arg:expr),*) => {{
        use std::sync::Once;
        static WARN: Once = Once::new();
        WARN.call_once(|| {
            eprintln!(concat!("DEPRECATION WARNING: ", $first), $($arg),*);
        });
    }};
    ($first: expr) => {{
        use std::sync::Once;
        static WARN: Once = Once::new();
        WARN.call_once(|| eprintln!(concat!("DEPRECATION WARNING: ", $first)));
    }}
}
