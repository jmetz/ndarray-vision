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

impl<T> HoughParameters<T> {
    fn is_valid(&self) -> bool {
        let shape = self.bounds.dim();
        if shape.0 != 2 {
            false
        } else {
            match self.resolution {
                Some(ref s) => s.dim() == shape.1,
                None => true,
            }
        }
    }
}

pub trait HoughSearch<T> {
    fn get_entries(&self, coordinate: (usize, usize), params: &HoughParameters<T>) -> Array2<T>;
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

impl GeneralisedHoughTransformExt for Array2<bool>
{
    fn hough<T, D, S, Search>(&self, params: HoughParameters<T>, search: &Search) -> ArrayBase<S, D>
    where
        S: Data<Elem = usize>,
        Search: HoughSearch<T>,
    {
        // work out dimensions of accumulator from parameters 
        for i in self.indexed_iter().filter(|(_, b)| **b).map(|(i, _)| i) {
            let _entries = search.get_entries(i, &params);
            // Add to accumulator
        }
        // Return accumulator
        unimplemented!();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameters() {
        let x = HoughParameters {
            bounds: arr2(&[[0, 1, 2]]),
            resolution: None,
        };
        assert_eq!(x.is_valid(), false);
        
        let x = HoughParameters {
            bounds: arr2(&[[0, 1, 2], [1, 2, 3]]),
            resolution: None,
        };

        assert!(x.is_valid());
        
        let x = HoughParameters {
            bounds: arr2(&[[0, 1, 2], [0, 1, 2], [0, 0, 0]]),
            resolution: None,
        };
        assert_eq!(x.is_valid(), false);

        let x = HoughParameters {
            bounds: arr2(&[[0, 2],[1, 3]]),
            resolution: Some(arr1(&[1, 2])),
        };
        assert!(x.is_valid());

        let x = HoughParameters {
            bounds: arr2(&[[0, 2],[1, 3]]),
            resolution: Some(arr1(&[1, 2, 3])),
        };
        assert_eq!(x.is_valid(), false);
    }
}
