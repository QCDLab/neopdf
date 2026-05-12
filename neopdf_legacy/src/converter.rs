//! This module provides utilities for converting LHAPDF sets to the NeoPDF format and for
//! combining multiple nuclear PDF sets into a single NeoPDF file.
//_!
//! Main functions:
//! - `convert_lhapdf`: Converts an LHAPDF set to NeoPDF format and writes it to disk.
//! - `combine_lhapdf_npdfs`: Combines several nuclear PDF sets (with different nucleon
//!   numbers) into a single NeoPDF file with explicit A dependence.
use ndarray::{concatenate, Array1, Axis};
use regex::Regex;

use super::gridpdf::GridArray;
use super::metadata::{InterpolatorType, MetaData};
use super::parser::LhapdfSet;
use super::subgrid::{ParamRange, SubGrid};
use super::writer::GridArrayCollection;

/// Converts an LHAPDF set to the NeoPDF format and writes it to disk.
///
/// # Arguments
///
/// * `pdf_name` - The name of the LHAPDF set (e.g., "NNPDF40_nnlo_as_01180").
/// * `output_path` - The path to the output NeoPDF file.
///
/// # Errors
///
/// Returns an error if reading or writing fails.
pub fn convert_lhapdf<P: AsRef<std::path::Path>>(
    pdf_name: &str,
    output_path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let lhapdf_set = LhapdfSet::new(pdf_name);
    let members = lhapdf_set.members();
    if members.is_empty() {
        return Err("No members found in the LHAPDF set".into());
    }

    // All members share the same metadata
    let metadata = &members[0].0.clone();
    let grids: Vec<&GridArray> = members
        .iter()
        .map(|(_meta, knot_array)| knot_array)
        .collect();

    GridArrayCollection::compress(&grids, metadata, output_path)?;
    Ok(())
}

/// Combines a list of nuclear PDF sets (differing in nucleon number A) into a single NeoPDF
/// file with explicit A dependence.
///
/// # Arguments
/// * `pdf_names` - List of PDF set names (each with a different A).
/// * `output_path` - Output NeoPDF file path.
///
/// # Errors
/// Returns an error if loading or writing fails, or if the sets are not compatible.
pub fn combine_lhapdf_npdfs<P: AsRef<std::path::Path>>(
    pdf_names: &[&str],
    output_path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    if pdf_names.is_empty() {
        return Err("No PDF set names provided".into());
    }

    // Regexes to extract A from the PDF set name
    let re_nnpdf = Regex::new(r"_A(\d+)").unwrap();
    let re_ncteq = Regex::new(r"_(\d+)_(\d+)$").unwrap();
    let re_epps = Regex::new(r"[a-zA-Z]+(\d+)$").unwrap();
    let mut a_values = Vec::new();
    let mut all_members: Vec<Vec<(MetaData, GridArray)>> = Vec::new();

    for &pdf_name in pdf_names {
        let a = if let Some(cap) = re_nnpdf.captures(pdf_name) {
            cap[1].parse::<f64>().unwrap()
        } else if let Some(cap) = re_ncteq.captures(pdf_name) {
            cap[1].parse::<f64>().unwrap()
        } else if let Some(cap) = re_epps.captures(pdf_name) {
            cap[1].parse::<f64>().unwrap()
        } else if pdf_name.ends_with("_p") {
            1.0 // proton
        } else {
            return Err(format!("Could not extract A from PDF name: {}", pdf_name).into());
        };
        a_values.push(a);
        let set = LhapdfSet::new(pdf_name);
        let members = set.members();
        if members.is_empty() {
            return Err(format!("No members found in set: {}", pdf_name).into());
        }
        all_members.push(members);
    }

    let nucleons_range = ParamRange::new(
        *a_values
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap(),
        *a_values
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap(),
    );

    // Check all sets have the same number of members
    let num_members = all_members[0].len();
    if !all_members.iter().all(|v| v.len() == num_members) {
        return Err("All sets must have the same number of members".into());
    }

    // For each member index, combine the corresponding member from each set along the A dimension
    let mut combined_grids = Vec::with_capacity(num_members);
    let mut meta = all_members[0][0].0.clone();
    meta.set_desc = format!("Combined nuclear PDFs: {}", pdf_names.join(", "));
    meta.num_members = num_members as u32;
    meta.interpolator_type = InterpolatorType::LogTricubic;

    for member_idx in 0..num_members {
        let member_arrays: Vec<&GridArray> = all_members.iter().map(|v| &v[member_idx].1).collect();

        let pids = member_arrays[0].pids.clone();
        let num_subgrids = member_arrays[0].subgrids.len();
        if !member_arrays
            .iter()
            .all(|ga| ga.pids == pids && ga.subgrids.len() == num_subgrids)
        {
            return Err("All sets must have the same flavors and subgrid structure".into());
        }

        // For each subgrid, stack along the A dimension
        let mut combined_subgrids = Vec::with_capacity(num_subgrids);
        for subgrid_idx in 0..num_subgrids {
            // For each set, get the subgrid
            let subgrids: Vec<&SubGrid> = member_arrays
                .iter()
                .map(|ga| &ga.subgrids[subgrid_idx])
                .collect();

            // Check x, q2, alphas shapes match
            let xs = &subgrids[0].xs;
            let q2s = &subgrids[0].q2s;
            let kts = &subgrids[0].kts;
            let alphas = &subgrids[0].alphas;
            if !subgrids
                .iter()
                .all(|sg| sg.xs == *xs && sg.q2s == *q2s && sg.alphas == *alphas && sg.kts == *kts)
            {
                return Err("All sets must have the same x, q2, kT, and alphas grids".into());
            }

            // Concatenate along the nucleons axis to get [nucleons=pdf_names.len(), ...]
            let grid_views: Vec<_> = subgrids.iter().map(|sg| sg.grid.view()).collect();
            let concatenated = concatenate(Axis(0), &grid_views.to_vec())?;
            let nucleons = Array1::from(a_values.clone());
            let new_subgrid = SubGrid {
                xs: xs.clone(),
                q2s: q2s.clone(),
                kts: kts.clone(),
                grid: concatenated,
                nucleons,
                alphas: alphas.clone(),
                nucleons_range,
                alphas_range: subgrids[0].alphas_range,
                kt_range: subgrids[0].kt_range,
                x_range: subgrids[0].x_range,
                q2_range: subgrids[0].q2_range,
            };
            combined_subgrids.push(new_subgrid);
        }
        let combined_grid = GridArray {
            pids: pids.clone(),
            subgrids: combined_subgrids,
        };
        combined_grids.push(combined_grid);
    }

    let combined_grids: Vec<&GridArray> = combined_grids.iter().collect();
    GridArrayCollection::compress(&combined_grids, &meta, output_path)?;
    Ok(())
}

/// Combines a list of PDF sets (differing in alpha_s) into a single NeoPDF file with explicit
/// `alpha_s` dependence.
///
/// # Arguments
/// * `pdf_names` - List of PDF set names (each with a different alpha_s).
/// * `output_path` - Output NeoPDF file path.
///
/// # Errors
/// Returns an error if loading or writing fails, or if the sets are not compatible.
pub fn combine_lhapdf_alphas<P: AsRef<std::path::Path>>(
    pdf_names: &[&str],
    output_path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    if pdf_names.is_empty() {
        return Err("No PDF set names provided".into());
    }

    // Regexes to extract alpha_s from the PDF set name
    let re_nnpdf_ct = Regex::new(r"_as_(\d+)_?").unwrap();
    let re_msht = Regex::new(r"_as(\d+)").unwrap();
    let mut alphas_values = Vec::new();
    let mut all_members: Vec<Vec<(MetaData, GridArray)>> = Vec::new();

    for &pdf_name in pdf_names {
        let alphas = if let Some(cap) = re_nnpdf_ct.captures(pdf_name) {
            cap[1].parse::<f64>().unwrap() / 10000.0
        } else if let Some(cap) = re_msht.captures(pdf_name) {
            cap[1].parse::<f64>().unwrap() / 1000.0
        } else {
            return Err(format!("Could not extract alpha_s from PDF name: {}", pdf_name).into());
        };
        alphas_values.push(alphas);
        let set = LhapdfSet::new(pdf_name);
        let members = set.members();
        if members.is_empty() {
            return Err(format!("No members found in set: {}", pdf_name).into());
        }
        all_members.push(members);
    }

    let alphas_range = ParamRange::new(
        *alphas_values
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap(),
        *alphas_values
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap(),
    );

    // Check all sets have the same number of members
    let num_members = all_members[0].len();
    if !all_members.iter().all(|v| v.len() == num_members) {
        return Err("All sets must have the same number of members".into());
    }

    // For each member index, combine the corresponding member from each set along the alpha_s dimension
    let mut combined_grids = Vec::with_capacity(num_members);
    let mut meta = all_members[0][0].0.clone();
    meta.set_desc = format!("Combined alpha_s PDFs: {}", pdf_names.join(", "));
    meta.num_members = num_members as u32;
    meta.interpolator_type = InterpolatorType::LogTricubic;

    for member_idx in 0..num_members {
        // For each set, get the GridArray for this member
        let member_arrays: Vec<&GridArray> = all_members.iter().map(|v| &v[member_idx].1).collect();

        // Assume all have the same pids and subgrid structure
        let pids = member_arrays[0].pids.clone();
        let num_subgrids = member_arrays[0].subgrids.len();
        if !member_arrays
            .iter()
            .all(|ga| ga.pids == pids && ga.subgrids.len() == num_subgrids)
        {
            return Err("All sets must have the same flavors and subgrid structure".into());
        }

        // For each subgrid, stack along the alpha_s dimension
        let mut combined_subgrids = Vec::with_capacity(num_subgrids);
        for subgrid_idx in 0..num_subgrids {
            // For each set, get the subgrid
            let subgrids: Vec<&SubGrid> = member_arrays
                .iter()
                .map(|ga| &ga.subgrids[subgrid_idx])
                .collect();

            // Check x, q2, nucleons shapes match
            let xs = &subgrids[0].xs;
            let q2s = &subgrids[0].q2s;
            let kts = &subgrids[0].kts;
            let nucleons = &subgrids[0].nucleons;
            if !subgrids.iter().all(|sg| {
                sg.xs == *xs && sg.q2s == *q2s && sg.nucleons == *nucleons && sg.kts == *kts
            }) {
                return Err("All sets must have the same x, q2, kT, and nucleons grids".into());
            }

            // Concatenate along the alphas axis to get [..., alphas=pdf_names.len(), ...]
            let grid_views: Vec<_> = subgrids.iter().map(|sg| sg.grid.view()).collect();
            let concatenated = concatenate(Axis(1), &grid_views.to_vec())?;
            let alphas = Array1::from(alphas_values.clone());
            let new_subgrid = SubGrid {
                xs: xs.clone(),
                q2s: q2s.clone(),
                kts: kts.clone(),
                grid: concatenated,
                nucleons: nucleons.clone(),
                alphas,
                nucleons_range: subgrids[0].nucleons_range,
                alphas_range,
                kt_range: subgrids[0].kt_range,
                x_range: subgrids[0].x_range,
                q2_range: subgrids[0].q2_range,
            };
            combined_subgrids.push(new_subgrid);
        }
        let combined_grid = GridArray {
            pids: pids.clone(),
            subgrids: combined_subgrids,
        };
        combined_grids.push(combined_grid);
    }

    let combined_grids: Vec<&GridArray> = combined_grids.iter().collect();
    GridArrayCollection::compress(&combined_grids, &meta, output_path)?;
    Ok(())
}
