program check_writer_8d_fapi
    use neopdf
    use iso_c_binding
    use iso_fortran_env, only: real64

    implicit none

    integer, parameter :: dp = real64

    type (neopdf_pdf) :: pdf
    type (neopdf_pdf_members) :: pdfs
    type (neopdf_grid) :: grid
    type (neopdf_grid_array_collection) :: collection
    character(len=256) :: pdf_name, output_path
    integer, allocatable, target :: pids(:)
    integer :: num_pids, num_subgrids, m, subgrid_idx, res
    real(dp), allocatable :: xs(:), q2s(:), alphas(:), nucleons(:), kts(:), xis(:), deltas(:), grid_data(:)
    real(dp) :: x_range(2), q2_range(2)
    real(dp), target :: alphas_q_values(1), alphas_vals(1)
    type(neopdf_metadata_v2) :: meta
    type(neopdf_physics_parameters) :: phys_params

    pdf_name = "NNPDF40_nnlo_as_01180"
    output_path = "check-writer-8d-fapi.neopdf.lz4"

    pdfs = neopdf_pdf_load_all(trim(pdf_name))
    pdf%ptr = c_get_pointer_to_member(pdfs, 0)

    num_pids = neopdf_pdf_num_pids(pdf)
    allocate(pids(num_pids))
    call neopdf_pdf_pids(pdf, pids)

    num_subgrids = neopdf_pdf_num_subgrids(pdf)

    allocate(xis(1), deltas(1))
    xis = 0.0_dp
    deltas = 0.0_dp

    collection = neopdf_gridarray_collection_new()

    do m = 0, pdfs%size - 1
        pdf%ptr = c_get_pointer_to_member(pdfs, m)
        grid = neopdf_grid_new()

        do subgrid_idx = 0, num_subgrids - 1
            call extract_subgrid_params(pdf, NEOPDF_SUBGRID_PARAMS_MOMENTUM, subgrid_idx+1, num_subgrids, xs)
            call extract_subgrid_params(pdf, NEOPDF_SUBGRID_PARAMS_SCALE, subgrid_idx+1, num_subgrids, q2s)
            call extract_subgrid_params(pdf, NEOPDF_SUBGRID_PARAMS_ALPHAS, subgrid_idx+1, num_subgrids, alphas)
            call extract_subgrid_params(pdf, NEOPDF_SUBGRID_PARAMS_NUCLEONS, subgrid_idx+1, num_subgrids, nucleons)
            call extract_subgrid_params(pdf, NEOPDF_SUBGRID_PARAMS_KT, subgrid_idx+1, num_subgrids, kts)

            call compute_grid_data(pdf, xs, q2s, pids, grid_data)

            res = neopdf_grid_add_subgridv2(grid, nucleons, alphas, xis, deltas, kts, xs, q2s, grid_data)
            if (res /= 0) then
                write(*,*) "Failed to add subgrid v2"
                stop 1
            end if
        end do

        res = neopdf_grid_set_flavors(grid, pids)
        if (res /= 0) then
            write(*,*) "Failed to set flavors"
            stop 1
        end if

        res = neopdf_gridarray_collection_add_grid(collection, grid)
        if (res /= 0) then
            write(*,*) "Failed to add grid to collection"
            stop 1
        end if
    end do

    ! Reset to first member to get ranges
    pdf%ptr = c_get_pointer_to_member(pdfs, 0)
    call neopdf_pdf_param_range(pdf, NEOPDF_SUBGRID_PARAMS_MOMENTUM, x_range)
    call neopdf_pdf_param_range(pdf, NEOPDF_SUBGRID_PARAMS_SCALE, q2_range)

    phys_params = neopdf_physics_parameters( &
        flavor_scheme=c_char_ptr("variable" // c_null_char), &
        order_qcd=2, alphas_order_qcd=2, &
        m_w=80.352_dp, m_z=91.1876_dp, &
        m_up=0.0_dp, m_down=0.0_dp, m_strange=0.0_dp, &
        m_charm=1.51_dp, m_bottom=4.92_dp, m_top=172.5_dp, &
        alphas_type=c_char_ptr("ipol" // c_null_char), &
        number_flavors=4)

    alphas_q_values(1) = 2.0_dp
    alphas_vals(1) = 0.118_dp

    meta = neopdf_metadata_v2( &
        set_desc=c_char_ptr("NNPDF40_nnlo_as_01180 8D collection" // c_null_char), &
        set_index=0, num_members=int(pdfs%size, c_int32_t), &
        x_min=x_range(1), x_max=x_range(2), &
        q_min=sqrt(q2_range(1)), q_max=sqrt(q2_range(2)), &
        flavors=c_loc(pids), num_flavors=size(pids), &
        format=c_char_ptr("neopdf" // c_null_char), &
        alphas_q_values=c_loc(alphas_q_values), num_alphas_q=1, &
        alphas_vals=c_loc(alphas_vals), num_alphas_vals=1, &
        polarised=.false._c_bool, set_type=0, interpolator_type=0, &
        error_type=c_char_ptr("replicas" // c_null_char), hadron_pid=2212, &
        phys_params=phys_params, &
        xi_min=xis(1), xi_max=xis(1), &
        delta_min=deltas(1), delta_max=deltas(1))

    res = neopdf_grid_compress_v2(collection, meta, trim(output_path))
    if (res /= 0) then
        write(*,*) "Compression failed"
        stop 1
    end if

    call neopdf_gridarray_collection_free(collection)
    call neopdf_pdf_array_free(pdfs)

contains
    function c_get_pointer_to_member(pdfs, m) result(pdf_ptr)
        use iso_c_binding
        implicit none
        type(neopdf_pdf_members), intent(in) :: pdfs
        integer, intent(in) :: m
        type(c_ptr) :: pdf_ptr
        type(c_ptr), pointer :: pdf_array_ptr(:)

        call c_f_pointer(pdfs%pdfs, pdf_array_ptr, [pdfs%size])
        pdf_ptr = pdf_array_ptr(m + 1)
    end function

    subroutine extract_subgrid_params(pdf, param_type, subgrid_idx, num_subgrids, values)
        implicit none
        type(neopdf_pdf), intent(in) :: pdf
        integer, intent(in) :: param_type, subgrid_idx, num_subgrids
        real(dp), allocatable, intent(out) :: values(:)
        integer(c_size_t) :: shape(num_subgrids)

        call neopdf_pdf_subgrids_shape_for_param(pdf, shape, param_type)
        allocate(values(shape(subgrid_idx)))
        call neopdf_pdf_subgrids_for_param(pdf, values, param_type, shape, subgrid_idx-1)
    end subroutine

    subroutine compute_grid_data(pdf, xs, q2s, pids, grid_data)
        implicit none
        type(neopdf_pdf), intent(in) :: pdf
        real(dp), intent(in) :: xs(:), q2s(:)
        integer, intent(in) :: pids(:)
        real(dp), allocatable, intent(out) :: grid_data(:)
        integer :: xi, qi, f, idx

        allocate(grid_data(size(xs) * size(q2s) * size(pids)))
        idx = 1
        do xi = 1, size(xs)
            do qi = 1, size(q2s)
                do f = 1, size(pids)
                    grid_data(idx) = neopdf_pdf_xfxq2(pdf, pids(f), xs(xi), q2s(qi))
                    idx = idx + 1
                end do
            end do
        end do
    end subroutine

    function c_char_ptr(s) result(p)
        use iso_c_binding
        implicit none
        character(*), intent(in) :: s
        type(c_ptr) :: p
        character(len=len(s), kind=c_char), target :: s_c
        s_c = transfer(s, s_c)
        p = c_loc(s_c)
    end function
end program check_writer_8d_fapi
