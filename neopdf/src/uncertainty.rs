//! PDF uncertainty computation for NeoPDF sets.
//!
//! This module provides the [`Uncertainty`] struct and the [`uncertainty`] function
//! for computing PDF uncertainties from an ensemble of member values, supporting
//! both Monte Carlo (replica) and Hessian error types.

/// 1-sigma confidence level (68.268949213708578%).
pub const CL_1_SIGMA: f64 = 68.268_949_213_708_58;

/// Inverse normal CDF (probit function) using the Abramowitz & Stegun approximation.
///
/// Maps a probability `p ∈ (0, 1)` to the corresponding quantile of the standard
/// normal distribution (i.e., the number of sigma corresponding to `p`).
fn map_prob(p: f64) -> f64 {
    let c = [2.515_517_f64, 0.802_853, 0.010_328];
    let d = [1.432_788_f64, 0.189_269, 0.001_308];

    let (sign, q) = if p >= 0.5 { (1.0, 1.0 - p) } else { (-1.0, p) };
    let t = (-2.0 * q.ln()).sqrt();
    let num = c[0] + c[1] * t + c[2] * t * t;
    let den = 1.0 + d[0] * t + d[1] * t * t + d[2] * t * t * t;
    sign * (t - num / den)
}

/// Converts a two-sided CL percentage to its normal-distribution sigma equivalent.
fn cl_to_sigma(cl: f64) -> f64 {
    map_prob(0.5 * (1.0 + cl / 100.0))
}

/// PDF uncertainty information computed from an ensemble of member values.
#[derive(Clone, Debug)]
pub struct Uncertainty {
    /// Central value (from member 0).
    pub central: f64,
    /// Downward error (absolute value).
    pub errminus: f64,
    /// Upward error (absolute value).
    pub errplus: f64,
}

/// Computes PDF uncertainties using the LHAPDF-compatible algorithm.
///
/// This function mirrors the behaviour of `lhapdf.PdfSet.uncertainty()`, including
/// confidence-level rescaling from the native CL of the set (`error_conf_level`) to
/// the requested output CL (`cl`), and the `alternative` prescription for replica sets.
///
/// # Arguments
///
/// * `values` - Slice of member values; `values[0]` is the central value (member 0),
///   the remaining elements are the error members in their natural ordering.
/// * `error_type` - The `ErrorType` metadata string of the PDF set (e.g. `"replicas"`,
///   `"hessian"`, `"symmhessian"`, `"asymhessian"`).
/// * `error_conf_level` - The confidence level (in %) at which the set's error members
///   were constructed (taken from `ErrorConfLevel` in the set's `.info` file).
///   Defaults to `CL_1_SIGMA` (≈ 68.27 %) for most replica and many Hessian sets;
///   use 90.0 for sets such as CT18.
/// * `cl` - The desired output confidence level in %.
/// * `alternative` - If `true`, replica sets use a quantile-based (asymmetric) interval
///   instead of the standard deviation.
///
/// # Errors
///
/// Returns an error string if `values` is empty.
pub fn uncertainty(
    values: &[f64],
    error_type: &str,
    error_conf_level: f64,
    cl: f64,
    alternative: bool,
) -> Result<Uncertainty, String> {
    if values.is_empty() {
        return Err("values slice must not be empty".to_string());
    }

    let err_type = error_type.to_lowercase();
    let native_cl = if error_conf_level > 0.0 {
        error_conf_level
    } else {
        CL_1_SIGMA
    };
    let cl_scale = cl_to_sigma(cl) / cl_to_sigma(native_cl);

    let (central, errminus, errplus) = if err_type.contains("replicas")
        || err_type.contains("monte carlo")
        || err_type.contains("mc_stat")
    {
        let n_err = (values.len() - 1) as f64;
        // LHAPDF convention for replica sets: the reported central value is the
        // arithmetic mean of the replica ensemble (members 1..N), not member 0.
        let mean: f64 = values.iter().skip(1).sum::<f64>() / n_err;
        if alternative {
            let mut sorted: Vec<f64> = values[1..].to_vec();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let n = sorted.len();
            let p_lo = (1.0 - cl / 100.0) / 2.0;
            let p_hi = (1.0 + cl / 100.0) / 2.0;
            let i_lo = (p_lo * n as f64).floor() as usize;
            let i_hi = ((p_hi * n as f64).ceil() as usize)
                .saturating_sub(1)
                .min(n - 1);
            let lo = sorted[i_lo];
            let hi = sorted[i_hi];
            (mean, (mean - lo).abs(), (hi - mean).abs())
        } else {
            let variance: f64 = values
                .iter()
                .skip(1)
                .map(|&v| (v - mean).powi(2))
                .sum::<f64>()
                / (n_err - 1.0);
            let err = variance.sqrt() * cl_scale;
            (mean, err, err)
        }
    } else if err_type.contains("asymhessian")
        || (err_type.contains("hessian") && !err_type.contains("symm"))
    {
        let c = values[0];
        let mut sum_plus_sq = 0.0;
        let mut sum_minus_sq = 0.0;
        for pair in values[1..].chunks(2) {
            if pair.len() == 2 {
                let (t_plus, t_minus) = (pair[0], pair[1]);
                sum_plus_sq +=
                    f64::max(0.0, t_plus - c).powi(2) + f64::max(0.0, t_minus - c).powi(2);
                sum_minus_sq +=
                    f64::max(0.0, c - t_plus).powi(2) + f64::max(0.0, c - t_minus).powi(2);
            }
        }
        (
            c,
            sum_minus_sq.sqrt() * cl_scale,
            sum_plus_sq.sqrt() * cl_scale,
        )
    } else {
        let c = values[0];
        let mut sum_sq = 0.0;
        for pair in values[1..].chunks(2) {
            if pair.len() == 2 {
                let diff = (pair[0] - pair[1]).abs() / 2.0;
                sum_sq += diff.powi(2);
            }
        }
        let err = sum_sq.sqrt() * cl_scale;
        (c, err, err)
    };

    Ok(Uncertainty {
        central,
        errminus,
        errplus,
    })
}
