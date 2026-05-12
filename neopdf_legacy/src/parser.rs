//! This module provides parsing utilities for reading and interpreting PDF set data files.
//!
//! It defines types and methods for loading, parsing, and representing both LHAPDF and NeoPDF
//! set formats, including subgrid data extraction and metadata reading.
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use super::gridpdf::GridArray;
use super::manage::{ManageData, PdfSetFormat};
use super::metadata::MetaData;
use super::writer::{GridArrayReader, LazyGridArrayIterator};

/// Represents the data for a single subgrid within a PDF data file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubgridData {
    pub nucleons: Vec<f64>,
    pub alphas: Vec<f64>,
    pub kts: Vec<f64>,
    pub xs: Vec<f64>,
    pub q2s: Vec<f64>,
    pub grid_data: Vec<f64>,
}

/// Represents the parsed data from an LHAPDF `.dat` file.
#[derive(Debug, Serialize, Deserialize)]
pub struct PdfData {
    pub subgrid_data: Vec<SubgridData>,
    pub pids: Vec<i32>,
    pub alphas_q_values: Option<Vec<f64>>,
    pub alphas_vals: Option<Vec<f64>>,
}

/// Manages the loading and parsing of LHAPDF data sets.
///
/// This struct provides methods to read metadata and member data files
/// for a given LHAPDF set.
pub struct LhapdfSet {
    manager: ManageData,
    pub info: MetaData,
}

impl LhapdfSet {
    /// Creates a new `LhapdfSet` instance for a given PDF set name.
    ///
    /// This constructor initializes the data manager and reads the metadata
    /// for the specified PDF set.
    ///
    /// # Arguments
    ///
    /// * `pdf_name` - The name of the PDF set (e.g., "NNPDF40_nnlo_as_01180").
    pub fn new(pdf_name: &str) -> Self {
        let manager = ManageData::new(pdf_name, PdfSetFormat::Lhapdf);
        let pdfset_path = manager.set_path();
        let info_path = pdfset_path.join(format!(
            "{}.info",
            pdfset_path.file_name().unwrap().to_str().unwrap()
        ));
        let info: MetaData = Self::read_metadata(&info_path).unwrap();

        Self { manager, info }
    }

    /// Reads the metadata and data for a specific member of the PDF set.
    ///
    /// # Arguments
    ///
    /// * `member` - The ID of the PDF member to load.
    ///
    /// # Returns
    ///
    /// A tuple containing the `MetaData` and `PdfData` for the specified member.
    pub fn member(&self, member: usize) -> (MetaData, GridArray) {
        let pdfset_path = self.manager.set_path();
        let data_path = pdfset_path.join(format!(
            "{}_{:04}.dat",
            pdfset_path.file_name().unwrap().to_str().unwrap(),
            member
        ));

        let pdf_data = Self::read_data(&data_path);
        let knot_array = GridArray::new(pdf_data.subgrid_data, pdf_data.pids);

        let mut info = self.info.clone();
        if info.alphas_vals.is_empty() {
            if let (Some(vals), Some(q_values)) = (pdf_data.alphas_vals, pdf_data.alphas_q_values) {
                if !vals.is_empty() && !q_values.is_empty() {
                    info.alphas_vals = vals;
                    info.alphas_q_values = q_values;
                }
            }
        }
        (info, knot_array)
    }

    /// Reads the metadata and data for all members of the PDF set.
    ///
    /// # Returns
    ///
    /// A vector of tuples, where each tuple contains the `MetaData` and `PdfData`
    /// for a member of the set.
    pub fn members(&self) -> Vec<(MetaData, GridArray)> {
        (0..self.info.num_members as usize)
            .map(|i| self.member(i))
            .collect()
    }

    /// Reads the `.info` file for a PDF set and deserializes it into an `Info` struct.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the `.info` file.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Info` struct if successful, or a `serde_yaml::Error` otherwise.
    fn read_metadata(path: &Path) -> Result<MetaData, serde_yaml::Error> {
        let content = fs::read_to_string(path).unwrap();
        serde_yaml::from_str(&content)
    }

    /// Reads an LHAPDF `.dat` file for a PDF set and parses its content.
    ///
    /// This function extracts x-knots, Q2-knots, flavor IDs, and the grid data
    /// from the specified data file. It can handle files with multiple subgrids
    /// separated by "---".
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the `.dat` file.
    ///
    /// # Returns
    ///
    /// A `PdfData` struct containing the parsed subgrid data and flavor IDs.
    pub fn read_data(path: &Path) -> PdfData {
        let content = fs::read_to_string(path).unwrap();
        let mut subgrid_data = Vec::new();
        let mut flavors = Vec::new();
        let mut alphas_q_values: Option<Vec<f64>> = None;
        let mut alphas_vals: Option<Vec<f64>> = None;

        let blocks: Vec<&str> = content.split("---").map(|s| s.trim()).collect();

        // NOTE: support cases in which `AlphaS` grid info are in `.dat` files.
        if !blocks.is_empty() {
            #[derive(serde::Deserialize)]
            struct DatMeta {
                #[serde(rename = "AlphaS_Qs", default)]
                alphas_q_values: Vec<f64>,
                #[serde(rename = "AlphaS_Vals", default)]
                alphas_vals: Vec<f64>,
            }

            let metadata_block = blocks[0];
            if let Ok(dat_meta) = serde_yaml::from_str::<DatMeta>(metadata_block) {
                if !dat_meta.alphas_q_values.is_empty() {
                    alphas_q_values = Some(dat_meta.alphas_q_values);
                }
                if !dat_meta.alphas_vals.is_empty() {
                    alphas_vals = Some(dat_meta.alphas_vals);
                }
            }
        }

        for block in blocks.iter().skip(1) {
            if block.is_empty() {
                continue;
            }

            let mut lines = block.lines();

            let x_knots_line = lines.next().unwrap();
            let xs: Vec<f64> = x_knots_line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            let q2_knots_line = lines.next().unwrap();
            let q2s: Vec<f64> = q2_knots_line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .map(|q: f64| q * q)
                .collect();

            // Read the flavors (only once from the first subgrid)
            if flavors.is_empty() {
                let flavors_line = lines.next().unwrap();
                flavors = flavors_line
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
            } else {
                // Skip the flavors line in subsequent subgrids
                lines.next();
            }

            let mut grid_data = Vec::new();
            for line in lines {
                let values: Vec<f64> = line
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                grid_data.extend(values);
            }

            // NOTE: given that there isn't really a proper way to extract the
            // following values from LHAPDF, their defaults are set to zeros.
            let nucleons: Vec<f64> = vec![0.0];
            let alphas: Vec<f64> = vec![0.0];
            let kts: Vec<f64> = vec![0.0];

            subgrid_data.push(SubgridData {
                nucleons,
                alphas,
                kts,
                xs,
                q2s,
                grid_data,
            });
        }

        PdfData {
            subgrid_data,
            pids: flavors,
            alphas_q_values,
            alphas_vals,
        }
    }
}

/// Manages the loading and parsing of NeoPDF sets.
pub struct NeopdfSet {
    pub info: MetaData,
    grid_reader: GridArrayReader,
    setpath: PathBuf,
}

impl NeopdfSet {
    /// TODO
    pub fn new(pdf_name: &str) -> Self {
        let manager = ManageData::new(pdf_name, PdfSetFormat::Neopdf);
        let neopdf_setpath = manager.set_path();
        let grid_readers = GridArrayReader::from_file(neopdf_setpath).unwrap();
        let metadata_info = grid_readers.metadata().as_ref().clone();

        Self {
            info: metadata_info,
            grid_reader: grid_readers,
            setpath: neopdf_setpath.to_path_buf(),
        }
    }

    /// TODO
    pub fn member(&self, member: usize) -> (MetaData, GridArray) {
        let load_grid = self.grid_reader.load_grid(member).unwrap();
        (self.info.clone(), load_grid.grid)
    }

    /// TODO
    pub fn into_lazy_iterators(&self) -> LazyGridArrayIterator {
        LazyGridArrayIterator::from_file(&self.setpath).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_info() {
        let yaml_content = r#"
        SetDesc: "NNPDF40_nnlo_as_01180"
        SetIndex: 4000
        NumMembers: 101
        XMin: 1.0e-9
        XMax: 1.0
        QMin: 1.0
        QMax: 10000.0
        Flavors: [21, 1, 2, 3, 4, 5, -1, -2, -3, -4, -5]
        Format: "LHAPDF"
        "#;
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", yaml_content).unwrap();
        let info = LhapdfSet::read_metadata(temp_file.path()).unwrap();

        assert_eq!(info.set_desc, "NNPDF40_nnlo_as_01180");
        assert_eq!(info.set_index, 4000);
        assert_eq!(info.num_members, 101);
        assert_eq!(info.x_min, 1.0e-9);
        assert_eq!(info.x_max, 1.0);
        assert_eq!(info.q_min, 1.0);
        assert_eq!(info.q_max, 10000.0);
        assert_eq!(info.flavors, vec![21, 1, 2, 3, 4, 5, -1, -2, -3, -4, -5]);
        assert_eq!(info.format, "LHAPDF");
    }

    #[test]
    fn test_read_data() {
        let data_content = r#"
        # Some header
        ---
        1.0e-9 1.0e-8 1.0e-7
        1.0 10.0 100.0
        21 1 2
        1.0 2.0 3.0
        4.0 5.0 6.0
        7.0 8.0 9.0
        ---
        1.0e-7 1.0e-6 1.0e-5
        100.0 1000.0 10000.0
        21 1 2
        10.0 11.0 12.0
        13.0 14.0 15.0
        16.0 17.0 18.0
        "#;
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", data_content).unwrap();
        let pdf_data = LhapdfSet::read_data(temp_file.path());

        assert_eq!(pdf_data.pids, vec![21, 1, 2]);
        assert_eq!(pdf_data.subgrid_data.len(), 2);

        // Check the first subgrid
        assert_eq!(pdf_data.subgrid_data[0].xs, vec![1.0e-9, 1.0e-8, 1.0e-7]);
        assert_eq!(pdf_data.subgrid_data[0].q2s, vec![1.0, 100.0, 10000.0]); // Q values are squared
        assert_eq!(
            pdf_data.subgrid_data[0].grid_data,
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
        );

        // Check the second subgrid
        assert_eq!(pdf_data.subgrid_data[1].xs, vec![1.0e-7, 1.0e-6, 1.0e-5]);
        assert_eq!(
            pdf_data.subgrid_data[1].q2s,
            vec![10000.0, 1000000.0, 100000000.0]
        ); // Q values are squared
        assert_eq!(
            pdf_data.subgrid_data[1].grid_data,
            vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0]
        );
    }
}
