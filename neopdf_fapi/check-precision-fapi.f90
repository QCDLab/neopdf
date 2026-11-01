program check_precision_fapi
    use neopdf
    use iso_c_binding
    use iso_fortran_env

    implicit none

    type (neopdf_pdf)           :: pdf, tmd
    type (neopdf_precision_pdf) :: view
    integer                     :: ix, iq, ip, status
    integer                     :: pids(4) = [-2, 1, 2, 21]
    double precision            :: xs(4) = [1d-5, 1d-2, 0.3d0, 0.8d0]
    double precision            :: q2s(3) = [3d0, 1d2, 1d5]
    double precision            :: reference
    real (real128)              :: xf, alphas

    pdf = neopdf_pdf_load("NNPDF40_nnlo_as_01180", 0)

    status = neopdf_pdf_with_quad(pdf, view)
    call check(status == neopdf_result_success, "preparing the quadruple-precision view")

    write(*,'(A)') repeat('-', 70)
    write(*,'(A4, 2X, A10, 2X, A10, 2X, A40)') 'pid', 'x', 'Q2', 'real128'
    write(*,'(A)') repeat('-', 70)

    do ip = 1, size(pids)
        do ix = 1, size(xs)
            do iq = 1, size(q2s)
                status = neopdf_precision_pdf_xfxq2_quad(view, pids(ip), real(xs(ix), real128), real(q2s(iq), real128), xf)
                call check(status == neopdf_result_success, "evaluating inside the grid")
                reference = neopdf_pdf_xfxq2(pdf, pids(ip), xs(ix), q2s(iq))
                call check(abs(xf - reference) <= 1d-13 * abs(reference), "agreement with neopdf_pdf_xfxq2")
                write(*,'(I4, 2X, ES10.3, 2X, ES10.3, 2X, ES40.30)') pids(ip), xs(ix), q2s(iq), xf
            end do
        end do
    end do

    write(*,'(A)') repeat('-', 70)

    ! A flavour that the set does not contain is zero.
    status = neopdf_precision_pdf_xfxq2_quad(view, 6, 0.1_real128, 100.0_real128, xf)
    call check(status == neopdf_result_success .and. abs(xf) < tiny(xf), "missing flavour")

    status = neopdf_precision_pdf_alphas_q2_quad(view, 8315.0_real128, alphas)
    call check(status == neopdf_result_success, "evaluating alpha_s")
    call check(abs(alphas - neopdf_pdf_alphas_q2(pdf, 8315d0)) <= 1d-15, "agreement with neopdf_pdf_alphas_q2")
    write(*,'(A, ES40.30)') 'alpha_s(Q2 = 8315) = ', alphas

    ! There is no extrapolation.
    status = neopdf_precision_pdf_xfxq2_quad(view, 21, 1e-12_real128, 100.0_real128, xf)
    write(*,'(A, I3)') 'x outside the grid:  ', status
    status = neopdf_precision_pdf_alphas_q2_quad(view, 1.0_real128, alphas)
    write(*,'(A, I3)') 'Q2 outside alpha_s:  ', status

    ! The view does not refer to the PDF once it is prepared.
    call neopdf_pdf_free(pdf)
    status = neopdf_precision_pdf_xfxq2_quad(view, 21, 0.1_real128, 100.0_real128, xf)
    call check(status == neopdf_result_success .and. xf > 0.0_real128, "evaluating after freeing the PDF")
    call neopdf_precision_pdf_free(view)

    ! Only collinear `LogBicubic` sets are supported.
    tmd = neopdf_pdf_load("MAP22_grids_FF_Km_N3LL.neopdf.lz4", 0)
    status = neopdf_pdf_with_quad(tmd, view)
    write(*,'(A, I3)') 'unsupported set:     ', status
    call check(.not. c_associated(view%ptr), "no view for an unsupported set")
    call neopdf_pdf_free(tmd)

contains

    subroutine check(condition, what)
        logical, intent(in) :: condition
        character (*), intent(in) :: what

        if (.not. condition) then
            write(*, *) "failed: ", what
            stop 1
        end if
    end subroutine

end program check_precision_fapi
