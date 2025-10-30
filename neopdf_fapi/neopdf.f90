module neopdf
    use iso_c_binding
    use iso_fortran_env

    implicit none

    integer, parameter, private :: dp = kind(0.0d0)

    type neopdf_pdf
        type (c_ptr) :: ptr = c_null_ptr
    end type

    type neopdf_lazy_iterator
        type (c_ptr) :: ptr = c_null_ptr
    end type

    type neopdf_grid
        type (c_ptr) :: ptr = c_null_ptr
    end type

    type neopdf_grid_array_collection
        type (c_ptr) :: ptr = c_null_ptr
    end type

    type, bind(c) :: neopdf_physics_parameters
        type (c_ptr) :: flavor_scheme = c_null_ptr
        integer(c_int32_t) :: order_qcd
        integer(c_int32_t) :: alphas_order_qcd
        real(c_double) :: m_w
        real(c_double) :: m_z
        real(c_double) :: m_up
        real(c_double) :: m_down
        real(c_double) :: m_strange
        real(c_double) :: m_charm
        real(c_double) :: m_bottom
        real(c_double) :: m_top
        type (c_ptr) :: alphas_type = c_null_ptr
        integer(c_int32_t) :: number_flavors
    end type

    type, bind(c) :: neopdf_metadata
        type (c_ptr) :: set_desc = c_null_ptr
        integer(c_int32_t) :: set_index
        integer(c_int32_t) :: num_members
        real(c_double) :: x_min
        real(c_double) :: x_max
        real(c_double) :: q_min
        real(c_double) :: q_max
        type (c_ptr) :: flavors = c_null_ptr
        integer(c_size_t) :: num_flavors
        type (c_ptr) :: format = c_null_ptr
        type (c_ptr) :: alphas_q_values = c_null_ptr
        integer(c_size_t) :: num_alphas_q
        type (c_ptr) :: alphas_vals = c_null_ptr
        integer(c_size_t) :: num_alphas_vals
        logical(c_bool) :: polarised
        integer(c_int) :: set_type
        integer(c_int) :: interpolator_type
        type (c_ptr) :: error_type = c_null_ptr
        integer(c_int) :: hadron_pid
        type(neopdf_physics_parameters) :: phys_params
    end type

    type, bind(c) :: neopdf_metadata_v2
        type (c_ptr) :: set_desc = c_null_ptr
        integer(c_int32_t) :: set_index
        integer(c_int32_t) :: num_members
        real(c_double) :: x_min
        real(c_double) :: x_max
        real(c_double) :: q_min
        real(c_double) :: q_max
        type (c_ptr) :: flavors = c_null_ptr
        integer(c_size_t) :: num_flavors
        type (c_ptr) :: format = c_null_ptr
        type (c_ptr) :: alphas_q_values = c_null_ptr
        integer(c_size_t) :: num_alphas_q
        type (c_ptr) :: alphas_vals = c_null_ptr
        integer(c_size_t) :: num_alphas_vals
        logical(c_bool) :: polarised
        integer(c_int) :: set_type
        integer(c_int) :: interpolator_type
        type (c_ptr) :: error_type = c_null_ptr
        integer(c_int) :: hadron_pid
        type(neopdf_physics_parameters) :: phys_params
        real(c_double) :: xi_min
        real(c_double) :: xi_max
        real(c_double) :: delta_min
        real(c_double) :: delta_max
    end type

    type, bind(c) :: neopdf_pdf_members
        type (c_ptr) :: pdfs = c_null_ptr
        integer (c_size_t) :: size
    end type

    enum, bind(c) ! :: neopdf_subgrid_params
        enumerator :: neopdf_subgrid_params_nucleons
        enumerator :: neopdf_subgrid_params_alphas
        enumerator :: neopdf_subgrid_params_xi
        enumerator :: neopdf_subgrid_params_delta
        enumerator :: neopdf_subgrid_params_kt
        enumerator :: neopdf_subgrid_params_momentum
        enumerator :: neopdf_subgrid_params_scale

        enumerator :: neopdf_subgrid_params
    end enum

    enum, bind(c) ! :: neopdf_force_positive
        enumerator :: neopdf_force_positive_clip_negative
        enumerator :: neopdf_force_positive_clip_small
        enumerator :: neopdf_force_positive_no_clipping

        enumerator :: neopdf_force_positive
    end enum

    interface
        function strlen(s) bind(c, name="strlen")
            use iso_c_binding
            implicit none
            type (c_ptr), value :: s
            integer (c_size_t)  :: strlen
        end function strlen

        type (c_ptr) function c_neopdf_pdf_load(pdf_name, member) bind(c, name="neopdf_pdf_load")
            use iso_c_binding
            character (c_char) :: pdf_name(*)
            integer (c_size_t), value :: member
        end function

        type (c_ptr) function c_neopdf_pdf_load_lazy(pdf_name) bind(c, name="neopdf_pdf_load_lazy")
            use iso_c_binding
            character (c_char) :: pdf_name(*)
        end function

        type (c_ptr) function c_neopdf_lazy_iterator_next(iter) bind(c, name="neopdf_lazy_iterator_next")
            use iso_c_binding
            type (c_ptr), value :: iter
        end function

        subroutine c_neopdf_lazy_iterator_free(iter) bind(c, name="neopdf_lazy_iterator_free")
            use iso_c_binding
            type (c_ptr), value :: iter
        end subroutine

        subroutine c_neopdf_pdf_free(pdf) bind(c, name="neopdf_pdf_free")
            use iso_c_binding
            type (c_ptr), value :: pdf
        end subroutine

        function c_neopdf_pdf_xfxq2(pdf, id, x, q2) bind(c, name="neopdf_pdf_xfxq2")
            use iso_c_binding
            type (c_ptr), value :: pdf
            integer (c_int), value :: id
            real (c_double), value :: x, q2
            real (c_double) :: c_neopdf_pdf_xfxq2
        end function

        function c_neopdf_pdf_alphas_q2(pdf, q2) bind(c, name="neopdf_pdf_alphas_q2")
            use iso_c_binding
            type (c_ptr), value :: pdf
            real (c_double), value :: q2
            real (c_double) :: c_neopdf_pdf_alphas_q2
        end function

        function c_neopdf_pdf_x_min(pdf) bind(c, name="neopdf_pdf_x_min")
            use iso_c_binding
            type (c_ptr), value :: pdf
            real (c_double) :: c_neopdf_pdf_x_min
        end function

        function c_neopdf_pdf_x_max(pdf) bind(c, name="neopdf_pdf_x_max")
            use iso_c_binding
            type (c_ptr), value :: pdf
            real (c_double) :: c_neopdf_pdf_x_max
        end function

        function c_neopdf_pdf_q2_min(pdf) bind(c, name="neopdf_pdf_q2_min")
            use iso_c_binding
            type (c_ptr), value :: pdf
            real (c_double) :: c_neopdf_pdf_q2_min
        end function

        function c_neopdf_pdf_q2_max(pdf) bind(c, name="neopdf_pdf_q2_max")
            use iso_c_binding
            type (c_ptr), value :: pdf
            real (c_double) :: c_neopdf_pdf_q2_max
        end function

        function c_neopdf_pdf_num_pids(pdf) bind(c, name="neopdf_pdf_num_pids")
            use iso_c_binding
            type (c_ptr), value :: pdf
            integer (c_size_t) :: c_neopdf_pdf_num_pids
        end function

        subroutine c_neopdf_pdf_pids(pdf, pids, num_pids) bind(c, name="neopdf_pdf_pids")
            use iso_c_binding
            type (c_ptr), value :: pdf
            integer (c_int) :: pids(*)
            integer (c_size_t), value :: num_pids
        end subroutine

        function c_neopdf_pdf_num_subgrids(pdf) bind(c, name="neopdf_pdf_num_subgrids")
            use iso_c_binding
            type (c_ptr), value :: pdf
            integer (c_size_t) :: c_neopdf_pdf_num_subgrids
        end function

        subroutine c_neopdf_pdf_param_range(pdf, param, param_range) bind(c, name="neopdf_pdf_param_range")
            use iso_c_binding
            type (c_ptr), value :: pdf
            integer (c_int), value :: param
            real (c_double) :: param_range(*)
        end subroutine

        subroutine c_neopdf_pdf_subgrids_shape_for_param(pdf, subgrid_shape, num_subgrid, subgrid_param) bind(c, name="neopdf_pdf_subgrids_shape_for_param")
            use iso_c_binding
            type (c_ptr), value :: pdf
            integer (c_size_t) :: subgrid_shape(*)
            integer (c_size_t), value :: num_subgrid
            integer (c_int), value :: subgrid_param
        end subroutine

        subroutine c_neopdf_pdf_subgrids_for_param(pdf, subgrid, subgrid_param, num_subgrid, subgrid_shape, subgrid_index) bind(c, name="neopdf_pdf_subgrids_for_param")
            use iso_c_binding
            type (c_ptr), value :: pdf
            real (c_double) :: subgrid(*)
            integer (c_int), value :: subgrid_param
            integer (c_size_t), value :: num_subgrid
            integer (c_size_t) :: subgrid_shape(*)
            integer (c_size_t), value :: subgrid_index
        end subroutine

        type (neopdf_pdf_members) function c_neopdf_pdf_load_all(pdf_name) bind(c, name="neopdf_pdf_load_all")
            use iso_c_binding
            import :: neopdf_pdf_members
            character (c_char) :: pdf_name(*)
        end function

        subroutine c_neopdf_pdf_array_free(array) bind(c, name="neopdf_pdf_array_free")
            use iso_c_binding
            import :: neopdf_pdf_members
            type (neopdf_pdf_members), value :: array
        end subroutine

        function c_neopdf_pdf_xfxq2_nd(pdf, id, params, num_params) bind(c, name="neopdf_pdf_xfxq2_nd")
            use iso_c_binding
            type (c_ptr), value :: pdf
            integer (c_int), value :: id
            real (c_double) :: params(*)
            integer (c_size_t), value :: num_params
            real (c_double) :: c_neopdf_pdf_xfxq2_nd
        end function

        subroutine c_neopdf_pdf_xfxq2_cheby_batch(pdf, flavor_id, points, num_points, num_dims, results) bind(c, name="neopdf_pdf_xfxq2_cheby_batch")
            use iso_c_binding
            type(c_ptr), value :: pdf
            integer(c_int), value :: flavor_id
            real(c_double), intent(in) :: points(*)
            integer(c_size_t), value :: num_points
            integer(c_size_t), value :: num_dims
            real(c_double), intent(out) :: results(*)
        end subroutine

        subroutine c_neopdf_pdf_set_force_positive(pdf, option) bind(c, name="neopdf_pdf_set_force_positive")
            use iso_c_binding
            type (c_ptr), value :: pdf
            integer (c_int), value :: option
        end subroutine

        subroutine c_neopdf_pdf_set_force_positive_members(pdfs, option) bind(c, name="neopdf_pdf_set_force_positive_members")
            use iso_c_binding
            import :: neopdf_pdf_members
            type (neopdf_pdf_members) :: pdfs
            integer (c_int), value :: option
        end subroutine

        function c_neopdf_pdf_is_force_positive(pdf) bind(c, name="neopdf_pdf_is_force_positive")
            use iso_c_binding
            type (c_ptr), value :: pdf
            integer (c_int) :: c_neopdf_pdf_is_force_positive
        end function

        type (c_ptr) function c_neopdf_grid_new() bind(c, name="neopdf_grid_new")
            use iso_c_binding
        end function

        subroutine c_neopdf_grid_free(grid) bind(c, name="neopdf_grid_free")
            use iso_c_binding
            type (c_ptr), value :: grid
        end subroutine

        function c_neopdf_grid_add_subgrid(grid, nucleons, num_nucleons, alphas, num_alphas, kts, num_kts, xs, num_xs, q2s, num_q2s, grid_data, grid_data_len) bind(c, name="neopdf_grid_add_subgrid")
            use iso_c_binding
            type (c_ptr), value :: grid
            real (c_double) :: nucleons(*)
            integer (c_size_t), value :: num_nucleons
            real (c_double) :: alphas(*)
            integer (c_size_t), value :: num_alphas
            real (c_double) :: kts(*)
            integer (c_size_t), value :: num_kts
            real (c_double) :: xs(*)
            integer (c_size_t), value :: num_xs
            real (c_double) :: q2s(*)
            integer (c_size_t), value :: num_q2s
            real (c_double) :: grid_data(*)
            integer (c_size_t), value :: grid_data_len
            integer (c_int) :: c_neopdf_grid_add_subgrid
        end function

        function c_neopdf_grid_add_subgridv2(grid, nucleons, num_nucleons, alphas, num_alphas, xis, num_xis, deltas, num_deltas, kts, num_kts, xs, num_xs, q2s, num_q2s, grid_data, grid_data_len) bind(c, name="neopdf_grid_add_subgridv2")
            use iso_c_binding
            type (c_ptr), value :: grid
            real (c_double) :: nucleons(*)
            integer (c_size_t), value :: num_nucleons
            real (c_double) :: alphas(*)
            integer (c_size_t), value :: num_alphas
            real (c_double) :: xis(*)
            integer (c_size_t), value :: num_xis
            real (c_double) :: deltas(*)
            integer (c_size_t), value :: num_deltas
            real (c_double) :: kts(*)
            integer (c_size_t), value :: num_kts
            real (c_double) :: xs(*)
            integer (c_size_t), value :: num_xs
            real (c_double) :: q2s(*)
            integer (c_size_t), value :: num_q2s
            real (c_double) :: grid_data(*)
            integer (c_size_t), value :: grid_data_len
            integer (c_int) :: c_neopdf_grid_add_subgridv2
        end function

        function c_neopdf_grid_set_flavors(grid, flavors, num_flavors) bind(c, name="neopdf_grid_set_flavors")
            use iso_c_binding
            type (c_ptr), value :: grid
            integer (c_int) :: flavors(*)
            integer (c_size_t), value :: num_flavors
            integer (c_int) :: c_neopdf_grid_set_flavors
        end function

        type (c_ptr) function c_neopdf_gridarray_collection_new() bind(c, name="neopdf_gridarray_collection_new")
            use iso_c_binding
        end function

        subroutine c_neopdf_gridarray_collection_free(collection) bind(c, name="neopdf_gridarray_collection_free")
            use iso_c_binding
            type (c_ptr), value :: collection
        end subroutine

        function c_neopdf_gridarray_collection_add_grid(collection, grid) bind(c, name="neopdf_gridarray_collection_add_grid")
            use iso_c_binding
            type (c_ptr), value :: collection
            type (c_ptr), value :: grid
            integer (c_int) :: c_neopdf_gridarray_collection_add_grid
        end function

        function c_neopdf_grid_compress(collection, metadata, output_path) bind(c, name="neopdf_grid_compress")
            use iso_c_binding
            import :: neopdf_metadata
            type (c_ptr), value :: collection
            type (neopdf_metadata) :: metadata
            character (c_char) :: output_path(*)
            integer (c_int) :: c_neopdf_grid_compress
        end function

        function c_neopdf_grid_compress_v2(collection, metadata, output_path) bind(c, name="neopdf_grid_compress_v2")
            use iso_c_binding
            import :: neopdf_metadata_v2
            type (c_ptr), value :: collection
            type (neopdf_metadata_v2) :: metadata
            character (c_char) :: output_path(*)
            integer (c_int) :: c_neopdf_grid_compress_v2
        end function

        subroutine c_setlhaparm(line, len) bind(c, name="setlhaparm_")
            use iso_c_binding
            character(kind=c_char) :: line(*)
            integer(c_int), value :: len
        end subroutine

        subroutine c_initpdfsetbyname(name, len) bind(c, name="initpdfsetbyname_")
            use iso_c_binding
            character(kind=c_char) :: name(*)
            integer(c_int), value :: len
        end subroutine

        subroutine c_initpdf(member) bind(c, name="initpdf_")
            use iso_c_binding
            integer(c_int) :: member
        end subroutine

        subroutine c_evolvepdf(x, q, f) bind(c, name="evolvepdf_")
            use iso_c_binding
            real(c_double) :: x, q
            real(c_double) :: f(*)
        end subroutine
    end interface

contains
    function c_f_string(c_str) result(f_str)
        use :: iso_c_binding

        type(c_ptr), intent(in) :: c_str
        character(kind=c_char), dimension(:), pointer :: arr_f_ptr => null()
        character(len=:, kind=c_char), allocatable :: f_str
        integer(kind=c_size_t) :: i, length

        length = strlen(c_str)
        call c_f_pointer(c_str, arr_f_ptr, [length])

        if (.not.associated(arr_f_ptr)) then
            f_str = "NULL"
            return
        end if

        allocate(character(len=length)::f_str)

        do i = 1, length
            f_str(i:i) = arr_f_ptr(i)
        end do
    end function

    type (neopdf_pdf) function neopdf_pdf_load(pdf_name, member)
        implicit none

        character (*), intent(in) :: pdf_name
        integer, intent(in) :: member

        neopdf_pdf_load = neopdf_pdf(c_neopdf_pdf_load(pdf_name // c_null_char, int(member, c_size_t)))
    end function

    type (neopdf_lazy_iterator) function neopdf_pdf_load_lazy(pdf_name)
        implicit none

        character (*), intent(in) :: pdf_name

        neopdf_pdf_load_lazy = neopdf_lazy_iterator(c_neopdf_pdf_load_lazy(pdf_name // c_null_char))
    end function

    type (neopdf_pdf) function neopdf_lazy_iterator_next(iter)
        implicit none

        type (neopdf_lazy_iterator), intent(in) :: iter

        neopdf_lazy_iterator_next = neopdf_pdf(c_neopdf_lazy_iterator_next(iter%ptr))
    end function

    subroutine neopdf_lazy_iterator_free(iter)
        implicit none

        type (neopdf_lazy_iterator), intent(in) :: iter

        call c_neopdf_lazy_iterator_free(iter%ptr)
    end subroutine

    subroutine neopdf_pdf_free(pdf)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf

        call c_neopdf_pdf_free(pdf%ptr)
    end subroutine

    function neopdf_pdf_xfxq2(pdf, id, x, q2) result(res)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        integer, intent(in) :: id
        real (dp), intent(in) :: x, q2
        real (dp) :: res

        res = c_neopdf_pdf_xfxq2(pdf%ptr, id, x, q2)
    end function

    function neopdf_pdf_alphas_q2(pdf, q2) result(res)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        real (dp), intent(in) :: q2
        real (dp) :: res

        res = c_neopdf_pdf_alphas_q2(pdf%ptr, q2)
    end function

    function neopdf_pdf_x_min(pdf) result(res)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        real (dp) :: res

        res = c_neopdf_pdf_x_min(pdf%ptr)
    end function

    function neopdf_pdf_x_max(pdf) result(res)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        real (dp) :: res

        res = c_neopdf_pdf_x_max(pdf%ptr)
    end function

    function neopdf_pdf_q2_min(pdf) result(res)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        real (dp) :: res

        res = c_neopdf_pdf_q2_min(pdf%ptr)
    end function

    function neopdf_pdf_q2_max(pdf) result(res)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        real (dp) :: res

        res = c_neopdf_pdf_q2_max(pdf%ptr)
    end function

    function neopdf_pdf_num_pids(pdf) result(res)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        integer :: res

        res = int(c_neopdf_pdf_num_pids(pdf%ptr))
    end function

    subroutine neopdf_pdf_pids(pdf, pids)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        integer, intent(out) :: pids(:)

        call c_neopdf_pdf_pids(pdf%ptr, pids, int(size(pids), c_size_t))
    end subroutine

    function neopdf_pdf_num_subgrids(pdf) result(res)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        integer :: res

        res = int(c_neopdf_pdf_num_subgrids(pdf%ptr))
    end function

    subroutine neopdf_pdf_param_range(pdf, param, param_range)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        integer, intent(in) :: param
        real (dp), intent(out) :: param_range(2)

        call c_neopdf_pdf_param_range(pdf%ptr, param, param_range)
    end subroutine

    subroutine neopdf_pdf_subgrids_shape_for_param(pdf, subgrid_shape, subgrid_param)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        integer(c_size_t), intent(out) :: subgrid_shape(:)
        integer, intent(in) :: subgrid_param

        call c_neopdf_pdf_subgrids_shape_for_param(pdf%ptr, subgrid_shape, int(size(subgrid_shape), c_size_t), subgrid_param)
    end subroutine

    subroutine neopdf_pdf_subgrids_for_param(pdf, subgrid, subgrid_param, subgrid_shape, subgrid_index)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        real (dp), intent(out) :: subgrid(*)
        integer, intent(in) :: subgrid_param
        integer(c_size_t), intent(in) :: subgrid_shape(:)
        integer, intent(in) :: subgrid_index

        call c_neopdf_pdf_subgrids_for_param(pdf%ptr, subgrid, subgrid_param, int(size(subgrid_shape), c_size_t), subgrid_shape, int(subgrid_index, c_size_t))
    end subroutine

    type (neopdf_pdf_members) function neopdf_pdf_load_all(pdf_name)
        implicit none

        character (*), intent(in) :: pdf_name

        neopdf_pdf_load_all = c_neopdf_pdf_load_all(pdf_name // c_null_char)
    end function

    subroutine neopdf_pdf_array_free(array)
        implicit none

        type (neopdf_pdf_members), intent(in) :: array

        call c_neopdf_pdf_array_free(array)
    end subroutine

    function neopdf_pdf_xfxq2_nd(pdf, id, params) result(res)
        implicit none

        type (neopdf_pdf), intent(in) :: pdf
        integer, intent(in) :: id
        real (dp), intent(in) :: params(:)
        real (dp) :: res

        res = c_neopdf_pdf_xfxq2_nd(pdf%ptr, id, params, int(size(params), c_size_t))
    end function

    subroutine neopdf_pdf_xfxq2_cheby_batch(pdf, flavor_id, points, results)
        implicit none
        type(neopdf_pdf), intent(in) :: pdf
        integer, intent(in) :: flavor_id
        real(dp), intent(in) :: points(:,:)
        real(dp), intent(out) :: results(:)
        integer(c_size_t) :: num_dims, num_points

        num_dims = size(points, 1)
        num_points = size(points, 2)

        call c_neopdf_pdf_xfxq2_cheby_batch(pdf%ptr, flavor_id, points, num_points, num_dims, results)
    end subroutine

    subroutine neopdf_pdf_set_force_positive(pdf, option)
        implicit none
        type (neopdf_pdf), intent(in) :: pdf
        integer, intent(in) :: option
        call c_neopdf_pdf_set_force_positive(pdf%ptr, option)
    end subroutine

    subroutine neopdf_pdf_set_force_positive_members(pdfs, option)
        implicit none
        type (neopdf_pdf_members), intent(inout) :: pdfs
        integer, intent(in) :: option
        call c_neopdf_pdf_set_force_positive_members(pdfs, option)
    end subroutine

    function neopdf_pdf_is_force_positive(pdf) result(res)
        implicit none
        type (neopdf_pdf), intent(in) :: pdf
        integer :: res
        res = c_neopdf_pdf_is_force_positive(pdf%ptr)
    end function

    type (neopdf_grid) function neopdf_grid_new()
        implicit none
        neopdf_grid_new = neopdf_grid(c_neopdf_grid_new())
    end function

    subroutine neopdf_grid_free(grid)
        implicit none
        type (neopdf_grid), intent(in) :: grid
        call c_neopdf_grid_free(grid%ptr)
    end subroutine

    function neopdf_grid_add_subgrid(grid, nucleons, alphas, kts, xs, q2s, grid_data) result(res)
        implicit none
        type (neopdf_grid), intent(in) :: grid
        real (dp), intent(in) :: nucleons(:), alphas(:), kts(:), xs(:), q2s(:), grid_data(:)
        integer :: res
        res = c_neopdf_grid_add_subgrid(grid%ptr, nucleons, int(size(nucleons), c_size_t), alphas, int(size(alphas), c_size_t), kts, int(size(kts), c_size_t), xs, int(size(xs), c_size_t), q2s, int(size(q2s), c_size_t), grid_data, int(size(grid_data), c_size_t))
    end function

    function neopdf_grid_add_subgridv2(grid, nucleons, alphas, xis, deltas, kts, xs, q2s, grid_data) result(res)
        implicit none
        type (neopdf_grid), intent(in) :: grid
        real (dp), intent(in) :: nucleons(:), alphas(:), xis(:), deltas(:), kts(:), xs(:), q2s(:), grid_data(:)
        integer :: res
        res = c_neopdf_grid_add_subgridv2(grid%ptr, &
            nucleons, int(size(nucleons), c_size_t), &
            alphas, int(size(alphas), c_size_t), &
            xis, int(size(xis), c_size_t), &
            deltas, int(size(deltas), c_size_t), &
            kts, int(size(kts), c_size_t), &
            xs, int(size(xs), c_size_t), &
            q2s, int(size(q2s), c_size_t), &
            grid_data, int(size(grid_data), c_size_t))
    end function

    function neopdf_grid_set_flavors(grid, flavors) result(res)
        implicit none
        type (neopdf_grid), intent(in) :: grid
        integer, intent(in) :: flavors(:)
        integer :: res
        res = c_neopdf_grid_set_flavors(grid%ptr, flavors, int(size(flavors), c_size_t))
    end function

    type (neopdf_grid_array_collection) function neopdf_gridarray_collection_new()
        implicit none
        neopdf_gridarray_collection_new = neopdf_grid_array_collection(c_neopdf_gridarray_collection_new())
    end function

    subroutine neopdf_gridarray_collection_free(collection)
        implicit none
        type (neopdf_grid_array_collection), intent(in) :: collection
        call c_neopdf_gridarray_collection_free(collection%ptr)
    end subroutine

    function neopdf_gridarray_collection_add_grid(collection, grid) result(res)
        implicit none
        type (neopdf_grid_array_collection), intent(in) :: collection
        type (neopdf_grid), intent(in) :: grid
        integer :: res
        res = c_neopdf_gridarray_collection_add_grid(collection%ptr, grid%ptr)
    end function

    function neopdf_grid_compress(collection, metadata, output_path) result(res)
        implicit none
        type (neopdf_grid_array_collection), intent(in) :: collection
        type (neopdf_metadata), intent(in) :: metadata
        character (*), intent(in) :: output_path
        integer :: res
        res = c_neopdf_grid_compress(collection%ptr, metadata, output_path // c_null_char)
    end function

    function neopdf_grid_compress_v2(collection, metadata, output_path) result(res)
        implicit none
        type (neopdf_grid_array_collection), intent(in) :: collection
        type (neopdf_metadata_v2), intent(in) :: metadata
        character (*), intent(in) :: output_path
        integer :: res
        res = c_neopdf_grid_compress_v2(collection%ptr, metadata, output_path // c_null_char)
    end function

    subroutine setlhaparm(line)
        character(len=*), intent(in) :: line
        call c_setlhaparm(line, int(len(trim(line)), c_int))
    end subroutine

    subroutine initpdfsetbyname(name)
        character(len=*), intent(in) :: name
        call c_initpdfsetbyname(name, int(len(trim(name)), c_int))
    end subroutine

    subroutine initpdf(member)
        integer, intent(in) :: member
        call c_initpdf(member)
    end subroutine

    subroutine evolvepdf(x, q, f)
        real(dp), intent(in) :: x, q
        real(dp), intent(out) :: f(-6:6)
        call c_evolvepdf(x, q, f)
    end subroutine

end module
