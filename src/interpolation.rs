#[cfg(not(feature = "use_std"))]
use core as std;

use std::ops::{Add, Mul, Sub};

/// A trait used by the various interpolation functions as a trait bound for the `t` parameter
pub trait One {
    fn one() -> Self;
}

mod one_ext_seal {
    pub trait Seal {}

    impl<T: super::One> Seal for T {}
}

/// This trait is not indended to be implemented outside this crate itself
// sealed, so it cannot be implemented on its own
pub trait OneExt: One + one_ext_seal::Seal {
    fn n(n: usize) -> Self
    where
        Self: Add<Output = Self> + Sized,
    {
        let mut res = Self::one();
        for _ in 0..n - 1 {
            res = res + Self::one()
        }
        res
    }

    fn mul(self, n: usize) -> Self
    where
        Self: Add<Output = Self> + Clone,
    {
        let mut res = self.clone();
        for _ in 0..n - 1 {
            res = res + self.clone()
        }
        res
    }

    fn pow(&self, n: usize) -> Self
    where
        Self: Mul<Output = Self> + Clone,
    {
        (0..n).fold(Self::one(), |res, _| res * self.clone())
    }
}

#[inline]
fn one_minus<T: One + Sub<Output = T>>(t: T) -> T {
    T::one() - t
}

impl<T: One> OneExt for T {}

macro_rules! impl_one_trait {
    ($($t:ty),*) => {
        $(
            impl One for $t {
                fn one() -> Self {
                    1 as $t
                }
            }
        )*
    };
}

impl_one_trait! {
    u8, u16, u32, u64, usize,
    i8, i16, i32, i64, isize,
    f32, f64
}

/// Linear interpolation
#[inline]
pub fn lerp<P, T>(a: P, b: P, t: T) -> P
where
    P: Add<Output = P> + Mul<T, Output = P>,
    T: OneExt + Sub<Output = T> + Clone,
{
    a * one_minus(t.clone()) + b * t
}

/// Quadratic bézier interpolation
#[inline]
pub fn qbez<P, T>(a: P, b: P, c: P, t: T) -> P
where
    P: Add<Output = P> + Mul<T, Output = P>,
    T: OneExt + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Clone,
{
    let u = one_minus(t.clone());
    let t2 = t.pow(2);
    a * u.pow(2) + b * (T::n(2) * u * t) + c * t2
}

/// Cubic bézier interpolation
#[inline]
pub fn cubez<P, T>(a: P, b: P, c: P, d: P, t: T) -> P
where
    P: Add<Output = P> + Mul<T, Output = P>,
    T: OneExt + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Clone,
{
    let u = one_minus(t.clone());
    let (t2, t3) = (t.pow(2), t.pow(3));
    a * u.pow(3) + b * (T::n(3) * u.pow(2) * t) + c * (T::n(3) * u * t2) + d * t3
}

#[cfg(feature = "use_std")]
/// Computes the binomial coefficient (nCk)
fn choose(n: usize, k: usize) -> usize {
    let half = n / 2;
    // this also does the checking for k > n bc if k > n then k > half and (n -k) < 0 => subtract with overflow
    let k = if k > half { n - k } else { k };
    if n == 0 || n == 1 || n == k || k == 0 {
        1
    } else if n == 2 {
        [1, 2][k]
    } else if n == 3 {
        [1, 3][k]
    } else if n == 4 {
        [1, 4, 6][k]
    } else if n == 5 {
        [1, 5, 10][k]
    } else {
        choose(n - 1, k - 1) + choose(n - 1, k)
    }
}

#[cfg(feature = "use_std")]
/// Bézier interpolation of any degree (greater than 0)
pub fn bez<P, T>(mut pts: Vec<P>, t: T) -> Result<P, &'static str>
where
    P: Add<Output = P> + Mul<T, Output = P>,
    T: OneExt + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Clone,
{
    match pts.len() {
        0 => Err("no points given"),
        1 => Ok(pts.pop().unwrap()),
        2 => {
            let b = pts.pop().unwrap();
            let a = pts.pop().unwrap();
            Ok(lerp(a, b, t))
        }
        3 => {
            let c = pts.pop().unwrap();
            let b = pts.pop().unwrap();
            let a = pts.pop().unwrap();
            Ok(qbez(a, b, c, t))
        }
        4 => {
            let d = pts.pop().unwrap();
            let c = pts.pop().unwrap();
            let b = pts.pop().unwrap();
            let a = pts.pop().unwrap();
            Ok(cubez(a, b, c, d, t))
        }
        l => {
            let n = l - 1;
            let u = one_minus(t);

            let last = pts.pop().unwrap();
            let mut res = last * t.pow(n);

            let mut k = 0;
            for p in pts {
                let c = choose(n, k);
                let mul = OneExt::mul(u.pow(n - k) * t.pow(k), c);
                res = res + p * mul;
                k += 1;
            }
            Ok(res)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        #[inline]
        pub fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }

        #[inline]
        pub fn newi(x: i32, y: i32) -> Self {
            Self::new(x as f64, y as f64)
        }
    }

    impl Add for Point {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl Sub for Point {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self {
            Self {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    impl Mul<f64> for Point {
        type Output = Self;

        fn mul(self, rhs: f64) -> Self {
            Self {
                x: self.x * rhs,
                y: self.y * rhs,
            }
        }
    }

    #[test]
    fn test_lerp() {
        let a = Point::newi(5, 0);
        let b = Point::newi(17, -12);
        assert_eq!(Point::newi(13, -8), lerp(a, b, 2.0 / 3.0));
    }

    #[test]
    fn test_qbez() {
        let a = Point::newi(5, 0);
        let b = Point::newi(17, -12);
        let c = Point::newi(3, 0);
        assert_eq!(
            lerp(lerp(a, b, 0.75), lerp(b, c, 0.75), 0.75),
            qbez(a, b, c, 0.75)
        );
    }

    #[test]
    fn test_cubez() {
        let a = Point::newi(5, 0);
        let b = Point::newi(17, -12);
        let c = Point::newi(3, 0);
        let d = Point::newi(3, -100);
        assert_eq!(
            lerp(qbez(a, b, c, 0.2), qbez(b, c, d, 0.2), 0.2),
            cubez(a, b, c, d, 0.2)
        );
    }

    #[test]
    #[cfg(feature = "use_std")]
    fn test_nbez() {
        let a = Point::newi(5, 0);
        let b = Point::newi(17, -12);
        let c = Point::newi(3, 0);
        let d = Point::newi(3, -23);
        let e = Point::newi(4, 5);
        assert_eq!(Ok(lerp(a, b, 0.6)), bez(vec![a, b], 0.6));
        assert_eq!(Ok(qbez(a, b, c, 0.6)), bez(vec![a, b, c], 0.6));
        assert_eq!(Ok(cubez(a, b, c, d, 0.6)), bez(vec![a, b, c, d], 0.6));
        let r = bez(vec![a, b, c, d, e], 0.6);
        let ctrl = lerp(cubez(a, b, c, d, 0.6), cubez(b, c, d, e, 0.6), 0.6);
        // small rounding errors should not make the test fail
        assert!(r.is_ok() && (r.unwrap() - ctrl).less_than(0.00000001, 0.00000001));
    }
}
