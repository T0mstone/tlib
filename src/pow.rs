use num_traits::float::FloatCore;
use num_traits::{Inv, One, Zero};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[allow(missing_docs)]
/// An integer (not requiring any of `Sub`, `Div`, `Rem`)
pub trait Integer: Ord + One + Zero {
    fn is_positive(&self) -> bool {
        self > &Self::zero()
    }

    fn is_negative(&self) -> bool {
        self < &Self::zero()
    }

    fn two() -> Self {
        Self::one() + Self::one()
    }

    fn is_even(&self) -> bool;

    fn is_odd(&self) -> bool {
        !self.is_even()
    }
}

#[allow(missing_docs)]
pub trait Abs {
    type Output;

    fn abs(&self) -> Self::Output;
}

/// A dummy type for setting and getting precision
pub enum PowPrecision {}

impl PowPrecision {
    fn inner() -> &'static mut HashMap<TypeId, Box<dyn Any>> {
        static mut MAP: Option<HashMap<TypeId, Box<dyn Any>>> = None;
        unsafe {
            if MAP.is_none() {
                MAP = Some(HashMap::new());
            }
            MAP.as_mut().unwrap()
        }
    }

    /// Set the precision for a type
    pub fn set<T: 'static>(t: T) -> Option<T> {
        let map = Self::inner();
        let res = map.remove(&TypeId::of::<T>()).map(|b| {
            *b.downcast::<T>()
                .unwrap_or_else(|_| unreachable!("Internal Error: malformed Precision entry"))
        });
        map.insert(TypeId::of::<T>(), Box::new(t) as Box<dyn Any>);
        res
    }

    /// Get the precision for a type
    pub fn get<T: 'static>() -> Option<&'static T> {
        Self::inner().get(&TypeId::of::<T>()).map(|b| {
            b.downcast_ref::<T>()
                .unwrap_or_else(|| unreachable!("Internal Error: malformed Precision entry"))
        })
    }

    /// Get the precision for a type (mutable)
    pub fn get_mut<T: 'static>() -> Option<&'static mut T> {
        Self::inner().get_mut(&TypeId::of::<T>()).map(|b| {
            b.downcast_mut::<T>()
                .unwrap_or_else(|| unreachable!("Internal Error: malformed Precision entry"))
        })
    }
}

/// Exponentioation with integer powers, using the exponentiation by squaring algorithm
pub fn powi<T, E>(base: T, mut exp: E) -> T
where
    T: One + Inv<Output = T> + Clone + Mul<Output = T>,
    E: Integer + Neg<Output = E> + Div<Output = E> + Sub<Output = E>,
{
    if exp.is_zero() {
        return T::one();
    } else if exp.is_negative() {
        return powi(base, -exp).inv();
    } else if exp.is_one() {
        return base;
    }

    let mut fac = T::one();
    let mut res = base;
    while exp > E::one() {
        if exp.is_even() {
            res = res.clone() * res;
            exp = exp / E::two();
        } else {
            fac = fac * res.clone();
            exp = exp - E::one();
        }
    }
    fac * res
}

#[allow(missing_docs)]
pub enum PowError {
    /// The `root` and `pow` functions require a set precision (for the type of the first argument)
    UnsetPrecision,
    /// An even root is only defined for positive numbers
    EvenRootOfNegativeNumber,
}

#[allow(missing_docs)]
pub enum RootError {
    PowError(PowError),
    /// A zeroth root cannot exist (that would be the exponent 1/0)
    ZerothRoot,
    /// A negative root is not supported
    NegativethRoot,
}

/// An unsigned integer root of a number, using the [algorithm from Wikipedia](https://en.wikipedia.org/wiki/Nth_root_algorithm)
/// with an initial guess of one
pub fn root<T, E>(x: T, n: E) -> Result<T, RootError>
where
    T: 'static
        + Zero
        + One
        + Ord
        + Clone
        + From<E>
        + Add<Output = T>
        + Sub<Output = T>
        + Inv<Output = T>
        + Div<Output = T>
        + Abs<Output = T>,
    E: Integer + Sub<Output = E> + Clone + Neg<Output = E> + Div<Output = E>,
{
    if x < T::zero() && n.is_even() {
        return Err(RootError::PowError(PowError::EvenRootOfNegativeNumber));
    } else if n.is_zero() {
        return Err(RootError::ZerothRoot);
    } else if n.is_negative() {
        return Err(RootError::NegativethRoot);
    }
    let prec: &T = PowPrecision::get::<T>().ok_or(RootError::PowError(PowError::UnsetPrecision))?;

    // initial guess
    let mut res = T::one();

    loop {
        let delta = (x.clone() / powi(res.clone(), n.clone() - E::one()) - res.clone())
            / T::from(n.clone());

        if delta.abs() < *prec {
            return Ok(res + delta);
        } else {
            res = res + delta;
        }
    }
}

#[allow(missing_docs)]
/// A rational number
pub trait Rational {
    type Int: Integer;

    fn numer(&self) -> Self::Int;

    fn denom(&self) -> Self::Int;

    fn reduced(self) -> Self;
}

/// Exponentiation to any rational exponent
pub fn pow<T, E, I>(x: T, exp: E) -> Result<T, PowError>
where
    T: 'static
        + Zero
        + One
        + Inv<Output = T>
        + Ord
        + Clone
        + Mul<Output = T>
        + From<I>
        + Add<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Abs<Output = T>,
    I: Integer + Neg<Output = I> + Div<Output = I> + Sub<Output = I> + Clone,
    E: Rational<Int = I>,
{
    // to spare work
    let exp = exp.reduced();

    let mut nu = exp.numer();
    let mut de = exp.denom();
    if de.is_negative() && nu.is_positive() {
        nu = -nu;
        de = -de;
    }
    root::<T, I>(powi(x, nu), de).map_err(|e| match e {
        RootError::PowError(p) => p,
        RootError::ZerothRoot => unreachable!("tried to raise to power of 1/0"),
        RootError::NegativethRoot => unreachable!("tried to raise to unreduced power"),
    })
}

macro_rules! int_impl {
    ($($t:ty),*) => {
    $(
        impl Integer for $t {
            fn is_even(&self) -> bool {
                self % 2 == 0
            }
        }
    )*
    };
}

int_impl!(i8, i16, i32, i64, i128, isize);

macro_rules! uint_impl {
    ($($t:ty),*) => {
    $(
        impl Integer for $t {
            fn is_positive(&self) -> bool {
                *self != 0
            }

            fn is_negative(&self) -> bool {
                false
            }

            fn is_even(&self) -> bool {
                self % 2 == 0
            }
        }
    )*
    };
}

uint_impl!(u8, u16, u32, u64, u128, usize);

macro_rules! float_impl {
    ($($t:ty),*) => {
    $(
        impl Rational for $t {
            type Int = i128;

            fn numer(&self) -> Self::Int {
                (self * (self.denom() as Self)) as Self::Int
            }

            fn denom(&self) -> Self::Int {
                1 << self.integer_decode().1
            }

            fn reduced(self) -> Self {
                self
            }
        }
    )*
    };
}

float_impl!(f32, f64);
