C++ OOP API Examples
====================

This example briefly demonstrates how to use the `NeoPDF` C++ Object Oriented (OOP) API to load
and evaluate parton distributions. More examples can be found in
`neopdf_capi/tests <https://github.com/Radonirinaunimi/neopdf/tree/master/neopdf_capi/tests>`_.

Prerequisites
*************

Build and install the C++ API as described in the
`installation guide <https://radonirinaunimi.github.io/neopdf/installation/>`_. The C++ OOP header
is needed for the following examples.

Example 1: Loading and Evaluating PDFs
**************************************

This example demonstrates the use of the `NeoPDF` C++ OOP API to load both single and multiple PDF
members, evaluate parton distributions for a range of :math:`x` and :math:`Q^2` values, and compare results
to LHAPDF.

**Technical details:**

- The `NeoPDF` and `NeoPDFs` objects manage their own memory and automatically release resources
  when they go out of scope (RAII).
- The evaluation of :math:`x f(x, Q^2)` and :math:`\alpha_s(Q^2)` is vectorized over the input axes for
  efficiency.
- The code asserts that the results from `NeoPDF` and LHAPDF agree within a tight tolerance,
  providing a robust validation.

.. code-block:: cpp
  :linenos:

  #include <LHAPDF/PDF.h>
  #include <LHAPDF/GridPDF.h>
  #include <NeoPDF.hpp>
  #include <cassert>
  #include <cmath>
  #include <iomanip>
  #include <iostream>
  #include <string>
  #include <vector>

  using namespace neopdf;

  const double TOLERANCE= 1e-16;

  template<typename T>
  std::vector<T> geomspace(T start, T stop, int num, bool endpoint = false) {
      std::vector<T> result(num);

      if (num == 1) {
          result[0] = start;
          return result;
      }

      T log_start = std::log(start);
      T log_stop = std::log(stop);
      T step = (log_stop - log_start) / (endpoint ? (num - 1) : num);

      for (int i = 0; i < num; ++i) {
          result[i] = std::exp(log_start + i * step);
      }

      return result;
  }

  template<typename T>
  std::vector<T> linspace(T start, T stop, int num, bool endpoint = true) {
      std::vector<T> result(num);

      if (num == 1) {
          result[0] = start;
          return result;
      }

      T step = (stop - start) / (endpoint ? (num - 1) : num);

      for (int i = 0; i < num; ++i) {
          result[i] = start + i * step;
      }

      return result;
  }

  void test_xfxq2() {
      std::cout << "=== Test xfxQ2 for single PDF member ===\n";

      // disable LHAPDF banners to guarantee deterministic output
      LHAPDF::setVerbosity(0);

      std::string pdfname = "NNPDF40_nnlo_as_01180";
      NeoPDF neo_pdf(pdfname.c_str(), 0);
      const LHAPDF::PDF* basepdf = LHAPDF::mkPDF(pdfname);
      const LHAPDF::GridPDF& lha_pdf = * dynamic_cast<const LHAPDF::GridPDF*>(basepdf);

      std::vector<int> pids = {-5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5};
      std::vector<double> xs = geomspace(neo_pdf.x_min(), neo_pdf.x_max(), 200);
      std::vector<double> q2s = geomspace(neo_pdf.q2_min(), neo_pdf.q2_max(), 200);

      // Headers of the table to print the results
      std::cout << std::right
          << std::setw(6) << "pid"
          << std::setw(15) << "x"
          << std::setw(15) << "Q2"
          << std::setw(15) << "LHAPDF"
          << std::setw(15) << "NeoPDF"
          << std::setw(15) << "Rel. Diff." << "\n";
      std::cout << std::string(81, '-') << "\n";

      for (const auto &pid: pids) {
          for (const auto &x: xs) {
              for (const auto &q2: q2s) {
                  double expected = lha_pdf.xfxQ2(pid, x, q2);
                  double result = neo_pdf.xfxQ2(pid, x, q2);
                  double reldif = std::abs(result - expected) / expected;

                  assert(std::abs(result - expected) < TOLERANCE);

                  // Print the results as a table
                  std::cout << std::scientific << std::setprecision(8)
                      << std::right
                      << std::setw(6)  << pid
                      << std::setw(15) << x
                      << std::setw(15) << q2
                      << std::right
                      << std::setw(15) << expected
                      << std::setw(15) << result
                      << std::setw(15) << reldif << "\n";
                  }
              }

      }
  }

  void test_alphas_q2() {
      std::cout << "=== Test alphasQ2 for single PDF member ===\n";

      // disable LHAPDF banners to guarantee deterministic output
      LHAPDF::setVerbosity(0);

      std::string pdfname = "NNPDF40_nnlo_as_01180";
      NeoPDF neo_pdf(pdfname.c_str(), 0);
      const LHAPDF::PDF* basepdf = LHAPDF::mkPDF(pdfname);
      const LHAPDF::GridPDF& lha_pdf = * dynamic_cast<const LHAPDF::GridPDF*>(basepdf);

      std::vector<double> q2_points = linspace(4.0, 1e10, 500);

      // Headers of the table to print the results
      std::cout << std::right
          << std::setw(15) << "Q2"
          << std::setw(15) << "LHAPDF"
          << std::setw(15) << "NeoPDF"
          << std::setw(15) << "Rel. Diff." << "\n";
      std::cout << std::string(60, '-') << "\n";

      for (const auto& q2: q2_points) {
          double expected = lha_pdf.alphasQ2(q2);
          double result = neo_pdf.alphasQ2(q2);
          double reldif = std::abs(result - expected) / expected;

          assert(std::abs(result - expected) < TOLERANCE);

          // Print the results as a table
          std::cout << std::scientific << std::setprecision(8)
              << std::right
              << std::setw(15) << q2
              << std::right
              << std::setw(15) << expected
              << std::setw(15) << result
              << std::setw(15) << reldif << "\n";
      }
  }

  void test_all_pdf_members() {
      std::cout << "=== Test PDFs class (loading all members) ===\n";

      // disable LHAPAPDF banners to guarantee deterministic output
      LHAPDF::setVerbosity(0);

      std::string pdfname = "NNPDF40_nnlo_as_01180";
      NeoPDFs neo_pdfs(pdfname.c_str());

      std::cout << "Loaded " << neo_pdfs.size() << " PDF members\n";

      // Test case: evaluate a simple point across all members
      int pid = 1;
      double x = 1e-9;
      double q2 = 1.65 * 1.65;

      std::cout << std::right
          << std::setw(8) << "Member"
          << std::setw(15) << "LHAPDF"
          << std::setw(15) << "NeoPDF"
          << std::setw(15) << "Rel. Diff." << "\n";
      std::cout << std::string(53, '-') << "\n";

      // Evaluate the same point across all PDF members
      std::vector<double> results;
      for (size_t i = 0; i < neo_pdfs.size(); ++i) {
          const LHAPDF::PDF* basepdf = LHAPDF::mkPDF(pdfname, i);
          const LHAPDF::GridPDF& lha_pdf = * dynamic_cast<const LHAPDF::GridPDF*>(basepdf);

          double expected = lha_pdf.xfxQ2(pid, x, q2);
          double result = neo_pdfs[i].xfxQ2(pid, x, q2);

          double reldif = std::abs(result - expected) / expected;
          assert(std::abs(result - expected) < TOLERANCE);
          results.push_back(result);

          std::cout << std::right
              << std::setw(8) << i
              << std::scientific << std::setprecision(8)
              << std::setw(15) << expected
              << std::setw(15) << result
              << std::setw(15) << reldif << "\n";
      }

      // Calculate some statistics
      double sum = 0.0;
      for (double result : results) {
          sum += result;
      }
      double mean = sum / results.size();

      double variance = 0.0;
      for (double result : results) {
          variance += (result - mean) * (result - mean);
      }
      variance /= results.size();
      double std_dev = std::sqrt(variance);

      std::cout << "\nStatistics across all members:\n";
      std::cout << "Mean: " << std::scientific << std::setprecision(8) << mean << "\n";
      std::cout << "Std Dev: " << std_dev << "\n";
      std::cout << "Relative Std Dev: " << std_dev / mean << "\n";
  }

  void test_lazy_loading() {
      std::cout << "=== Test NeoPDFLazy class (lazy loading) ===\n";

      // Disable LHAPDF banners to guarantee deterministic output
      LHAPDF::setVerbosity(0);

      std::string pdfname = "NNPDF40_nnlo_as_01180.neopdf.lz4";
      NeoPDFLazy lazy_pdfs(pdfname);

      std::cout << "Initialized lazy loader for " << pdfname << "\n";

      // Test case: evaluate a simple point across all members
      int pid = 1;
      double x = 1e-9;
      double q2 = 1.65 * 1.65;

      std::cout << std::right
          << std::setw(8) << "Member"
          << std::setw(15) << "LHAPDF"
          << std::setw(15) << "NeoPDF"
          << std::setw(15) << "Rel. Diff." << "\n";
      std::cout << std::string(53, '-') << "\n";

      // Evaluate the same point across all PDF members
      std::vector<double> results;
      int member_idx = 0;
      while (auto neo_pdf = lazy_pdfs.next()) {
          const LHAPAPDF::PDF* basepdf = LHAPDF::mkPDF("NNPDF40_nnlo_as_01180", member_idx);
          const LHAPDF::GridPDF& lha_pdf = *dynamic_cast<const LHAPDF::GridPDF*>(basepdf);

          double expected = lha_pdf.xfxQ2(pid, x, q2);
          double result = neo_pdf->xfxQ2(pid, x, q2);

          double reldif = std::abs(result - expected) / expected;
          assert(std::abs(result - expected) < TOLERANCE);
          results.push_back(result);

          std::cout << std::right
              << std::setw(8) << member_idx
              << std::scientific << std::setprecision(8)
              << std::setw(15) << expected
              << std::setw(15) << result
              << std::setw(15) << reldif << "\n";
          member_idx++;
      }

      std::cout << "\nSuccessfully iterated through all members lazily.\n";
  }

  int main() {
      // Test the computation of the PDF interpolations
      test_xfxq2();

      // Test the computation of the `alphas` interpolations
      test_alphas_q2();

      // Test the PDF interpolations by loading all the members
      test_all_pdf_members();

      // Test the lazy loading of PDF members
      test_lazy_loading();

      return EXIT_SUCCESS;
  }


Example 2: Filling and Writing a NeoPDF Grid
********************************************

This example illustrates how to fill and write a `NeoPDF` grid using the C++ OOP API. It
demonstrates the process of constructing a grid for each PDF member and serializing the
collection to disk.

The filling of the PDF grid in the following example assumes no dependence in the nucleon
numbers :math:`A` and strong coupling :math:`\alpha_s` (standard LHAPDF-like PDF). Refer to the Section
below in the case the grid should explicitly depend on more parameters.

**Technical details:**

- The grid axes are defined as vectors for :math:`x`, :math:`Q^2`, parton IDs, nucleons, and :math:`\alpha_s`
  values.
- The grid data is stored in a 6D array, with the layout ``[nucleons][alphas][pids][kT][xs][q2s]``.
- The ``GridWriter`` class manages the collection of grids and handles compression and serialization to disk.
- Metadata is filled in a ``MetaData`` object, which includes information about the set, axis ranges, flavors,
  and interpolation type. This metadata is essential for correct interpretation of the grid file.
- All memory management is automatic; no manual deallocation is required.
- The output file is compressed and written in the ``.neopdf.lz4`` format, suitable for use with `NeoPDF` (CLI)
  tools and APIs.


.. code-block:: cpp
  :linenos:

  #include <NeoPDF.hpp>
  #include <cassert>
  #include <cmath>
  #include <cstdio>
  #include <cstdlib>
  #include <cstring>
  #include <iostream>
  #include <vector>

  using namespace neopdf;

  const double TOLERANCE= 1e-16;

  int main() {
      const char* pdfname = "NNPDF40_nnlo_as_01180";
      // Load all PDF members
      NeoPDFs neo_pdfs(pdfname);
      if (neo_pdfs.size() == 0) {
          std::cerr << "Failed to load any PDF members!\n";
          return 1;
      }
      std::cout << "Loaded " << neo_pdfs.size() << " PDF members\n";

      // Get the first PDF as a reference for metadata
      NeoPDF& ref_pdf = neo_pdfs[0];

      // Extract the PID values of the PDF set
      auto pids = ref_pdf.pids();

      // Extract the number of subgrids
      std::size_t num_subgrids = ref_pdf.num_subgrids();

      // Create a grid writer
      GridWriter writer;

      // For each member, build a grid
      for (size_t m = 0; m < neo_pdfs.size(); ++m) {
          NeoPDF& pdf = neo_pdfs[m];

          // Start a new grid for the current member
          writer.new_grid();

          // Loop over the Subgrids
          for (std::size_t subgrid_idx = 0; subgrid_idx != num_subgrids; subgrid_idx++) {
              // Extract the knot values of the parameters for the subgrid
              auto xs = pdf.subgrid_for_param(NEOPDF_SUBGRID_PARAMS_MOMENTUM, subgrid_idx);
              auto q2s = pdf.subgrid_for_param(NEOPDF_SUBGRID_PARAMS_SCALE, subgrid_idx);
              auto alphas = pdf.subgrid_for_param(NEOPDF_SUBGRID_PARAMS_ALPHAS, subgrid_idx);
              auto nucleons = pdf.subgrid_for_param(NEOPDF_SUBGRID_PARAMS_NUCLEONS, subgrid_idx);
              auto kts = pdf.subgrid_for_param(NEOPDF_SUBGRID_PARAMS_KT, subgrid_idx);

              // Compute grid_data: [q2][x][flavor], instead of [nucleon][alphas][kt][q2][x][flavor]
              // NOTE: This assumes that there is no 'A' and `alphas` dependence.
              std::vector<double> grid_data;
              for (double x : xs) {
                  for (double q2 : q2s) {
                      for (int pid : pids) {
                          double val = pdf.xfxQ2(pid, x, q2);
                          grid_data.push_back(val);
                      }
                  }
              }

              // Add subgrid
              writer.add_subgrid(
                  nucleons,
                  alphas,
                  kts,
                  xs,
                  q2s,
                  grid_data
              );
          }

          // Finalize the Grid (inc. its subgrids) for this member.
          writer.push_grid(pids);
          std::cout << "Added grid for member " << m << "\n";
      }

      // Fill the running of alphas with some random values
      std::vector<double> alphas_qs = {2.0};
      std::vector<double> alphas_vals = {0.118};

      // Extract the ranges for the momentum x and scale Q2
      auto x_range = ref_pdf.param_range(NEOPDF_SUBGRID_PARAMS_MOMENTUM);
      auto q2_range = ref_pdf.param_range(NEOPDF_SUBGRID_PARAMS_SCALE);

      PhysicsParameters phys_params = {
          .flavor_scheme = "variable",
          .order_qcd = 2,
          .alphas_order_qcd = 2,
          .m_w = 80.352,
          .m_z = 91.1876,
          .m_up = 0.0,
          .m_down = 0.0,
          .m_strange = 0.0,
          .m_charm = 1.51,
          .m_bottom = 4.92,
          .m_top = 172.5,
          .alphas_type = "ipol",
          .number_flavors = 4,
      };

      MetaData meta = {
          .set_desc = "NNPDF40_nnlo_as_01180 collection",
          .set_index = 0,
          .num_members = (uint32_t)neo_pdfs.size(),
          .x_min = x_range[0],
          .x_max = x_range[1],
          .q_min = sqrt(q2_range[0]),
          .q_max = sqrt(q2_range[1]),
          .flavors = pids,
          .format = "neopdf",
          .alphas_q_values = alphas_qs,
          .alphas_vals = alphas_vals,
          .polarised = false,
          .set_type = NEOPDF_SET_TYPE_SPACE_LIKE,
          .interpolator_type = NEOPDF_INTERPOLATOR_TYPE_LOG_BICUBIC,
          .error_type = "replicas",
          .hadron_pid = 2212,
          .phys_params = phys_params,
      };

      // Check if `NEOPDF_DATA_PATH` is defined and store the Grid there.
      const char* filename = "check-writer-oop.neopdf.lz4";
      const char* neopdf_path = std::getenv("NEOPDF_DATA_PATH");
      std::string output_path = neopdf_path
          ? std::string(neopdf_path) + (std::string(neopdf_path).back() == '/' ? "" : "/") + filename
          : filename;

      // Write the PDF Grid into disk
      try {
          writer.compress(meta, output_path);
          std::cout << "Compression succeeded!\n";
      } catch (const std::runtime_error& err) {
          std::cerr << "Compression failed: " << err.what() << "\n";
          return EXIT_FAILURE;
      }

      // If `NEOPDF_DATA_PATH` is defined, reload the grid and check ther results.
      if (neopdf_path) {
          int pid_test = 21;
          double x_test = 1e-3;
          double q2_test1 = 1e2;
          double q2_test2 = 1e4;

          double ref1 = neo_pdfs[0].xfxQ2(pid_test, x_test, q2_test1);
          double ref2 = neo_pdfs[0].xfxQ2(pid_test, x_test, q2_test2);

          NeoPDF wpdf(filename);
          double res1 = wpdf.xfxQ2(pid_test, x_test, q2_test1);
          double res2 = wpdf.xfxQ2(pid_test, x_test, q2_test2);

          assert(std::abs(res1 - ref1) < TOLERANCE);
          assert(std::abs(res2 - ref2) < TOLERANCE);
      }

      // Clip the interpolated values to be zero if negatives.
      neo_pdfs[0].set_force_positive(NEOPDF_FORCE_POSITIVE_CLIP_NEGATIVE);
      assert(neo_pdfs[0].is_force_positive() == NEOPDF_FORCE_POSITIVE_CLIP_NEGATIVE);

      // Clip all the PDF members to be positive definite
      neo_pdfs.set_force_positive_members(NEOPDF_FORCE_POSITIVE_CLIP_SMALL);
      assert(neo_pdfs[4].is_force_positive() == NEOPDF_FORCE_POSITIVE_CLIP_SMALL);

      return EXIT_SUCCESS;
  }


Example 3: Filling TMD grids with :math:`k_T` Dependence
********************************************************

In the following example, we are going to see how to fill TMD grids which contains
a dependence on the transverse momentum :math:`k_T`. The following example makes use of
the `TMDlib <https://tmdlib.hepforge.org/>`_ library to provide the TMD distributions.

.. code-block:: cpp
  :linenos:

  #include "neopdf_capi.h"
  #include "tmdlib/TMDlib.h"
  #include <NeoPDF.hpp>
  #include <algorithm>
  #include <cassert>
  #include <cmath>
  #include <cstddef>
  #include <cstdlib>
  #include <fstream>
  #include <initializer_list>
  #include <string>
  #include <sstream>
  #include <tmdlib/factories.h>
  #include <vector>

  using namespace TMDlib;
  using namespace neopdf;

  const double TOLERANCE = 1e-16;

  struct Kinematics {
      std::vector<double> xs;
      std::vector<double> kts;
      std::vector<double> qs;
  };

  std::vector<double> compute_tmds(TMD& tmd, double x, double kt, double q2) {
      double xbar = 0.0;
      double mu = sqrt(q2);
      std::vector<double> pdfs = tmd.TMDpdf(x, xbar, kt, mu);

      return pdfs;
  }

  std::vector<double> parse_array(const std::string& line) {
      std::vector<double> result;
      size_t start = line.find('[');
      size_t end = line.find(']');

      if (start == std::string::npos || end == std::string::npos) {
          return result;
      }

      double value;
      std::string content = line.substr(start + 1, end - start - 1);
      std::replace(content.begin(), content.end(), ',', ' ');
      std::stringstream ss(content);
      while (ss >> value) { result.push_back(value); }

      return result;
  }

  Kinematics read_kinematics() {
      // Parse the Kinematics separately as it is difficult to retrieve
      std::ifstream input_kins("MAP22_N3LL.kinematics");

      if (!input_kins.is_open()) {
          input_kins.open("raw.data");
          if (!input_kins.is_open()) {
              return {{}, {}, {}};
          }
      }

      std::string line;
      std::vector<double> xs, kts, qs;
      while (std::getline(input_kins, line)) {
          if (line.find("qToQg:") != std::string::npos) { kts = parse_array(line); }
          else if (line.find("Qg:") != std::string::npos) { qs = parse_array(line); }
          else if (line.find("xg:") != std::string::npos) { xs = parse_array(line); }
      }
      input_kins.close();

      return { xs, kts, qs };
  }

  int main() {
      std::string setname = "MAP22_grids_FF_Km_N3LL";

      TMD tmd;
      tmd.setVerbosity(0);

      // Extract various informations from the TMD
      tmd.TMDinit(setname);
      std::size_t n_members = tmd.TMDgetNumMembers();
      double xmin = tmd.TMDgetXmin();
      double xmax = tmd.TMDgetXmax();
      double q2min = tmd.TMDgetQ2min();
      double q2max = tmd.TMDgetQ2max();

      // Define the kinematics
      Kinematics kins = read_kinematics();
      std::vector<double> xs = kins.xs;
      std::vector<double> qs = kins.qs;
      std::vector<double> kts = kins.kts;

      // Square the energy scale Q
      std::vector<double> q2s(qs.size());
      for (size_t i = 0; i < qs.size(); ++i) { q2s[i] = qs[i] * qs[i]; }

      // Define some physical parameters
      std::vector<int> pids = {-6, -5, -4, -3, -2, -1, 21, 1, 2, 3, 4, 5, 6};
      std::vector<double> nucleons = { 1.0 }; // assume to be a Proton
      std::vector<double> alphas = { 0.118 }; // assume to be determined with as=0.118

      // Instantiate NeoPDF grid writer
      GridWriter neopdf_writer;

      for (std::size_t m = 0; m != n_members; m++) {
          tmd.TMDinit(setname, m);
          std::cout << "Member " << m << " loaded!\n";

          // Start a new grid for the current member
          neopdf_writer.new_grid();

          std::vector<double> grid_data;
          for (double kt : kts) {
              for (double x : xs) {
                  for (double q2 : q2s) {
                      std::vector<double> tmd_pds = tmd.TMDpdf(x, 0.0, kt, sqrt(q2));
                      for (std::size_t pid_idx = 0; pid_idx != pids.size(); pid_idx++) {
                          grid_data.push_back(tmd_pds[pid_idx]);
                      }
                  }
              }
          }

          // Add subgrid member to the Grid
          neopdf_writer.add_subgrid(
              nucleons,
              alphas,
              kts,
              xs,
              q2s,
              grid_data
          );

          // Finalize the Grid (inc. its subgrids) for this member.
          neopdf_writer.push_grid(pids);
      }

      // Fill the running of alphas with some random values
      std::vector<double> alphas_qs = { 91.1876 };
      std::vector<double> alphas_vals = { 0.118 };

      // Construct the Metadata
      PhysicsParameters phys_params = {
          .flavor_scheme = "fixed",
          .order_qcd = 2,
          .alphas_order_qcd = 2,
          .m_w = 80.352,
          .m_z = 91.1876,
          .m_up = 0.0,
          .m_down = 0.0,
          .m_strange = 0.0,
          .m_charm = 1.51,
          .m_bottom = 4.92,
          .m_top = 172.5,
          .alphas_type = "ipol",
          .number_flavors = 4,
      };

      MetaData meta = {
          .set_desc = "NNPDF40_nnlo_as_01180 collection",
          .set_index = 0,
          .num_members = (uint32_t)n_members,
          .x_min = xmin,
          .x_max = xmax,
          .q_min = sqrt(q2min),
          .q_max = sqrt(q2max),
          .flavors = pids,
          .format = "neopdf",
          .alphas_q_values = alphas_qs,
          .alphas_vals = alphas_vals,
          .polarised = false,
          .set_type = NEOPDF_SET_TYPE_SPACE_LIKE,
          .interpolator_type = NEOPDF_INTERPOLATOR_TYPE_LOG_TRICUBIC,
          .error_type = "replicas",
          .hadron_pid = 2212,
          .phys_params = phys_params,
      };

      // Check if `NEOPDF_DATA_PATH` is defined and store the Grid there.
      const char* filename = "check-tmds.neopdf.lz4";
      const char* neopdf_path = std::getenv("NEOPDF_DATA_PATH");
      std::string output_path = neopdf_path
          ? std::string(neopdf_path) + (std::string(neopdf_path).back() == '/' ? "" : "/") + filename
          : filename;

      // Write the PDF Grid into disk
      try {
          neopdf_writer.compress(meta, output_path);
          std::cout << "Compression succeeded!\n";
      } catch (const std::runtime_error& err) {
          std::cerr << "Compression failed: " << err.what() << "\n";
          return EXIT_FAILURE;
      }


      // If `NEOPDF_DATA_PATH` is defined, reload the grid and check ther results.
      if (neopdf_path) {
          int irep = 12;
          int pid_test_idx = 2;
          double x_test = xs[20];
          double q_test = qs[25];

          // Re-load the NeoPDF and TMDLib TMD sets
          NeoPDF wpdf(filename, irep);
          tmd.TMDinit(setname, irep);

          for (double kt : kts) {
              std::vector<double> refs = tmd.TMDpdf(x_test, 0.0, kt, q_test);
              double ref = refs[pid_test_idx]; // Up Quark

              std::vector<double> params = { kt,x_test, q_test * q_test };
              double res = wpdf.xfxQ2_ND(pids[pid_test_idx], params);

              assert(std::abs(ref - res) < TOLERANCE);
          }
      }

      return EXIT_SUCCESS;
  }
