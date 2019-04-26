/// Converts `(Vec<T>, Vec<U>)` to `Vec<(T, U)>`
pub fn zip<T, U>(a: Vec<T>, b: Vec<U>) -> Vec<(T, U)> {
    a.into_iter().zip(b.into_iter()).collect()
}

/// Converts `Vec<(T, U)>` to `(Vec<T>, Vec<U>)`
pub fn unzip<T, U>(v: Vec<(T, U)>) -> (Vec<T>, Vec<U>) {
    let mut a = vec![];
    let mut b = vec![];
    for (x, y) in v {
        a.push(x);
        b.push(y);
    }
    (a, b)
}

/// The same as the [`zip`](#function.zip) function but called as `Vec::zip`
pub trait VecZipTrait<T, U> {
    fn zip(self, other: Vec<U>) -> Vec<(T, U)>;
}

/// The same as the [`unzip`](#function.unzip) function but called as `Vec::unzip`
pub trait VecUnzipTrait<T, U> {
    fn unzip(self) -> (Vec<T>, Vec<U>);
}

impl<T, U> VecZipTrait<T, U> for Vec<T> {
    fn zip(self, other: Vec<U>) -> Vec<(T, U)> {
        zip(self, other)
    }
}

impl<T, U> VecUnzipTrait<T, U> for Vec<(T, U)> {
    fn unzip(self) -> (Vec<T>, Vec<U>) {
        unzip(self)
    }
}
