# Fortran API Example

The following examples illustrate how to use the Fortran interface to the
`NeoPDF` library and to evaluate distributions.

## Prerequisites

Build and install the C API as described in the [installation guide](../installation.md).
Then, copy the `neopdf_fapi/neopdf.f90` in the working directory and include it in the
compilation. The following is an example of Makefile:

```make title="Makefile"
FC = gfortran
FFLAGS = -Wall -Wextra -O0 -g -ffree-line-length-none
NEOPDF_LIBS != pkg-config neopdf_capi --libs
LHAPDF_LIBS != pkg-config lhapdf --libs

PROGRAMS = check-fapi

all: $(PROGRAMS)

test-examples: $(PROGRAMS)
	set -e && for i in $(PROGRAMS); do ./$${i} > output; diff -u $${i}.output output; done; rm output

neopdf.o: neopdf.f90
	$(FC) $(FFLAGS) -c $< -o $@

check-fapi: check-fapi.f90 neopdf.o
	$(FC) $(FFLAGS) $< neopdf.o $(LHAPDF_LIBS) $(NEOPDF_LIBS) -o $@

check-lazy-fapi: check-lazy-fapi.f90 neopdf.o
	$(FC) $(FFLAGS) $< neopdf.o $(NEOPDF_LIBS) -o $@

check-writer-fapi: check-writer-fapi.f90 neopdf.o
	$(FC) $(FFLAGS) $< neopdf.o $(NEOPDF_LIBS) -o $@

.PHONY: clean

clean:
	rm -f *.o *.mod $(PROGRAMS)
```

## Example 1: Loading and Evaluating PDFs

This example demonstrates the use of the `NeoPDF` Fortran API to load a single PDF
member, evaluate parton distributions for a range of $x$ values, and compare the
results to LHAPDF.

```fortran linenums="1"
program check_fapi
    use neopdf
    use iso_c_binding

    implicit none

    type (neopdf_pdf)  :: pdf
    character(len=256) :: pdf_name
    integer            :: id, ix, member
    double precision   :: x, q2, g_neo, g_lha
    double precision xfs(-6:6)

    pdf_name = "NNPDF40_nnlo_as_01180"
    member = 0
    q2 = 100.0
    id = 21

    call setlhaparm("SILENT")
    call initpdfsetbyname(pdf_name)
    pdf = neopdf_pdf_load(trim(pdf_name), member)

    write(*,'(A)') repeat('-', 38)
    write(*,'(A10, 2X, A12, 2X, A12)') 'x', 'LHAPDF', 'NeOPDF'
    write(*,'(A)') repeat('-', 38)

    do ix=1,10
        x = (ix - 0.5d0) / 10.0d0

        call evolvepdf(x, sqrt(q2), xfs)
        g_lha = xfs(0)
        g_neo = neopdf_pdf_xfxq2(pdf, id, x, q2)

        ! check the agreement (absolute value)
        if (abs(g_lha - g_neo) > 1e-16) then
            write(*, *) "LHAPDF and NeoPDF differs"
            stop 1
        end if

        write(*,'(ES10.3, 2X, ES12.5, 2X, ES12.5)') x, g_lha, g_neo
    enddo

    write(*,'(A)') repeat('-', 38)

    call neopdf_pdf_free(pdf)

end program check_fapi
```

The above code would print the following results:

```text
--------------------------------------
         x        LHAPDF        NeOPDF
--------------------------------------
 5.000E-02   2.35826E+00   2.35826E+00
 1.500E-01   6.21341E-01   6.21341E-01
 2.500E-01   2.34180E-01   2.34180E-01
 3.500E-01   1.00192E-01   1.00192E-01
 4.500E-01   3.86353E-02   3.86353E-02
 5.500E-01   1.14658E-02   1.14658E-02
 6.500E-01   2.41062E-03   2.41062E-03
 7.500E-01   3.15737E-04   3.15737E-04
 8.500E-01   1.94082E-05   1.94082E-05
 9.500E-01   1.35325E-07   1.35325E-07
--------------------------------------
```

## Example 2: Loading PDF Members in Lazy Mode

The following example illustrates how to load all the PDF members **lazily**. This presents
some advantages in terms of speed and memory efficiency. However, the cost is delegated
to when the interpolation is computed an usually loading the entire PDF members in **eager**
mode when peforming evaluations on the members is generally much faster.

```fortran linenums="1"
program check_lazy_fapi
    use neopdf
    use iso_c_binding

    implicit none

    type (neopdf_pdf)  :: pdf
    type (neopdf_lazy_iterator) :: iter
    character(len=256) :: pdf_name
    integer            :: id, member
    double precision   :: x, q2, g_neo

    pdf_name = "NNPDF40_nnlo_as_01180.neopdf.lz4"
    q2 = 100.0
    id = 21
    x = 0.1

    iter = neopdf_pdf_load_lazy(trim(pdf_name))

    write(*,'(A)') repeat('-', 26)
    write(*,'(A10, 2X, A12)') 'Member', 'NeOPDF'
    write(*,'(A)') repeat('-', 26)

    do member=0,100
        pdf = neopdf_lazy_iterator_next(iter)
        if (.not. c_associated(pdf%ptr)) exit

        g_neo = neopdf_pdf_xfxq2(pdf, id, x, q2)

        write(*,'(I10, 2X, ES12.5)') member, g_neo

        call neopdf_pdf_free(pdf)
    enddo

    write(*,'(A)') repeat('-', 26)

    call neopdf_lazy_iterator_free(iter)

end program check_lazy_fapi
```

## Example 3: Filling and Writing a NeoPDF Grid

The following example illustrates how to fill, write, and compress `NeoPDF` grids using the
Fortran API.

The filling of the PDF grid in the following example assumes no dependence in the nucleon
numbers $A$ and strong coupling $\alpha_s$ (ie. standard LHAPDF-like PDF). Refer to the
[C++ tutorials](./c-oop.md) in the case the grid should explicitly depend on more
parameters.

!!! tip "NOTE"

    The following example fills the `NeoPDF` grid by re-computing the values of the subgrids
    from an LHAPDF set. This makes it possible to explicitly check that the filling of the grid
    is correct. However, this makes the codes very verbose. To easily spot the parts that
    actually fills the grid, some lines are highlighted.

```fortran linenums="1" hl_lines="50 57 63 73-80 82-93 95 101-102 129-147"
program check_writer_fapi
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
    real(dp), allocatable :: xs(:), q2s(:), alphas(:), nucleons(:), kts(:), grid_data(:)
    real(dp) :: x_range(2), q2_range(2)
    real(dp), target :: alphas_q_values(1), alphas_vals(1)
    type(neopdf_metadata) :: meta
    type(neopdf_physics_parameters) :: phys_params

    pdf_name = "NNPDF40_nnlo_as_01180"
    output_path = "check-writer-fapi.neopdf.lz4"

    pdfs = neopdf_pdf_load_all(trim(pdf_name))
    pdf%ptr = c_get_pointer_to_member(pdfs, 0)

    num_pids = neopdf_pdf_num_pids(pdf)
    allocate(pids(num_pids))
    call neopdf_pdf_pids(pdf, pids)

    num_subgrids = neopdf_pdf_num_subgrids(pdf)

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

            res = neopdf_grid_add_subgrid(grid, nucleons, alphas, kts, xs, q2s, grid_data)
            if (res /= 0) then
                write(*,*) "Failed to add subgrid"
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

    meta = neopdf_metadata( &
        set_desc=c_char_ptr("NNPDF40_nnlo_as_01180 collection" // c_null_char), &
        set_index=0, num_members=int(pdfs%size, c_int32_t), &
        x_min=x_range(1), x_max=x_range(2), &
        q_min=sqrt(q2_range(1)), q_max=sqrt(q2_range(2)), &
        flavors=c_loc(pids), num_flavors=size(pids), &
        format=c_char_ptr("neopdf" // c_null_char), &
        alphas_q_values=c_loc(alphas_q_values), num_alphas_q=1, &
        alphas_vals=c_loc(alphas_vals), num_alphas_vals=1, &
        polarised=.false._c_bool, set_type=0, interpolator_type=0, &
        error_type=c_char_ptr("replicas" // c_null_char), hadron_pid=2212, &
        phys_params=phys_params)

    res = neopdf_grid_compress(collection, meta, trim(output_path))
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
end program check_writer_fapi
```
