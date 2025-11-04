#pragma once

#include <cstddef>
#include <cstdint>
#include <neopdf_capi.h>
#include <string>
#include <sys/types.h>
#include <vector>
#include <memory>
#include <stdexcept>

/** @brief Object Oriented interface to NeoPDF. */
namespace neopdf {

/** @brief C++ representation of PhysicsParameters. */
struct PhysicsParameters {
    std::string flavor_scheme;
    uint32_t order_qcd;
    uint32_t alphas_order_qcd;
    double m_w;
    double m_z;
    double m_up;
    double m_down;
    double m_strange;
    double m_charm;
    double m_bottom;
    double m_top;
    std::string alphas_type;
    uint32_t number_flavors;

    // Conversion to C struct
    NeoPDFPhysicsParameters to_c() const {
        NeoPDFPhysicsParameters c_params;
        c_params.flavor_scheme = flavor_scheme.c_str();
        c_params.order_qcd = order_qcd;
        c_params.alphas_order_qcd = alphas_order_qcd;
        c_params.m_w = m_w;
        c_params.m_z = m_z;
        c_params.m_up = m_up;
        c_params.m_down = m_down;
        c_params.m_strange = m_strange;
        c_params.m_charm = m_charm;
        c_params.m_bottom = m_bottom;
        c_params.m_top = m_top;
        c_params.alphas_type = alphas_type.c_str();
        c_params.number_flavors = number_flavors;
        return c_params;
    }
};

/** @brief C++ representation of NeoPDFMetaData. */
struct MetaData {
    std::string set_desc;
    uint32_t set_index;
    uint32_t num_members;
    double x_min;
    double x_max;
    double q_min;
    double q_max;
    std::vector<int32_t> flavors;
    std::string format;
    std::vector<double> alphas_q_values;
    std::vector<double> alphas_vals;
    bool polarised;
    neopdf_set_type set_type;
    neopdf_interpolator_type interpolator_type;
    std::string error_type;
    int32_t hadron_pid;
    PhysicsParameters phys_params;

    // Conversion to C struct
    NeoPDFMetaData to_c() const {
        NeoPDFMetaData c_meta;
        c_meta.set_desc = set_desc.c_str();
        c_meta.set_index = set_index;
        c_meta.num_members = num_members;
        c_meta.x_min = x_min;
        c_meta.x_max = x_max;
        c_meta.q_min = q_min;
        c_meta.q_max = q_max;
        c_meta.flavors = flavors.data();
        c_meta.num_flavors = flavors.size();
        c_meta.format = format.c_str();
        c_meta.alphas_q_values = alphas_q_values.data();
        c_meta.num_alphas_q = alphas_q_values.size();
        c_meta.alphas_vals = alphas_vals.data();
        c_meta.num_alphas_vals = alphas_vals.size();
        c_meta.polarised = polarised;
        c_meta.set_type = set_type;
        c_meta.interpolator_type = interpolator_type;
        c_meta.error_type = error_type.c_str();
        c_meta.hadron_pid = hadron_pid;
        c_meta.phys_params = phys_params.to_c();
        return c_meta;
    }
};

/** @brief C++ representation of NeoPDFMetaDataV2. */
struct MetaDataV2 : public MetaData {
    double xi_min = 1.0;
    double xi_max = 1.0;
    double delta_min = 0.0;
    double delta_max = 0.0;

    // Conversion to C struct
    NeoPDFMetaDataV2 to_c_v2() const {
        NeoPDFMetaDataV2 c_meta;
        c_meta.set_desc = set_desc.c_str();
        c_meta.set_index = set_index;
        c_meta.num_members = num_members;
        c_meta.x_min = x_min;
        c_meta.x_max = x_max;
        c_meta.q_min = q_min;
        c_meta.q_max = q_max;
        c_meta.flavors = flavors.data();
        c_meta.num_flavors = flavors.size();
        c_meta.format = format.c_str();
        c_meta.alphas_q_values = alphas_q_values.data();
        c_meta.num_alphas_q = alphas_q_values.size();
        c_meta.alphas_vals = alphas_vals.data();
        c_meta.num_alphas_vals = alphas_vals.size();
        c_meta.polarised = polarised;
        c_meta.set_type = set_type;
        c_meta.interpolator_type = interpolator_type;
        c_meta.error_type = error_type.c_str();
        c_meta.hadron_pid = hadron_pid;
        c_meta.phys_params = phys_params.to_c();
        c_meta.xi_min = xi_min;
        c_meta.xi_max = xi_max;
        c_meta.delta_min = delta_min;
        c_meta.delta_max = delta_max;
        return c_meta;
    }
};

class NeoPDFs; // Forward declaration

/** @brief Base PDF class that instantiates the PDF object. */
class NeoPDF {
    friend class NeoPDFs; // Grant NeoPDFs access to private members
    private:
        /** @brief Underlying raw object. */
        NeoPDFWrapper* raw;

    protected:
        /** @brief Constructor (protected to avoid direct instantiation). */
        NeoPDF(NeoPDFWrapper* pdf) : raw(pdf) {}

        /** @brief Deleted copy/move semantics. */
        NeoPDF() = delete;
        NeoPDF(const NeoPDF&) = delete;
        NeoPDF(NeoPDF&&) = delete;

        NeoPDF& operator=(const NeoPDF&) = delete;
        NeoPDF& operator=(NeoPDF&&) = delete;

    public:
        /** @brief Destructor. */
        virtual ~NeoPDF() { neopdf_pdf_free(this->raw); }

        /**
         * @brief Constructor of the PDF object.
         * @brief `pdf_name` Name of the PDF set.
         * @brief `member` ID number of the PDF member.
         */
        NeoPDF(const std::string& pdf_name, size_t member = 0) {
            this->raw = neopdf_pdf_load(pdf_name.c_str(), member);
        }

        // Needed for `PDFs` to call the protected constructor
        // Static factory method to create PDF objects from NeoPDFWrapper*
        static std::unique_ptr<NeoPDF> from_raw(NeoPDFWrapper* pdf) {
            return std::unique_ptr<NeoPDF>(new NeoPDF(pdf));
        }

        /** @brief Get the minimum value of the x-grid for the PDF. */
        double x_min() const { return neopdf_pdf_x_min(this->raw); }

        /** @brief Get the maximum value of the x-grid for the PDF. */
        double x_max() const { return neopdf_pdf_x_max(this->raw); }

        /** @brief Get the minimum value of the Q2-grid for the PDF. */
        double q2_min() const { return neopdf_pdf_q2_min(this->raw); }

        /** @brief Get the maximum value of the Q2-grid for the PDF. */
        double q2_max() const { return neopdf_pdf_q2_max(this->raw); }

        /** @brief Compute the `xf` value for a given PID, x, and Q2. */
        double xfxQ2(int pid, double x, double q2) const {
            return neopdf_pdf_xfxq2(this->raw, pid, x, q2);
        }

        /** @brief Compute the `xf` value for a generic set of parameters. */
        double xfxQ2_ND(int pid, std::vector<double> params) const {
            return neopdf_pdf_xfxq2_nd(this->raw, pid, params.data(), params.size());
        }

        /** @brief Compute the `xf` value for a generic set of parameters using batch Chebyshev interpolation. */
        std::vector<double>
        xfxQ2_cheby_batch(int pid, const std::vector<std::vector<double>> &points) const {
            std::vector<const double *> c_points(points.size());
            std::vector<size_t> lengths(points.size());
            for (size_t i = 0; i < points.size(); ++i) {
                c_points[i] = points[i].data();
                lengths[i] = points[i].size();
            }

            std::vector<double> results(points.size());
            neopdf_pdf_xfxq2_cheby_batch(this->raw, pid, c_points.data(), lengths.data(),
                                        points.size(), results.data());

            return results;
        }

        /** @brief Compute the value of `alphas` at the Q2 value. */
        double alphasQ2(double q2) const {
            return neopdf_pdf_alphas_q2(this->raw, q2);
        }

        /** @brief Get the number of PIDs. */
        size_t num_pids() const {
            return neopdf_pdf_num_pids(this->raw);
        }

        /** @brief Get the PID representation of the PDF Grid. */
        std::vector<int32_t> pids() const {
            size_t num = num_pids();
            std::vector<int32_t> pids(num);
            neopdf_pdf_pids(this->raw, pids.data(), num);
            return pids;
        }

        /** @brief Get the number of subgrids in the PDF Grid. */
        size_t num_subgrids() const {
            return neopdf_pdf_num_subgrids(this->raw);
        }

        /** @brief Get the minimum and maximum value for a given parameter. */
        std::vector<double> param_range(NeopdfSubgridParams param) const {
            std::vector<double> range(2);
            neopdf_pdf_param_range(this->raw, param, range.data());
            return range;
        }

        /** @brief Get the shape of the subgrids in the order of their index for a given parameter. */
        std::vector<size_t> subgrids_shape_for_param(NeopdfSubgridParams param) const {
            size_t num = num_subgrids();
            std::vector<size_t> shape(num);
            neopdf_pdf_subgrids_shape_for_param(this->raw, shape.data(), num, param);
            return shape;
        }

        /** @brief Get the grid values of a parameter for a given subgrid. */
        std::vector<double> subgrid_for_param(NeopdfSubgridParams param, size_t subgrid_index) const {
            std::vector<size_t> shape = subgrids_shape_for_param(param);
            std::vector<double> values(shape[subgrid_index]);
            neopdf_pdf_subgrids_for_param(
                this->raw,
                values.data(),
                param,
                shape.size(),
                shape.data(),
                subgrid_index
            );
            return values;
        }

        /** @brief Clip the interpolated values if they turned out negatives. */
        void set_force_positive(neopdf_force_positive option) {
            neopdf_pdf_set_force_positive(this->raw, option);
        }

        /** @brief Returns the value of `ForcePositive` defining the PDF grid. */
        neopdf_force_positive is_force_positive() const {
            return neopdf_pdf_is_force_positive(this->raw);
        }
};

/** @brief Class to load and manage multiple PDF members. */
class NeoPDFs {
    private:
        std::vector<std::unique_ptr<NeoPDF>> pdf_members;

    public:
        /**
         * @brief Constructor that loads all PDF members for a given PDF set.
         * @param pdf_name Name of the PDF set.
         */
        NeoPDFs(const std::string& pdf_name) {
            NeoPDFMembers raw_pdfs = neopdf_pdf_load_all(pdf_name.c_str());

            for (size_t i = 0; i < raw_pdfs.size; ++i) {
                pdf_members.push_back(NeoPDF::from_raw(raw_pdfs.pdfs[i]));
            }
        }

        /** @brief Get the number of loaded PDF members. */
        size_t size() const { return pdf_members.size(); }

        /** @brief Access a specific PDF member by index. */
        NeoPDF& operator[](size_t index) { return *pdf_members[index]; }

        /** @brief Access a specific PDF member by index (const version). */
        const NeoPDF& operator[](size_t index) const { return *pdf_members[index]; }

        /** @brief Access a specific PDF member by index with bounds checking. */
        NeoPDF& at(size_t index) { return *pdf_members.at(index); }

        /** @brief Access a specific PDF member by index with bounds checking (const version). */
        const NeoPDF& at(size_t index) const { return *pdf_members.at(index); }

        /** @brief Clip the interpolated values if they turned out negatives for all members. */
        void set_force_positive_members(neopdf_force_positive option) {
            NeoPDFMembers members;
            members.size = pdf_members.size();
            std::vector<NeoPDFWrapper*> raw_pdfs;
            for (const auto& pdf : pdf_members) {
                raw_pdfs.push_back(pdf->raw);
            }
            members.pdfs = raw_pdfs.data();
            neopdf_pdf_set_force_positive_members(&members, option);
        }
};

/** @brief Class for lazily loading PDF members from a .neopdf.lz4 file. */
class NeoPDFLazy {
    private:
        ::NeoPDFLazyIterator* raw_iter;

    public:
        /**
         * @brief Constructor that initializes the lazy iterator for a given PDF set.
         * @param pdf_name Name of the PDF set (must be a .neopdf.lz4 file).
         * @throws std::runtime_error if the iterator cannot be created.
         */
        explicit NeoPDFLazy(const std::string& pdf_name) {
            raw_iter = neopdf_pdf_load_lazy(pdf_name.c_str());
            if (!raw_iter) {
                throw std::runtime_error("Failed to create lazy iterator. Check if file is a .neopdf.lz4 file.");
            }
        }

        /** @brief Destructor. */
        ~NeoPDFLazy() {
            if (raw_iter) {
                neopdf_lazy_iterator_free(raw_iter);
            }
        }

        /** @brief Move constructor. */
        NeoPDFLazy(NeoPDFLazy&& other) noexcept : raw_iter(other.raw_iter) {
            other.raw_iter = nullptr;
        }

        /** @brief Move assignment operator. */
        NeoPDFLazy& operator=(NeoPDFLazy&& other) noexcept {
            if (this != &other) {
                if (raw_iter) {
                    neopdf_lazy_iterator_free(raw_iter);
                }
                raw_iter = other.raw_iter;
                other.raw_iter = nullptr;
            }
            return *this;
        }

        /** @brief Deleted copy semantics. */
        NeoPDFLazy(const NeoPDFLazy&) = delete;
        NeoPDFLazy& operator=(const NeoPDFLazy&) = delete;

        /**
         * @brief Get the next PDF member from the iterator.
         * @return A unique_ptr to the NeoPDF object, or nullptr if the iteration is complete.
         */
        std::unique_ptr<NeoPDF> next() {
            if (!raw_iter) {
                return nullptr;
            }
            NeoPDFWrapper* pdf_raw = neopdf_lazy_iterator_next(raw_iter);
            if (pdf_raw) {
                return NeoPDF::from_raw(pdf_raw);
            }
            return nullptr;
        }
};

/** @brief Class for writing NeoPDF grid data to a file. */
class GridWriter {
    private:
        NeoPDFGridArrayCollection* collection_raw;
        NeoPDFGrid* current_grid;

    public:
        /** @brief Constructor. */
        GridWriter() : current_grid(nullptr) {
            collection_raw = neopdf_gridarray_collection_new();
            if (!collection_raw) {
                throw std::runtime_error("Failed to create `NeoPDFGridArrayCollection`");
            }
        }

        /** @brief Destructor. */
        ~GridWriter() {
            if (collection_raw) {
                neopdf_gridarray_collection_free(collection_raw);
            }
            if (current_grid) {
                neopdf_grid_free(current_grid);
            }
        }

        /**
         * @brief Starts a new grid for a new member.
         */
        void new_grid() {
            if (current_grid) {
                // Free the previous grid if it was not pushed
                neopdf_grid_free(current_grid);
            }
            current_grid = neopdf_grid_new();
            if (!current_grid) {
                throw std::runtime_error("Failed to create `NeoPDFGrid`");
            }
        }

        /**
         * @brief Adds a subgrid to the current grid.
         *
         * @param nucleons Vector of nucleon numbers.
         * @param alphas Vector of alpha_s values.
         * @param kts Vector of kt values.
         * @param xs Vector of x values.
         * @param q2s Vector of Q2 values.
         * @param grid_data Vector of grid data.
         */
        void add_subgrid(
            const std::vector<double>& nucleons,
            const std::vector<double>& alphas,
            const std::vector<double>& kts,
            const std::vector<double>& xs,
            const std::vector<double>& q2s,
            const std::vector<double>& grid_data
        ) {
            if (!current_grid) {
                throw std::runtime_error("No grid started. Call new_grid() first.");
            }
            NeopdfResult result = neopdf_grid_add_subgrid(
                current_grid,
                nucleons.data(), nucleons.size(),
                alphas.data(), alphas.size(),
                kts.data(), kts.size(),
                xs.data(), xs.size(),
                q2s.data(), q2s.size(),
                grid_data.data(), grid_data.size()
            );
            if (result != NeopdfResult::NEOPDF_RESULT_SUCCESS) {
                throw std::runtime_error("Failed to add subgrid");
            }
        }

        /**
         * @brief Adds a subgrid to the current grid (v2 for 8D).
         *
         * @param nucleons Vector of nucleon numbers.
         * @param alphas Vector of alpha_s values.
         * @param xis Vector of xi values.
         * @param deltas Vector of delta values.
         * @param kts Vector of kt values.
         * @param xs Vector of x values.
         * @param q2s Vector of Q2 values.
         * @param grid_data Vector of grid data.
         */
        void add_subgrid_v2(
            const std::vector<double>& nucleons,
            const std::vector<double>& alphas,
            const std::vector<double>& xis,
            const std::vector<double>& deltas,
            const std::vector<double>& kts,
            const std::vector<double>& xs,
            const std::vector<double>& q2s,
            const std::vector<double>& grid_data
        ) {
            if (!current_grid) {
                throw std::runtime_error("No grid started. Call new_grid() first.");
            }
            NeopdfResult result = neopdf_grid_add_subgridv2(
                current_grid,
                nucleons.data(), nucleons.size(),
                alphas.data(), alphas.size(),
                xis.data(), xis.size(),
                deltas.data(), deltas.size(),
                kts.data(), kts.size(),
                xs.data(), xs.size(),
                q2s.data(), q2s.size(),
                grid_data.data(), grid_data.size()
            );
            if (result != NeopdfResult::NEOPDF_RESULT_SUCCESS) {
                throw std::runtime_error("Failed to add subgrid (v2)");
            }
        }

        /**
         * @brief Finalizes the current grid, sets its flavors, and adds it to the collection.
         *
         * @param flavors Vector of flavor IDs.
         */
        void push_grid(const std::vector<int32_t>& flavors) {
            if (!current_grid) {
                throw std::runtime_error("No grid to commit. Call new_grid() and add_subgrid() first.");
            }

            NeopdfResult result = neopdf_grid_set_flavors(current_grid, flavors.data(), flavors.size());
            if (result != NeopdfResult::NEOPDF_RESULT_SUCCESS) {
                neopdf_grid_free(current_grid);
                current_grid = nullptr;
                throw std::runtime_error("Failed to set flavors");
            }

            result = neopdf_gridarray_collection_add_grid(collection_raw, current_grid);
            if (result != NeopdfResult::NEOPDF_RESULT_SUCCESS) {
                neopdf_grid_free(current_grid);
                current_grid = nullptr;
                throw std::runtime_error("Failed to add grid to collection");
            }

            // The collection now owns the grid, so we release it from GridWriter.
            current_grid = nullptr;
        }

        /**
         * @brief Compresses the added grids and writes them to a file.
         *
         * @param metadata The metadata for the PDF set.
         * @param output_path The path to the output file.
         */
        void compress(const MetaData& metadata, const std::string& output_path) {
            if (current_grid) {
                neopdf_grid_free(current_grid);
                current_grid = nullptr;
                throw std::runtime_error("A grid was being built but was not committed before compress().");
            }

            NeoPDFMetaData c_meta = metadata.to_c();
            NeopdfResult result = neopdf_grid_compress(collection_raw, &c_meta, output_path.c_str());

            if (result != NeopdfResult::NEOPDF_RESULT_SUCCESS) {
                throw std::runtime_error("Failed to compress grid data");
            }
        }

        /**
         * @brief Compresses the added grids and writes them to a file using V2 metadata.
         *
         * @param metadata The V2 metadata for the PDF set.
         * @param output_path The path to the output file.
         */
        void compress_v2(const MetaDataV2& metadata, const std::string& output_path) {
            if (current_grid) {
                neopdf_grid_free(current_grid);
                current_grid = nullptr;
                throw std::runtime_error("A grid was being built but was not committed before compress().");
            }

            NeoPDFMetaDataV2 c_meta = metadata.to_c_v2();
            NeopdfResult result = neopdf_grid_compress_v2(collection_raw, &c_meta, output_path.c_str());

            if (result != NeopdfResult::NEOPDF_RESULT_SUCCESS) {
                throw std::runtime_error("Failed to compress grid data (v2)");
            }
        }
};

} // namespace neopdf

/** @brief LHAPDF compatibility for no-code migration. */
namespace NEOLHAPDF {

class PDF {
    protected:
        std::unique_ptr<neopdf::NeoPDF> _neopdf;

        PDF() = default;
        explicit PDF(std::unique_ptr<neopdf::NeoPDF>&& pdf) : _neopdf(std::move(pdf)) {}

    public:
        virtual ~PDF() = default;

        virtual double xfxQ2(int id, double x, double q2) const {
            return _neopdf->xfxQ2(id, x, q2);
        }

        virtual double alphasQ2(double q2) const {
            return _neopdf->alphasQ2(q2);
        }

        double xMin() const { return _neopdf->x_min(); }
        double xMax() const { return _neopdf->x_max(); }
        double q2Min() const { return _neopdf->q2_min(); }
        double q2Max() const { return _neopdf->q2_max(); }
};

class GridPDF : public PDF {
    public:
        explicit GridPDF(std::unique_ptr<neopdf::NeoPDF>&& pdf) : PDF(std::move(pdf)) {}
};

inline PDF* mkPDF(const std::string& name, int member = 0) {
    std::unique_ptr<neopdf::NeoPDF> neopdf_ptr(new neopdf::NeoPDF(name, member));
    return new GridPDF(std::move(neopdf_ptr));
}

inline void setVerbosity(int /*verbosity*/) { }

} // namespace LHAPDF
