use core::ops::{Add, Div, Mul, Sub};

/// Linear interpolation
pub fn lerp<A, B, C, D, T>(a: A, b: B, t: T) -> B::Output
where
    A: Sub<B, Output = C>,
    B: Clone + Add<D>,
    C: Mul<T, Output = D>,
{
    b.clone() + (a - b) * t
}

/// A struct for mapping ranges linearly
///
/// Example:
/// ```
/// # use tlib::MapRange;
/// let map = MapRange::from_start_end((1, 3), (2, 6));
/// assert_eq!(map.into_eval(2), 4);
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct MapRange<T> {
    before_range_start: T,
    before_range_len: T,
    after_range_start: T,
    after_range_len: T,
}

impl<T> MapRange<T> {
    /// Construct the map from two start-end pairs of values
    ///
    /// The map will map the first range to the second
    pub fn from_start_end(before: (T, T), after: (T, T)) -> Self
    where
        T: Sub<Output = T> + Clone,
    {
        Self {
            before_range_start: before.0.clone(),
            before_range_len: before.1 - before.0,
            after_range_start: after.0.clone(),
            after_range_len: after.1 - after.0,
        }
    }

    /// Construct the map from two start-length pairs of values
    ///
    /// The map will map the first range to the second
    pub fn from_start_len(before: (T, T), after: (T, T)) -> Self {
        Self {
            before_range_start: before.0,
            before_range_len: before.1,
            after_range_start: after.0,
            after_range_len: after.1,
        }
    }

    /// Evaluate the map for a point.
    ///
    /// - This does not consume `self` however it requires `T: Clone`
    /// - This does the division after everything else to avoid precision errors
    pub fn eval(&self, point: T) -> T
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Clone,
    {
        let a = self.after_range_len.clone() * (point - self.before_range_start.clone());
        let b = self.after_range_start.clone() * self.before_range_len.clone();
        (a + b) / self.before_range_len.clone()
    }

    /// Evaluate the map for a point.
    ///
    /// - This does not require `T: Clone` however it consumes `self`
    /// - This does the division second to last to avoid precision errors as much as possible while avoiding `Clone`
    pub fn into_eval(self, point: T) -> T
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
    {
        let a = self.after_range_len * (point - self.before_range_start);
        (a / self.before_range_len) + self.after_range_start
    }
}
