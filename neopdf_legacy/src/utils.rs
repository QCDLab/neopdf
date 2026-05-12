//! This module provides utility functions for interpolation and grid operations.
//!
//! It includes helpers for finding interval indices in coordinate arrays and for
//! performing 1D cubic interpolation using Hermite basis functions. Finds the index
//! of the interval in a sorted coordinate array that contains the given value.
///
/// This function performs a binary search to efficiently locate the correct interval.
///
/// # Arguments
///
/// * `coords` - A sorted slice of f64 values representing the coordinates.
/// * `value` - The f64 value for which to find the interval.
///
/// # Returns
///
/// A `Result` containing the 0-based index of the left bound of the interval if successful.
/// Returns an `InterpolateError::ExtrapolateError` if the value is outside the bounds
/// of the `coords` array.
pub fn find_interval_index(
    coords: &[f64],
    value: f64,
) -> Result<usize, ninterp::error::InterpolateError> {
    // Check bounds
    if value < coords[0] || value > coords[coords.len() - 1] {
        return Err(ninterp::error::InterpolateError::ExtrapolateError(
            "Out of Bounds!".to_string(),
        ));
    }

    // Handle exact match with last coordinate
    if value == coords[coords.len() - 1] {
        return Ok(coords.len() - 2);
    }

    // Binary search for the interval
    let mut left = 0;
    let mut right = coords.len() - 1;

    while left < right {
        let mid = (left + right) / 2;
        if coords[mid] <= value {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    Ok(left - 1)
}

/// One-dimensional cubic interpolation using Hermite basis functions.
///
/// @arg t is the fractional distance of the evaluation x into the dx
/// interval.  @arg vl and @arg vh are the function values at the low and
/// high edges of the interval. @arg vdl and @arg vdh are linearly
/// extrapolated value changes from the product of dx and the discrete low-
/// and high-edge derivative estimates.
pub fn hermite_cubic_interpolate(t: f64, vl: f64, vdl: f64, vh: f64, vdh: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;

    let p0 = (2.0 * t3 - 3.0 * t2 + 1.0) * vl;
    let m0 = (t3 - 2.0 * t2 + t) * vdl;
    let p1 = (-2.0 * t3 + 3.0 * t2) * vh;
    let m1 = (t3 - t2) * vdh;

    p0 + m0 + p1 + m1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_interval_index() {
        let coords = vec![0.0, 1.0, 2.0, 3.0, 4.0];

        // Test within bounds
        assert_eq!(find_interval_index(&coords, 0.5).unwrap(), 0);
        assert_eq!(find_interval_index(&coords, 1.0).unwrap(), 1);
        assert_eq!(find_interval_index(&coords, 1.5).unwrap(), 1);
        assert_eq!(find_interval_index(&coords, 3.9).unwrap(), 3);

        // Test at boundaries
        assert_eq!(find_interval_index(&coords, 0.0).unwrap(), 0);
        assert_eq!(find_interval_index(&coords, 4.0).unwrap(), 3);

        // Test out of bounds
        assert!(find_interval_index(&coords, -0.1).is_err());
        assert!(find_interval_index(&coords, 4.1).is_err());
    }
}
