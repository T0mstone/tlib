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