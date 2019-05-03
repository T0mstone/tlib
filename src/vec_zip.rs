#![allow(unused_macros)]
/// Converts `(Vec<T>, Vec<U>)` to `Vec<(T, U)>`
pub fn zip<T, U>(a: Vec<T>, b: Vec<U>) -> Vec<(T, U)> {
    a.into_iter().zip(b.into_iter()).collect()
}

/// Converts `Vec<(T, U)>` to `(Vec<T>, Vec<U>)`
pub fn unzip<T, U>(v: Vec<(T, U)>) -> (Vec<T>, Vec<U>) {
    let mut a = Vec::new();
    let mut b = Vec::new();
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

// Some helper macros to make writing the zip functions way less painful
macro_rules! __flatten1 {
    ($first:ident; $($bind:ident),*) => {
        |($first, ($($bind),*))| ($first, $($bind),*)
    };
}

macro_rules! flatten_tuple_fn {
    ($a:ident, $b:ident, $c:ident) => {
        __flatten1!($a; $b, $c)
    };
    ($first:ident $(,$bind:ident)*) => {
        |(x, tup)| {
            let f1 = flatten_tuple_fn!($($bind),*);
            let flat = f1(tup);
            let f2 = __flatten1!($first; $($bind),*);
            f2((x, flat))
        }
    }
}

macro_rules! zip_rec {
    ($a:ident, $b:ident) => {
        $a.into_iter().zip($b.into_iter())
    };
    ($head:ident $(, $tail:ident)*) => {
        $head.into_iter().zip(zip_rec!($($tail),*))
    };
}

macro_rules! gen_zip_fn {
    ($fname:ident => $($binding:ident @ $param:ident: $t:ident),*) => {
        pub fn $fname<$($t),*>($($param: Vec<$t>),*) -> Vec<($($t),*)> {
            zip_rec!($($param),*).map(flatten_tuple_fn!($($binding),*)).collect()
        }
    };
}

macro_rules! gen_zip_fns_accum {
    ($fname:ident => $($binding:ident @ $param:ident: $t:ident),*; $($fname2:ident => $($binding2:ident @ $param2:ident: $t2:ident),*;)*) => {
        gen_zip_fns_accum!([] $fname => $($binding @ $param: $t),*; $($fname2 => $($binding2 @ $param2: $t2),*;)*);
    };
    ([$($binding1:ident @ $param1:ident: $t1:ident,)*] $fname:ident => $($binding:ident @ $param:ident: $t:ident),*;) => {
        gen_zip_fn!($fname => $($binding1 @ $param1: $t1,)* $($binding @ $param: $t),*);
    };
    ([$($binding1:ident @ $param1:ident: $t1:ident,)*] $fname:ident => $($binding:ident @ $param:ident: $t:ident),*; $($fname2:ident => $($binding2:ident @ $param2:ident: $t2:ident),*;)*) => {
        gen_zip_fn!($fname => $($binding1 @ $param1: $t1,)* $($binding @ $param: $t),*);
        gen_zip_fns_accum!([$($binding1 @ $param1: $t1,)* $($binding @ $param: $t,)*] $($fname2 => $($binding2 @ $param2: $t2),*;)*);
    };
}

// TODO: unzip functions up to 10 + add traits for these

gen_zip_fns_accum! {
    zip3 => t1 @ a: A, t2 @ b: B, t3 @ c: C;
    zip4 => t4 @ d: D;
    zip5 => t5 @ e: E;
    zip6 => t6 @ f: F;
    zip7 => t7 @ g: G;
    zip8 => t8 @ h: H;
    zip9 => t9 @ i: I;
    zip10 => t10 @ j: J;
}

#[cfg(test)]
mod test {
    use super::zip10;

    macro_rules! vecs {
        ($($a:literal, $b:literal),*) => {
            (
                $(
                    vec![$a, $b]
                ),*
            )
        };
    }

    #[test]
    fn test_zip10() {
        let (a, b, c, d, e, f, g, h, i, j) =
            vecs![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19];

        let v = zip10(a, b, c, d, e, f, g, h, i, j);

        assert_eq!(
            vec![
                (0, 2, 4, 6, 8, 10, 12, 14, 16, 18),
                (1, 3, 5, 7, 9, 11, 13, 15, 17, 19)
            ],
            v
        );
    }
}
