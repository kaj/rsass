/// A special kind of option relevant for selectors.
///
/// There is both a positive (match-anything) and a negative (match-nothing)
/// empty value.
#[derive(Debug)]
pub(crate) enum Opt<T> {
    Some(T),
    Any,
    None,
}

impl<T> Opt<T> {
    pub(crate) fn collect_pos(
        iter: impl Iterator<Item = Opt<T>>,
    ) -> Opt<Vec<T>> {
        let mut result = Vec::new();
        for p in iter {
            match p {
                Opt::Some(p) => result.push(p),
                Opt::Any => return Opt::Any,
                Opt::None => (),
            }
        }
        if result.is_empty() {
            Opt::None
        } else {
            Opt::Some(result)
        }
    }
    pub(crate) fn collect_neg(
        iter: impl Iterator<Item = Opt<T>>,
    ) -> Opt<Vec<T>> {
        let mut result = Vec::new();
        for p in iter {
            match p {
                Opt::Some(p) => result.push(p),
                Opt::Any => (),
                Opt::None => return Opt::None,
            }
        }
        if result.is_empty() {
            Opt::Any
        } else {
            Opt::Some(result)
        }
    }
    pub(crate) fn map<U, F: Fn(T) -> U>(self, f: F) -> Opt<U> {
        match self {
            Opt::Some(t) => Opt::Some(f(t)),
            Opt::Any => Opt::Any,
            Opt::None => Opt::None,
        }
    }
}

impl<T> Opt<Vec<T>> {
    pub(crate) fn positive(self) -> Option<Vec<T>> {
        match self {
            Opt::Some(v) => Some(v),
            Opt::Any => Some(vec![]),
            Opt::None => None,
        }
    }
}
