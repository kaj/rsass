macro_rules! one_arg {
    ($name:ident) => {
        (stringify!($name).into(), $crate::sass::Value::Null)
    };
    ($name:ident = $value:expr) => {{
        use $crate::parser::value::value_expression;
        (
            stringify!($name).into(),
            value_expression($value).unwrap().1,
        )
    }};
}

macro_rules! func {
    (( $($arg:ident $( = $value:expr )* ),* ), $body:expr) => {{
        use std::sync::Arc;
        SassFunction::builtin(vec![ $( one_arg!($arg $( = $value)* ) ),* ],
                              false,
                              Arc::new($body))
    }};
}
macro_rules! func_va {
    (( $($arg:ident $( = $value:expr )* ),* ), $body:expr) => {{
        use std::sync::Arc;
        SassFunction::builtin(vec![ $( one_arg!($arg $( = $value)* ) ),* ],
                              true,
                              Arc::new($body))
    }};
}
macro_rules! def {
    ($f:expr, $name:ident( $($arg:ident$(=$val:expr)* ),* ), $body:expr) => {
        $f.insert(
            stringify!($name),
            func!(($($arg $( = $val )* ),*), $body),
        )
    }
}
macro_rules! def_va {
    ($f:expr, $name:ident( $($arg:ident$(=$value:expr)* ),*), $body:expr) => {
        $f.insert(stringify!($name),
                  func_va!(($($arg $( = $value )* ),*), $body))
    }
}

macro_rules! func2 {
    ($name:ident( $($arg:ident $( = $value:expr )* ),* )) => {
        func!(($($arg $( = $value )* ),*),
              |s: &dyn Scope| $name($(s.get(stringify!($arg))?),*))
    };
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
