#[cfg(feature = "no_std")]
use core as std;

use std::ops::{Add, Mul};

/// Linear interpolation
#[inline]
pub fn lerp<T: Add<Output = T> + Mul<f64, Output = T>>(a: T, b: T, t: f64) -> T {
    a * (1.0 - t) + b * t
}

/// Quadratic bézier interpolation
#[inline]
pub fn qbez<T: Add<Output = T> + Mul<f64, Output = T>>(a: T, b: T, c: T, t: f64) -> T {
    let u = 1.0 - t;
    a * (u * u) + b * (2.0 * u * t) + c * (t * t)
}

/// Cubic bézier interpolation
#[inline]
#[rustfmt::skip]
pub fn cubez<T: Add<Output = T> + Mul<f64, Output = T>>(a: T, b: T, c: T, d: T, t: f64) -> T {
    let u = 1.0 - t;
    a * (u * u * u)
        + b * (3.0 * u * u * t)
        + c * (3.0 * u * t * t)
        + d * (t * t * t)
}

/// Computes the binomial coefficient (nCk)
fn choose(n: usize, k: usize) -> usize {
    if k > n {
        panic!("Called `choose(n, k)` but k was greater than n");
    }
    let half = n / 2;
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

/// Bézier interpolation of any degree (greater than 0)
pub fn bez<T: Add<Output = T> + Mul<f64, Output = T>>(
    mut pts: Vec<T>,
    t: f64,
) -> Result<T, &'static str> {
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
            let u = 1.0 - t;

            let last = pts.pop().unwrap();
            let mut res = last * t.powi(n as i32);

            let mut k = 0;
            for p in pts {
                let mul = choose(n, k) as f64 * u.powi((n - k) as i32) * t.powi(k as i32);
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
    use std::ops::Sub;

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

        pub(self) fn less_than(&self, x: f64, y: f64) -> bool {
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
            cubez(a, b, c, d, 0.2)
        );
    }

    #[test]
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
