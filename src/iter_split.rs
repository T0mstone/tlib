use crate::auto_escape::{
    indicator, indicator_not_escaped, unescape_all_except, AutoEscape, Unescape,
};
use std::iter::{once, FromIterator};
use std::marker::PhantomData;

/// An iterator for splitting another iterator by single items
pub struct SplitIter<I: Iterator, F, V> {
    // (dyn) the number of split segments already returned
    // starts at 0
    curr_len: usize,
    max_len: Option<usize>,
    iter: I,
    is_sep: F,
    // (setting) whether to emit the separator into the stream
    keep_sep: bool,
    // (dyn) the separator, if it was kept from the last call to `next`
    // starts at None
    last_sep: Option<I::Item>,
    // (dyn) this is set when no more `Some`s should be returned
    // starts at false
    done: bool,
    _marker: PhantomData<V>,
}

impl<I: Iterator, F: FnMut(&I::Item) -> bool, V: FromIterator<I::Item>> Iterator
    for SplitIter<I, F, V>
{
    type Item = V;

    fn next(&mut self) -> Option<V> {
        if self.done {
            return None;
        }
        if let Some(sep) = self.last_sep.take() {
            return Some(once(sep).collect());
        }
        self.curr_len += 1;
        if self.max_len.map_or(false, |len| self.curr_len == len) {
            let v = self.iter.by_ref().collect();
            self.done = true;
            // the length limit is reached: return the whole rest
            return Some(v);
        }
        let mut res = Vec::new();
        while let Some(x) = self.iter.next() {
            if (self.is_sep)(&x) {
                if self.keep_sep {
                    self.last_sep = Some(x);
                }
                // return here, without setting `done`
                // -> after a sep, there has to be another element (even if empty)
                return Some(res.into_iter().collect());
            } else {
                res.push(x);
            }
        }
        // reached the end
        self.done = true;
        Some(res.into_iter().collect())
    }
}

/// A trait for splitting another iterator by single items
pub trait IterSplit: Sized + IntoIterator + crate::into_iter_seal::IntoIterSeal {
    #[allow(missing_docs)]
    fn split_impl<F: FnMut(&Self::Item) -> bool, V: FromIterator<Self::Item>>(
        self,
        max_len: Option<usize>,
        is_sep: F,
        keep_sep: bool,
    ) -> SplitIter<Self::IntoIter, F, V>;

    /// Splits an iterator into smaller chunks.
    /// The separators are the items where `is_sep` returns `true`
    ///
    /// If `keep_sep` is `true`, the separators will also be emitted (on their own)
    fn split<F: FnMut(&Self::Item) -> bool, V: FromIterator<Self::Item>>(
        self,
        is_sep: F,
        keep_sep: bool,
    ) -> SplitIter<Self::IntoIter, F, V> {
        self.split_impl(None, is_sep, keep_sep)
    }

    /// Splits an iterator into smaller chunks,
    /// **stopping once it reaches the specified (`n`) number of chunks**.
    /// The separators are the items where `is_sep` returns `true`
    ///
    /// If `keep_sep` is `true`, the separators will also be emitted (on their own)
    fn splitn<F: FnMut(&Self::Item) -> bool, V: FromIterator<Self::Item>>(
        self,
        n: usize,
        is_sep: F,
        keep_sep: bool,
    ) -> SplitIter<Self::IntoIter, F, V> {
        self.split_impl(Some(n), is_sep, keep_sep)
    }
}

impl<I: IntoIterator> IterSplit for I {
    fn split_impl<F: FnMut(&Self::Item) -> bool, V: FromIterator<Self::Item>>(
        self,
        max_len: Option<usize>,
        is_sep: F,
        keep_sep: bool,
    ) -> SplitIter<Self::IntoIter, F, V> {
        SplitIter {
            curr_len: 0,
            max_len,
            iter: self.into_iter(),
            is_sep,
            keep_sep,
            last_sep: None,
            done: false,
            _marker: PhantomData,
        }
    }
}

/// A shortcut trait for the common operation of splitting a string
/// according to a single delimeter, respecting escaping
pub trait SplitNotEscapedString {
    #[allow(missing_docs)]
    fn split_not_escaped_impl<V: FromIterator<String>>(
        &self,
        max_len: Option<usize>,
        sep: char,
        esc: char,
        keep_sep: bool,
    ) -> V;

    /// Analogous to [`IterSplit::split`](trait.IterSplit#method.split)
    ///
    /// Splits with `sep`, escapes with `esc`
    #[inline]
    fn split_not_escaped<V: FromIterator<String>>(
        &self,
        sep: char,
        esc: char,
        keep_sep: bool,
    ) -> V {
        self.split_not_escaped_impl(None, sep, esc, keep_sep)
    }

    /// Analogous to [`IterSplit::splitn`](trait.IterSplit#method.splitn)
    ///
    /// Splits with `sep`, escapes with `esc`
    #[inline]
    fn splitn_not_escaped<V: FromIterator<String>>(
        &self,
        n: usize,
        sep: char,
        esc: char,
        keep_sep: bool,
    ) -> V {
        self.split_not_escaped_impl(Some(n), sep, esc, keep_sep)
    }
}

impl<S: AsRef<str>> SplitNotEscapedString for S {
    fn split_not_escaped_impl<V: FromIterator<String>>(
        &self,
        max_len: Option<usize>,
        sep: char,
        esc: char,
        keep_sep: bool,
    ) -> V {
        self.as_ref()
            .chars()
            .auto_escape(indicator(esc))
            .split_impl::<_, Vec<_>>(max_len, indicator_not_escaped(sep), keep_sep)
            .map(|v| {
                v.into_iter()
                    .unescape(unescape_all_except(sep, esc))
                    .collect::<String>()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_string() {
        let s = "abcd";
        assert_eq!(
            s.split_not_escaped::<Vec<_>>(':', '\\', false),
            vec!["abcd".to_string()]
        );
        let s = "abcd:efg:h";
        assert_eq!(
            s.split_not_escaped::<Vec<_>>(':', '\\', false),
            vec!["abcd".to_string(), "efg".to_string(), "h".to_string()]
        );
        let s = "abc\\:d:efg:h";
        assert_eq!(
            s.split_not_escaped::<Vec<_>>(':', '\\', false),
            vec!["abc:d".to_string(), "efg".to_string(), "h".to_string()]
        );
        let s = "";
        assert_eq!(
            s.split_not_escaped::<Vec<_>>(':', '\\', false),
            vec![String::new()]
        );
    }

    #[test]
    fn splitn_string() {
        let s = "abcd";
        assert_eq!(
            s.splitn_not_escaped::<Vec<_>>(3, ':', '\\', false),
            vec!["abcd".to_string()]
        );
        let s = "abcd:efg:h";
        assert_eq!(
            s.splitn_not_escaped::<Vec<_>>(2, ':', '\\', false),
            vec!["abcd".to_string(), "efg:h".to_string()]
        );
        let s = "abc\\:d:efg:h";
        assert_eq!(
            s.splitn_not_escaped::<Vec<_>>(2, ':', '\\', false),
            vec!["abc:d".to_string(), "efg:h".to_string()]
        );
        let s = "";
        assert_eq!(
            s.splitn_not_escaped::<Vec<_>>(3, ':', '\\', false),
            vec![String::new()]
        );
    }
}
