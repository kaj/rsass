
macro_rules! one_arg {
    ($name:ident) => {
        (stringify!($name).into(), Value::Null)
    };
    ($name:ident = $value:expr) => {{
        use valueexpression::value_expression;
        (stringify!($name).into(), value_expression($value).unwrap().1)
    }};
}

macro_rules! func {
    (( $($arg:ident $( = $value:expr )* ),* ), $body:expr) => {
        SassFunction::builtin(vec![ $( one_arg!($arg $( = $value)* ) ),* ],
                              Box::new($body))
    };
}
macro_rules! def {
    ($f:expr, $name:ident( $($arg:ident$(=$value:expr)* ),* ), $body:expr) => {
        $f.insert(stringify!($name),
                  func!(($($arg $( = $value )* ),*), $body))
    }
}

macro_rules! func2 {
    ($name:ident( $($arg:ident $( = $value:expr )* ),* )) => {
        func!(($($arg $( = $value )* ),*),
              |s: &Scope| $name($(s.get(stringify!($arg))),*))
    };
}
