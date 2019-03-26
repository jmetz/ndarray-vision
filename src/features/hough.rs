use crate::core::Image;
use ndarray::{prelude::*, Data};
use std::marker::PhantomData;

/// Contains the parameters of the Hough transform. As this is a generalised
/// Hough Transform, for each rotation of the target shape there is an
/// N-dimensioned point in the Hough space with a corresponding bucket that
/// needs a count incremented. These parameters detail the function to take
/// the angles into the N-dimensioned point, the parameter ranges and an optional
/// search resolution
///
/// Assume all angles are in Radians.
pub struct HoughParameters<T, D, F>
where
    D: Dimension,
    F: Sized + (Fn(f64) -> Array1<T>),
{
    /// Parameter bounds
    pub bounds: Array2<T>,
    /// Search resolution for each parameter. Should be the same size as bounds.
    pub resolution: Option<Array1<T>>,
    /// Function that taking an angle returns the n-dimensioned point in Hough
    /// Space that the entry would belong to. Output should be same dimensions
    /// as bounds.
    pub get_params: F,
    /// Represents the dimensionality of the hough space
    pub dim: PhantomData<D>,
}

/// Trait for a general Hough Transform.
pub trait GeneralisedHoughTransformExt<T, D, F>
where
    D: Dimension,
    F: Sized + (Fn(f64) -> Array1<T>),
{
    /// Given the `HoughParameters` return the accumulated view of the Hough Space
    fn hough<S>(&self, d_theta: f64, params: HoughParameters<T, D, F>) -> ArrayBase<S, D>
    where
        S: Data<Elem = usize>;
}


impl<T, D, F, S> GeneralisedHoughTransformExt<T, D, F> for Array2<S>
where 
    D: Dimension,
    S: Data<Elem = bool>,
    F: Sized + (Fn(f64) -> Array1<T>),
{

    fn hough<U>(&self, d_theta: f64, params: HoughParameters<T, D, F>) -> ArrayBase<U, D> 
    where
        U: Data<Elem = usize>
    {
        unimplemented!();
    }
}
