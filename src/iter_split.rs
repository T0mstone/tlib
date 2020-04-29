/// An iterator for splitting another iterator by single items
pub struct SplitIter<I: Iterator, F> {
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
}

impl<I: Iterator, F: FnMut(&I::Item) -> bool> Iterator for SplitIter<I, F> {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        if let Some(sep) = self.last_sep.take() {
            return Some(vec![sep]);
        }
        self.curr_len += 1;
        if self.max_len.map_or(false, |len| self.curr_len == len) {
            // the length limit is reached: return the whole rest
            return Some(self.iter.by_ref().collect());
        }
        let mut res: Option<Vec<I::Item>> = None;
        while let Some(x) = self.iter.next() {
            if res.is_none() {
                res = Some(Vec::new());
            }
            if (self.is_sep)(&x) {
                if self.keep_sep {
                    self.last_sep = Some(x);
                }
                break;
            } else {
                match res.as_mut() {
                    Some(v) => v.push(x),
                    None => res = Some(vec![x]),
                }
            }
        }
        res
    }
}

/// A trait for splitting another iterator by single items
pub trait IterSplit: Sized + IntoIterator + crate::into_iter_seal::IntoIterSeal {
    #[allow(missing_docs)]
    fn split_impl<F: FnMut(&Self::Item) -> bool>(
        self,
        max_len: Option<usize>,
        is_sep: F,
        keep_sep: bool,
    ) -> SplitIter<Self::IntoIter, F>;

    /// Splits an iterator into smaller chunks.
    /// The separators are the items where `is_sep` returns `true`
    ///
    /// If `keep_sep` is `true`, the separators will also be emitted (on their own)
    fn split<F: FnMut(&Self::Item) -> bool>(
        self,
        is_sep: F,
        keep_sep: bool,
    ) -> SplitIter<Self::IntoIter, F> {
        self.split_impl(None, is_sep, keep_sep)
    }

    /// Splits an iterator into smaller chunks,
    /// **stopping once it reaches the specified (`n`) number of chunks**.
    /// The separators are the items where `is_sep` returns `true`
    ///
    /// If `keep_sep` is `true`, the separators will also be emitted (on their own)
    fn splitn<F: FnMut(&Self::Item) -> bool>(
        self,
        n: usize,
        is_sep: F,
        keep_sep: bool,
    ) -> SplitIter<Self::IntoIter, F> {
        self.split_impl(Some(n), is_sep, keep_sep)
    }
}

impl<I: IntoIterator> IterSplit for I {
    fn split_impl<F: FnMut(&Self::Item) -> bool>(
        self,
        max_len: Option<usize>,
        is_sep: F,
        keep_sep: bool,
    ) -> SplitIter<Self::IntoIter, F> {
        SplitIter {
            curr_len: 0,
            max_len,
            iter: self.into_iter(),
            is_sep,
            keep_sep,
            last_sep: None,
        }
    }
}
