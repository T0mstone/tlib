/// Generates a new `Vec` out of several `Vec`s, where each new element is a tuple of the previous `Vec`s' elements at that index
pub fn zip<A, B>(a: Vec<A>, b: Vec<B>) -> Vec<(A, B)> {
    a.into_iter().zip(b.into_iter()).collect()
}

/// Generates several `Vec`s out of a `Vec` (where each element is a tuple), splitting the tuple elements all into single types
pub fn unzip<A, B>(v: Vec<(A, B)>) -> (Vec<A>, Vec<B>) {
    v.into_iter().unzip()
}

/// The same as the [`zip`](#function.zip) function but called as `Vec::zip`
pub trait VecZipTrait<T, U> {
    #[allow(missing_docs)]
    fn zip(self, other: Vec<U>) -> Vec<(T, U)>;
}

/// The same as the [`unzip`](#function.unzip) function but called as `Vec::unzip`
pub trait VecUnzipTrait<T, U> {
    #[allow(missing_docs)]
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

// note: maybe consider adding traits for the other numbers?

// Macros for generating the zip functions
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
        /// Like [`zip`](#function.zip) but with more `Vec`s
        pub fn $fname<$($t),*>($($param: Vec<$t>),*) -> Vec<($($t),*)> {
            zip_rec!($($param),*).map(flatten_tuple_fn!($($binding),*)).collect()
        }
    };
}

// Macro for generating the unzip functions

macro_rules! gen_unzip_fn {
    ($fname:ident => $($binding:ident @ $vname:ident: $t:ident),*) => {
        /// Like [`unzip`](#function.unzip) but with more `Vec`s
        pub fn $fname<$($t),*>(v: Vec<($($t),*)>) -> ($(Vec<$t>),*) {
            $(
                let mut $vname = Vec::new();
            )*
            for ($($binding),*) in v {
                $(
                    $vname.push($binding);
                )*
            }
            ($($vname),*)
        }
    };
}

// Unified macro to generate zip AND unzip functions

macro_rules! gen_zip_and_unzip_fns_accum {
    ($fname:ident, $ufname:ident => $($binding:ident @ $extra:ident: $t:ident),*; $($fname2:ident, $ufname2:ident => $($binding2:ident @ $extra2:ident: $t2:ident),*;)*) => {
        gen_zip_and_unzip_fns_accum!([] $fname, $ufname => $($binding @ $extra: $t),*; $($fname2, $ufname2 => $($binding2 @ $extra2: $t2),*;)*);
    };
    ([$($binding1:ident @ $extra1:ident: $t1:ident,)*] $fname:ident, $ufname:ident => $($binding:ident @ $extra:ident: $t:ident),*;) => {
        gen_zip_fn!($fname => $($binding1 @ $extra1: $t1,)* $($binding @ $extra: $t),*);
        gen_unzip_fn!($ufname => $($binding1 @ $extra1: $t1,)* $($binding @ $extra: $t),*);
    };
    ([$($binding1:ident @ $extra1:ident: $t1:ident,)*] $fname:ident, $ufname:ident => $($binding:ident @ $extra:ident: $t:ident),*; $($fname2:ident, $ufname2:ident => $($binding2:ident @ $extra2:ident: $t2:ident),*;)*) => {
        gen_zip_fn!($fname => $($binding1 @ $extra1: $t1,)* $($binding @ $extra: $t),*);
        gen_unzip_fn!($ufname => $($binding1 @ $extra1: $t1,)* $($binding @ $extra: $t),*);
        gen_zip_and_unzip_fns_accum!([$($binding1 @ $extra1: $t1,)* $($binding @ $extra: $t,)*] $($fname2, $ufname2 => $($binding2 @ $extra2: $t2),*;)*);
    };
}

// Generated functions

gen_zip_and_unzip_fns_accum! {
    zip3, unzip3 => t1 @ a: A, t2 @ b: B, t3 @ c: C;
    zip4, unzip4 => t4 @ d: D;
    zip5, unzip5 => t5 @ e: E;
    zip6, unzip6 => t6 @ f: F;
    zip7, unzip7 => t7 @ g: G;
    zip8, unzip8 => t8 @ h: H;
    zip9, unzip9 => t9 @ i: I;
    zip10, unzip10 => t10 @ j: J;
}

#[cfg(test)]
mod test {
    use super::{unzip10, zip10};

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

    #[test]
    fn test_unzip10() {
        let (a, b, c, d, e, f, g, h, i, j) =
            vecs![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19];

        let v = zip10(a, b, c, d, e, f, g, h, i, j);

        assert_eq!(
            vecs![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
            unzip10(v)
        )
    }
}
