use std::iter::{Fuse, FusedIterator, Peekable};

/// An iterator adaptor that wraps each element in an [`Position`].
///
/// Iterator element type is `(Position, I::Item)`.
///
/// See [`.with_position()`](crate::Itertools::with_position) for more information.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct WithPosition<I>
where
    I: Iterator,
{
    handled_first: bool,
    peekable: Peekable<Fuse<I>>,
}

impl<I> Clone for WithPosition<I>
where
    I: Clone + Iterator,
    I::Item: Clone,
{
    clone_fields!(handled_first, peekable);
}

/// Create a new `WithPosition` iterator.
pub fn with_position<I>(iter: I) -> WithPosition<I>
where
    I: Iterator,
{
    WithPosition {
        handled_first: false,
        peekable: iter.fuse().peekable(),
    }
}

/// The first component of the value yielded by `WithPosition`.
/// Indicates the position of this element in the iterator results.
///
/// See [`.with_position()`](crate::Itertools::with_position) for more information.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Position {
    /// This is the first element.
    First,
    /// This is neither the first nor the last element.
    Middle,
    /// This is the last element.
    Last,
    /// This is the only element.
    Only,
}

impl<I: Iterator> Iterator for WithPosition<I> {
    type Item = (Position, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        match self.peekable.next() {
            Some(item) => {
                if !self.handled_first {
                    // Haven't seen the first item yet, and there is one to give.
                    self.handled_first = true;
                    // Peek to see if this is also the last item,
                    // in which case tag it as `Only`.
                    match self.peekable.peek() {
                        Some(_) => Some((Position::First, item)),
                        None => Some((Position::Only, item)),
                    }
                } else {
                    // Have seen the first item, and there's something left.
                    // Peek to see if this is the last item.
                    match self.peekable.peek() {
                        Some(_) => Some((Position::Middle, item)),
                        None => Some((Position::Last, item)),
                    }
                }
            }
            // Iterator is finished.
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.peekable.size_hint()
    }
}

impl<I> ExactSizeIterator for WithPosition<I> where I: ExactSizeIterator {}

impl<I: Iterator> FusedIterator for WithPosition<I> {}
