#[cfg(feature = "use_std")]
pub use self::unescape_iter::*;
#[cfg(not(feature = "use_std"))]
use core as std;
use std::iter::{once, Once};

/// Automatically converts items into an easy-to-work-with representation
///
/// Example:
/// When using `indicator('\\')` as `is_esc`,
/// this will convert `['a', 'b', '\\', 'c']` to `[(false, 'a'), (false, 'b'), (true, 'c')]`,
/// `['\\', '\\']` to `[(true, '\\')]` and `['\\']` to `[(false, '\\')]`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct AutoEscapeIter<I, F> {
    iter: I,
    is_esc: F,
}

impl<I: Iterator, F: FnMut(&I::Item) -> bool> Iterator for AutoEscapeIter<I, F> {
    type Item = (bool, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let nx = self.iter.next()?;
        if (self.is_esc)(&nx) {
            match self.iter.next() {
                Some(t) => Some((true, t)),
                None => Some((false, nx)),
            }
        } else {
            Some((false, nx))
        }
    }
}

/// Trait for creating an [`AutoEscapeIter`](struct.AutoEscapeIter.html)
pub trait AutoEscape: Sized + IntoIterator + crate::into_iter_seal::IntoIterSeal {
    /// Creates an `AutoEscapeIter` which uses `is_esc` to test if an item counts as escaping
    fn auto_escape<F: FnMut(&Self::Item) -> bool>(
        self,
        is_esc: F,
    ) -> AutoEscapeIter<Self::IntoIter, F>;
}

impl<I: IntoIterator> AutoEscape for I {
    #[inline]
    fn auto_escape<F: FnMut(&Self::Item) -> bool>(
        self,
        is_esc: F,
    ) -> AutoEscapeIter<Self::IntoIter, F> {
        AutoEscapeIter {
            iter: self.into_iter(),
            is_esc,
        }
    }
}

#[cfg(feature = "use_std")]
mod unescape_iter {
    use std::collections::VecDeque;
    use std::iter::Map;

    /// Reverses escaping done by [`AutoEscapeIter`](struct.AutoEscapeIter.html)
    ///
    /// An example:
    /// Equipped with `unescape_all('\\')` as `escape_item`, this converts
    /// `[(false, 'a'), (false, 'b'), (true, 'c')]` to `['a', 'b', '\\', 'c']`.
    pub struct UnescapeIter<T, I, F> {
        iter: I,
        escape_item: F,
        queue: VecDeque<T>,
    }

    impl<T, I: Iterator<Item = (bool, T)>, J: IntoIterator<Item = T>, F: FnMut(&T) -> J> Iterator
        for UnescapeIter<T, I, F>
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(x) = self.queue.pop_front() {
                return Some(x);
            }

            let (esc, t) = self.iter.next()?;
            if esc {
                let iter = (self.escape_item)(&t)
                    .into_iter()
                    .chain(core::iter::once(t));
                self.queue.extend(iter);
                // at least the chained `once` will always produce one value
                Some(self.queue.pop_front().unwrap())
            } else {
                Some(t)
            }
        }
    }

    #[allow(missing_docs)]
    pub type UnescapeIgnoreIter<T, I> = Map<I, fn((bool, T)) -> T>;

    /// Trait for creating an [`UnescapeIter`](struct.UnescapeIter.html)
    pub trait Unescape: Sized + IntoIterator + crate::into_iter_seal::IntoIterSeal {
        /// The item inside the escaped-tuple
        type UnescapedItem;

        /// Creates an Iterator which completely discards all escapedness information
        fn unescape_ignore(self) -> UnescapeIgnoreIter<Self::UnescapedItem, Self::IntoIter>;

        /// Creates an `UnescapeIter` which uses `escape_item` to generate escape items
        /// - `escape_item` is given the current item to decide what to unescape it as
        fn unescape<
            I: IntoIterator<Item = Self::UnescapedItem>,
            F: FnMut(&Self::UnescapedItem) -> I,
        >(
            self,
            escape_item: F,
        ) -> UnescapeIter<Self::UnescapedItem, Self::IntoIter, F>;
    }

    impl<T, I: IntoIterator<Item = (bool, T)>> Unescape for I {
        type UnescapedItem = T;

        fn unescape_ignore(self) -> UnescapeIgnoreIter<Self::UnescapedItem, Self::IntoIter> {
            self.into_iter().map(|t| t.1)
        }

        fn unescape<
            J: IntoIterator<Item = Self::UnescapedItem>,
            F: FnMut(&Self::UnescapedItem) -> J,
        >(
            self,
            escape_item: F,
        ) -> UnescapeIter<Self::UnescapedItem, Self::IntoIter, F> {
            UnescapeIter {
                iter: self.into_iter(),
                escape_item,
                queue: VecDeque::new(),
            }
        }
    }
}

/// Creates a function that returns `true` when its argument is equal to `x`
#[inline]
pub fn indicator<T: PartialEq>(x: T) -> impl FnMut(&T) -> bool {
    move |t| *t == x
}

/// Creates a function that returns `true` when its argument is equal to `(false, x)`
#[inline]
pub fn indicator_not_escaped<T: PartialEq>(x: T) -> impl FnMut(&(bool, T)) -> bool {
    move |(esc, t)| !*esc && *t == x
}

/// Creates a function that always returns `esc.clone()`
#[inline]
pub fn unescape_all<T, U: Clone>(esc: U) -> impl FnMut(&T) -> Once<U> {
    move |_| once(esc.clone())
}

/// Creates a function that always returns `esc.clone()` except for when its argument is equal to `t`
#[inline]
pub fn unescape_all_except<T: PartialEq, U: Clone>(t: T, esc: U) -> impl FnMut(&T) -> Option<U> {
    move |x| if x == &t { None } else { Some(esc.clone()) }
}
