// https://users.rust-lang.org/t/iterator-need-to-identify-the-last-element\
// /8836/8

use std::iter;

pub trait IdentifyLast: Iterator + Sized {
    fn identify_last(self) -> Iter<Self>;
}

impl<I> IdentifyLast for I
    where I: Iterator
{
    fn identify_last(self) -> Iter<Self> {
        Iter(true, self.peekable())
    }
}

pub struct Iter<I>(bool, iter::Peekable<I>) where I: Iterator;

impl<I> Iterator for Iter<I>
    where I: Iterator
{
    type Item = (bool, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|e| (self.1.peek().is_none(), e))
    }
}

#[test]
fn test_identify_last() {
    assert_eq!("1, 2, 3, 4.",
               (1..5).identify_last()
               .map(|(is_last, e)| {
                   format!("{}{}", e, if is_last { "." } else { "," })
               })
               .collect::<Vec<_>>()
               .join(" "))
}
