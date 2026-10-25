//! This module provides utility functions for interpolation and grid operations.
//!
//! It includes helpers for finding interval indices in coordinate arrays and for
//! performing 1D cubic interpolation using Hermite basis functions. Finds the index
//! of the interval in a sorted coordinate array that contains the given value.

/// Dynamical object that contains the LHAPDF mapping from LHAID to PDF names.
const LHAPDF_INDEX_URL: &str = "https://lhapdfsets.web.cern.ch/current/pdfsets.index";

/// Resolves an LHAID to a `(set_name, member_index)` pair by fetching the LHAPDF index.
///
/// Each entry in `pdfsets.index` gives the base LHAID for a named PDF set. The range
/// owned by a set is `[base_id, next_base_id)`, so member N has LHAID `base_id + N`.
/// For example, if `NNPDF40_nnlo_as_01180` has base ID 331100, then LHAID 331103
/// maps to `("NNPDF40_nnlo_as_01180", 3)`.
pub(crate) fn lookup_lhaid(lhaid: u32) -> Result<(String, usize), String> {
    let text = ureq::get(LHAPDF_INDEX_URL)
        .call()
        .map_err(|e| format!("Failed to fetch pdfsets.index: {e}"))?
        .into_string()
        .map_err(|e| format!("Failed to read pdfsets.index response: {e}"))?;

    let mut entries: Vec<(u32, String)> = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let base_id: u32 = match parts.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => continue,
        };
        let name = match parts.next() {
            Some(s) => s.to_string(),
            None => continue,
        };
        entries.push((base_id, name));
    }

    for window in entries.windows(2) {
        let (base, name) = &window[0];
        let (next_base, _) = &window[1];
        if lhaid >= *base && lhaid < *next_base {
            return Ok((name.clone(), (lhaid - base) as usize));
        }
    }

    if let Some((base, name)) = entries.last() {
        if lhaid >= *base {
            return Ok((name.clone(), (lhaid - base) as usize));
        }
    }

    Err(format!("LHAID {lhaid} not found in pdfsets.index"))
}

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
    find_interval_index_by(coords, &value, f64::partial_cmp).ok_or_else(|| {
        ninterp::error::InterpolateError::ExtrapolateError("Out of Bounds!".to_string())
    })
}

/// Arithmetic-generic form of [`find_interval_index`].
///
/// `compare(coordinate, value)` orders a coordinate against the value in the caller's own
/// arithmetic, so no conversion to `f64` is involved. The coordinates must be sorted and contain
/// at least two entries. Returns `None` if the value lies outside `[coords[0], coords[last]]` or
/// cannot be ordered.
pub(crate) fn find_interval_index_by<T>(
    coords: &[T],
    value: &T,
    compare: impl Fn(&T, &T) -> Option<std::cmp::Ordering>,
) -> Option<usize> {
    use std::cmp::Ordering;

    if coords.len() < 2 {
        return None;
    }
    let last = coords.len() - 1;
    // Check bounds
    if compare(&coords[0], value)? == Ordering::Greater {
        return None;
    }
    match compare(&coords[last], value)? {
        Ordering::Less => return None,
        // Handle exact match with last coordinate
        Ordering::Equal => return Some(last - 1),
        Ordering::Greater => {}
    }

    // Binary search for the interval
    let mut left = 0;
    let mut right = last;
    while left < right {
        let mid = (left + right) / 2;
        if compare(&coords[mid], value)? != Ordering::Greater {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    Some(left - 1)
}

/// One-dimensional cubic interpolation using Hermite basis functions.
///
/// @arg t is the fractional distance of the evaluation x into the dx
/// interval.  @arg vl and @arg vh are the function values at the low and
/// high edges of the interval. @arg vdl and @arg vdh are linearly
/// extrapolated value changes from the product of dx and the discrete low-
/// and high-edge derivative estimates.
pub fn hermite_cubic_interpolate(t: f64, vl: f64, vdl: f64, vh: f64, vdh: f64) -> f64 {
    crate::precision::hermite(&crate::precision::Binary64, &t, &vl, &vdl, &vh, &vdh)
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

        // An unordered query is out of range rather than a panic.
        assert!(find_interval_index(&coords, f64::NAN).is_err());
    }
}
