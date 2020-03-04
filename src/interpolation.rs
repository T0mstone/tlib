#[cfg(not(feature = "use_std"))]
use core as std;

use num_traits::{One, Zero};

use std::ops::{Add, Mul, Sub};

#[inline]
fn one_minus<T: One + Sub<Output = T>>(t: T) -> T {
    T::one() - t
}

fn n<T: One + Add<Output = T>>(n: usize) -> T {
    assert_ne!(n, 0);
    (0..(n - 1)).fold(T::one(), |a, _| a + T::one())
}

fn mul_by_add<T: Zero + One + Add<Output = T> + Clone>(t: T, mut n: usize) -> T {
    if n == 0 {
        return T::zero();
    }

    let mut acc = T::zero();
    let mut res = t;

    while n > 1 {
        if n % 2 == 0 {
            res = res.clone() + res;
            n /= 2;
        } else {
            acc = res.clone() + acc;
            n -= 1;
        }
    }

    acc + res
}

fn pow<T: One + Mul<Output = T> + Clone>(t: T, mut n: usize) -> T {
    if n == 0 {
        return T::one();
    }

    let mut acc = T::one();
    let mut res = t;

    while n > 1 {
        if n % 2 == 0 {
            res = res.clone() * res;
            n /= 2;
        } else {
            acc = res.clone() * acc;
            n -= 1;
        }
    }

    acc * res
}

/// The core trait that provides the function to evaluate an interpolation object
pub trait Interpolation<T> {
    /// The type a point in the interpolation has
    type Point: Add<Output = Self::Point> + Mul<T, Output = Self::Point>;

    /// Get the point in the interpolation at value `t` (between `0` and `1`)
    fn eval(self, t: T) -> Self::Point;
}

/// A Linear interpolation (Lerp) struct
pub struct Linear<P> {
    start: P,
    end: P,
}

impl<P> Linear<P> {
    #[allow(missing_docs)]
    pub fn new(start: P, end: P) -> Self {
        Self { start, end }
    }
}

impl<T, P> Interpolation<T> for Linear<P>
where
    T: Clone + One + Sub<Output = T>,
    P: Add<Output = P> + Mul<T, Output = P>,
{
    type Point = P;

    fn eval(self, t: T) -> Self::Point {
        self.start * one_minus(t.clone()) + self.end * t
    }
}

/// A Quadratic Bézier interpolation struct
pub struct QuadraticBezier<P> {
    start: P,
    control: P,
    end: P,
}

impl<P> QuadraticBezier<P> {
    #[allow(missing_docs)]
    pub fn new(start: P, control: P, end: P) -> Self {
        Self {
            start,
            control,
            end,
        }
    }
}

impl<T, P> Interpolation<T> for QuadraticBezier<P>
where
    T: Clone + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    P: Add<Output = P> + Mul<T, Output = P>,
{
    type Point = P;

    fn eval(self, t: T) -> Self::Point {
        let u = one_minus(t.clone());
        let t2 = pow(t.clone(), 2);
        self.start * pow(u.clone(), 2) + self.control * (n::<T>(2) * u * t) + self.end * t2
    }
}

/// A Cubic Bézier interpolation struct
pub struct CubicBezier<P> {
    start: P,
    control1: P,
    control2: P,
    end: P,
}

impl<P> CubicBezier<P> {
    #[allow(missing_docs)]
    pub fn new(start: P, control1: P, control2: P, end: P) -> Self {
        Self {
            start,
            control1,
            control2,
            end,
        }
    }
}

impl<T, P> Interpolation<T> for CubicBezier<P>
where
    T: Clone + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    P: Add<Output = P> + Mul<T, Output = P>,
{
    type Point = P;

    fn eval(self, t: T) -> Self::Point {
        let u = one_minus(t.clone());
        let (t2, t3) = (pow(t.clone(), 2), pow(t.clone(), 3));
        self.start * pow(u.clone(), 3)
            + self.control1 * (n::<T>(3) * pow(u.clone(), 2) * t)
            + self.control2 * (n::<T>(3) * u * t2)
            + self.end * t3
    }
}

/// linear interpolation: a shortcut for quick usage
#[inline]
pub fn lerp<T, P>(start: P, end: P, t: T) -> P
where
    T: Clone + One + Sub<Output = T>,
    P: Add<Output = P> + Mul<T, Output = P>,
{
    Linear::new(start, end).eval(t)
}

/// quadratic bézier interpolation: a shortcut for quick usage
#[inline]
pub fn bez2<T, P>(start: P, control: P, end: P, t: T) -> P
where
    T: Clone + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    P: Add<Output = P> + Mul<T, Output = P>,
{
    QuadraticBezier::new(start, control, end).eval(t)
}

/// quadratic bézier interpolation: a shortcut for quick usage (identical to [`bez2`](#function.bez2)
#[inline]
pub fn qbez<T, P>(start: P, control: P, end: P, t: T) -> P
where
    T: Clone + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    P: Add<Output = P> + Mul<T, Output = P>,
{
    bez2(start, control, end, t)
}

/// cubic bézier interpolation: a shortcut for quick usage
#[inline]
pub fn bez3<T, P>(start: P, control1: P, control2: P, end: P, t: T) -> P
where
    T: Clone + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    P: Add<Output = P> + Mul<T, Output = P>,
{
    CubicBezier::new(start, control1, control2, end).eval(t)
}

/// cubic bézier interpolation: a shortcut for quick usage (identical to [`bez3`](#function.bez3)
#[inline]
pub fn cbez<T, P>(start: P, control1: P, control2: P, end: P, t: T) -> P
where
    T: Clone + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    P: Add<Output = P> + Mul<T, Output = P>,
{
    bez3(start, control1, control2, end, t)
}

/// Computes the binomial coefficient (nCk)
fn binomial(n: usize, k: usize) -> usize {
    let half = n / 2;
    // this also panicks (desired behaviour) for k > n
    // because if k > n then k > half and (n - k) < 0 => subtract with overflow
    let k = if k > half { n - k } else { k };

    match n {
        0 | 1 => 1,
        _ if k == 0 => 1,
        2 => [1, 2][k],
        3 => [1, 3][k],
        4 => [1, 4, 6][k],
        5 => [1, 5, 10][k],
        _ => binomial(n - 1, k - 1) + binomial(n - 1, k),
    }
}

/// True arbitrary-order bézier interpolation
pub struct Bezier<'a, P> {
    start: P,
    control_points: &'a [P],
    end: P,
}

impl<'a, P> Bezier<'a, P> {
    #[allow(missing_docs)]
    pub fn new(start: P, control_points: &'a [P], end: P) -> Self {
        Self {
            start,
            control_points,
            end,
        }
    }
}

impl<'a, T, P> Interpolation<T> for Bezier<'a, P>
where
    T: Clone + Zero + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    P: Clone + Add<Output = P> + Mul<T, Output = P>,
{
    type Point = P;

    fn eval(self, t: T) -> Self::Point {
        match self.control_points {
            [] => lerp(self.start, self.end, t),
            [c] => bez2(self.start, c.clone(), self.end, t),
            [c1, c2] => bez3(self.start, c1.clone(), c2.clone(), self.end, t),
            c => {
                let n = c.len() + 1;
                let u = one_minus(t.clone());

                let mut res = self.end * pow(t.clone(), n);

                for (k, p) in std::iter::once(self.start)
                    .chain(c.iter().cloned())
                    .enumerate()
                {
                    let c = binomial(n, k);
                    let mul = mul_by_add(pow(u.clone(), n - k) * pow(t.clone(), k), c);
                    res = res + p * mul;
                }
                res
            }
        }
    }
}

/// arbitrary-order bézier interpolation: a shortcut for quick usage
#[inline]
pub fn bez<T, P>(start: P, control_points: &[P], end: P, t: T) -> P
where
    T: Clone + Zero + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    P: Clone + Add<Output = P> + Mul<T, Output = P>,
{
    Bezier::new(start, control_points, end).eval(t)
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

        pub fn less_than(&self, x: f64, y: f64) -> bool {
            self.x < x && self.y < y
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
            cbez(a, b, c, d, 0.2)
        );
    }

    #[test]
    fn test_nbez() {
        let a = Point::newi(5, 0);
        let b = Point::newi(17, -12);
        let c = Point::newi(3, 0);
        let d = Point::newi(3, -23);
        let e = Point::newi(4, 5);
        assert_eq!(lerp(a, b, 0.6), bez(a, &[], b, 0.6));
        assert_eq!(qbez(a, b, c, 0.6), bez(a, &[b], c, 0.6));
        assert_eq!(cbez(a, b, c, d, 0.6), bez(a, &[b, c], d, 0.6));
        let r = bez(a, &[b, c, d], e, 0.6);
        let ctrl = lerp(cbez(a, b, c, d, 0.6), cbez(b, c, d, e, 0.6), 0.6);
        // small rounding errors should not make the test fail
        assert!((r - ctrl).less_than(0.00000001, 0.00000001));
    }
}
