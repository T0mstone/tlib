pub fn zip<T, U>(mut a: Vec<T>, mut b: Vec<U>) -> Vec<(T, U)> {
    let mut res = vec![];
    while let (Some(a), Some(b)) = (a.pop(), b.pop()) {
        res.push((a, b));
    }
    res.reverse();
    res
}

pub fn unzip<T, U>(v: Vec<(T, U)>) -> (Vec<T>, Vec<U>) {
    let mut a = vec![];
    let mut b = vec![];
    for (x, y) in v {
        a.push(x);
        b.push(y);
    }
    (a, b)
}

pub trait VecZipTrait<T, U> {
    fn zip(self, other: Vec<U>) -> Vec<(T, U)>;
}

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