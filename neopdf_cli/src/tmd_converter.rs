//! CLI logic for `NeoPDF` TMD conversion utilities.

use ndarray::Array1;
use serde::Deserialize;
use std::f64::consts::PI;
use std::fs;

use neopdf::gridpdf::GridArray;
use neopdf::metadata::{InterpolatorType, MetaDataV2, SetType};
use neopdf::subgrid::SubGrid;
use neopdf::writer::GridArrayCollection;
use neopdf_tmdlib::Tmd;

#[derive(Deserialize)]
struct TmdConfig {
    set_name: String,
    set_desc: String,
    set_index: u32,
    n_members: Option<usize>,
    n_x: Vec<usize>,
    n_q: Vec<usize>,
    n_kt: Vec<usize>,
    x_inner_edges: Vec<f64>,
    q_inner_edges: Vec<f64>,
    kt_inner_edges: Vec<f64>,
    pids: Vec<i32>,
    nucleons: Vec<f64>,
    alphas: Vec<f64>,
    alphas_qs: Vec<f64>,
    alphas_vals: Vec<f64>,
    polarised: bool,
    set_type: String,
    interpolator_type: String,
    error_type: String,
    hadron_pid: i32,
    flavor_scheme: String,
    order_qcd: u32,
    alphas_order_qcd: u32,
    m_w: f64,
    m_z: f64,
    m_up: f64,
    m_down: f64,
    m_strange: f64,
    m_charm: f64,
    m_bottom: f64,
    m_top: f64,
    alphas_type: String,
    number_flavors: u32,
}

#[allow(clippy::cast_precision_loss)]
fn create_cheby_grid(n_points: usize, min: f64, max: f64) -> Vec<f64> {
    let (u_min, u_max) = (min.ln(), max.ln());
    let range = u_max - u_min;
    let n_points_f64 = n_points as f64;
    let n_minus_1 = n_points_f64 - 1.0;

    (0..n_points)
        .map(|j| {
            let j_f64 = j as f64;
            let t_j = (PI * (n_minus_1 - j_f64) / n_minus_1).cos();
            (u_min + range * (t_j + 1.0) / 2.0).exp()
        })
        .collect()
}

fn create_geomspace_grid(n_points: usize, min: f64, max: f64) -> Vec<f64> {
    Array1::linspace(min.ln(), max.ln(), n_points)
        .mapv(f64::exp)
        .to_vec()
}

fn parse_set_type(s: &str) -> Result<SetType, String> {
    match s.to_ascii_lowercase().as_str() {
        "spacelike" => Ok(SetType::SpaceLike),
        "timelike" => Ok(SetType::TimeLike),
        _ => Err(format!("Unknown SetType: {s}")),
    }
}

fn parse_interpolator_type(s: &str) -> Result<InterpolatorType, String> {
    match s.to_ascii_lowercase().as_str() {
        "logtricubic" => Ok(InterpolatorType::LogTricubic),
        "logchebyshev" => Ok(InterpolatorType::LogChebyshev),
        _ => Err(format!("Unknown InterpolatorType: {s}")),
    }
}

fn construct_subgrids(
    zmin: f64,
    zmax: f64,
    z_inner_edges: &[f64],
    nz_points: &[usize],
    grid_fn: fn(usize, f64, f64) -> Vec<f64>,
) -> Vec<Vec<f64>> {
    if !z_inner_edges.is_empty() {
        assert!(
            &zmin < z_inner_edges.first().unwrap(),
            "The lower edge must be greater than the minimum value."
        );
        assert!(
            z_inner_edges.last().unwrap() < &zmax,
            "The upper edge must be smaller than the maximum value."
        );
    }

    let mut boundaries = vec![zmin];
    boundaries.extend_from_slice(z_inner_edges);
    boundaries.push(zmax);

    boundaries
        .windows(2)
        .zip(nz_points.iter())
        .map(|(window, &n_points)| grid_fn(n_points, window[0], window[1]))
        .collect()
}

fn create_grid_data(
    tmd: &mut Tmd,
    config: &TmdConfig,
    kts: &[f64],
    xs: &[f64],
    q2s: &[f64],
) -> Vec<f64> {
    // NOTE: Hard-coded definition of how `TMDlib` flavours are constructed.
    const TMDLIB_PIDS: &[i32] = &[-6, -5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5, 6];

    config
        .nucleons
        .iter()
        .flat_map(|_nuc| config.alphas.iter())
        .flat_map(|_alpha| kts.iter())
        .flat_map(|&kt| xs.iter().map(move |&x| (kt, x)))
        .flat_map(|(kt, x)| q2s.iter().map(move |&q2| (kt, x, q2)))
        .flat_map(|(kt, x, q2)| {
            let tmd_pds = tmd.xfxq2kt(x, kt, q2.sqrt());
            config.pids.iter().map(move |&pid| {
                TMDLIB_PIDS
                    .iter()
                    .position(|&p| p == pid)
                    .map_or(0.0, |pos| tmd_pds[pos])
            })
        })
        .collect()
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
fn create_member_grid(
    tmd: &mut Tmd,
    config: &TmdConfig,
    member: usize,
    kt_subgrids: &[&[f64]],
    x_subgrids: &[&[f64]],
    q2_subgrids: &[&[f64]],
) -> GridArray {
    tmd.init(&config.set_name, member as i32);

    let mut subgrids = Vec::new();
    for kts in kt_subgrids {
        for xs in x_subgrids {
            for q2s in q2_subgrids {
                let grid_data = create_grid_data(tmd, config, kts, xs, q2s);

                let subgrid = SubGrid::new(
                    config.nucleons.clone(),
                    config.alphas.clone(),
                    kts.to_vec(),
                    xs.to_vec(),
                    q2s.to_vec(),
                    config.pids.len(),
                    grid_data,
                );
                subgrids.push(subgrid);
            }
        }
    }

    GridArray {
        pids: config.pids.clone().into(),
        subgrids,
    }
}

/// TODO
///
/// # Errors
///
/// TODO
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
pub fn convert_tmd(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config: TmdConfig = toml::from_str(&fs::read_to_string(input_path)?)?;

    let mut tmd = Tmd::new();
    tmd.set_verbosity(0);
    tmd.init(&config.set_name, 0);
    let xmin = tmd.x_min();
    let xmax = tmd.x_max();
    let q2min = tmd.q2_min();
    let q2max = tmd.q2_max();
    let ktmin = tmd.kt_min();
    let ktmax = tmd.kt_max();

    let interpol_type = parse_interpolator_type(&config.interpolator_type)?;
    let create_grid_fn = match interpol_type {
        InterpolatorType::LogChebyshev => create_cheby_grid,
        _ => create_geomspace_grid,
    };

    let n_members = config.n_members.unwrap_or_else(|| tmd.num_members());
    let config_q2_inner_edges: Vec<f64> = config.q_inner_edges.iter().map(|&q| q.ln()).collect();
    let x_subgrids = construct_subgrids(
        xmin,
        xmax,
        &config.x_inner_edges,
        &config.n_x,
        create_grid_fn,
    );
    let q2_subgrids = construct_subgrids(
        q2min,
        q2max,
        &config_q2_inner_edges,
        &config.n_q,
        create_grid_fn,
    );
    let kt_subgrids = construct_subgrids(
        ktmin,
        ktmax,
        &config.kt_inner_edges,
        &config.n_kt,
        create_grid_fn,
    );

    // TODO: Find a better way to do this!
    let kts: Vec<&[f64]> = kt_subgrids.iter().map(Vec::as_slice).collect();
    let xs: Vec<&[f64]> = x_subgrids.iter().map(Vec::as_slice).collect();
    let q2s: Vec<&[f64]> = q2_subgrids.iter().map(Vec::as_slice).collect();

    let member_grids: Vec<_> = (0..n_members)
        .map(|m| {
            tmd.init(&config.set_name, m as i32);
            create_member_grid(&mut tmd, &config, m, &kts, &xs, &q2s)
        })
        .collect();

    let member_grid_refs: Vec<&GridArray> = member_grids.iter().collect();

    let meta = MetaDataV2 {
        set_desc: config.set_desc,
        set_index: config.set_index,
        num_members: n_members as u32,
        x_min: tmd.x_min(),
        x_max: tmd.x_max(),
        q_min: tmd.q2_min().sqrt(),
        q_max: tmd.q2_max().sqrt(),
        flavors: config.pids.clone(),
        format: "neopdf".to_string(),
        alphas_q_values: config.alphas_qs,
        alphas_vals: config.alphas_vals,
        polarised: config.polarised,
        set_type: parse_set_type(&config.set_type)?,
        interpolator_type: interpol_type,
        error_type: config.error_type,
        hadron_pid: config.hadron_pid,
        git_version: String::new(),
        code_version: String::new(),
        flavor_scheme: config.flavor_scheme,
        order_qcd: config.order_qcd,
        alphas_order_qcd: config.alphas_order_qcd,
        m_w: config.m_w,
        m_z: config.m_z,
        m_up: config.m_up,
        m_down: config.m_down,
        m_strange: config.m_strange,
        m_charm: config.m_charm,
        m_bottom: config.m_bottom,
        m_top: config.m_top,
        alphas_type: config.alphas_type,
        number_flavors: config.number_flavors,
        xi_min: 0.0,
        xi_max: 0.0,
        delta_min: 0.0,
        delta_max: 0.0,
    };

    GridArrayCollection::compress(&member_grid_refs, &meta, output_path)?;
    println!("Compression succeeded!");

    Ok(())
}
