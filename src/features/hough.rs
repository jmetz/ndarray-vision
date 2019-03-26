use crate::core::Image;
use ndarray::{prelude::*, Data, Zip};
use num_traits::float::FloatConst;

/// Contains the parameters of the Hough transform. As this is a generalised
/// Hough Transform, for each rotation of the target shape there is an
/// N-dimensioned point in the Hough space with a corresponding bucket that
/// needs a count incremented. These parameters detail the function to take
/// the angles into the N-dimensioned point, the parameter ranges and an optional
/// search resolution
///
/// Assume all angles are in Radians.
pub struct HoughParameters<T> {
    /// Parameter bounds
    pub bounds: Array2<T>,
    /// Search resolution for each parameter. Should be the same size as bounds.
    pub resolution: Option<Array1<T>>,
}

pub trait HoughSearch<T> {
    fn get_entry(&self, coordinate: (usize, usize), params: &HoughParameters<T>) -> Array2<T>;
}

/// Trait for a general Hough Transform.
pub trait GeneralisedHoughTransformExt {
    /// Given the `HoughParameters` return the accumulated view of the Hough Space
    fn hough<T, D, S, Search>(
        &self,
        params: HoughParameters<T>,
        search: &Search,
    ) -> ArrayBase<S, D>
    where
        S: Data<Elem = usize>,
        Search: HoughSearch<T>;
}

impl<U> GeneralisedHoughTransformExt for Array2<U>
where
    U: Data<Elem = bool>,
{
    fn hough<T, D, S, Search>(&self, params: HoughParameters<T>, search: &Search) -> ArrayBase<S, D>
    where
        S: Data<Elem = usize>,
        Search: HoughSearch<T>,
    {
        // Get all accumulator entries for coordinate (0, 0)
        let _entry = search.get_entry((0, 0), &params);

        unimplemented!();
    }
}
