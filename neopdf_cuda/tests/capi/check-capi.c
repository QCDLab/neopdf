#include <neopdf_cuda.h>
#include <stdio.h>
#include <stdlib.h>

/* Evaluates every member of a set on the GPU from host arrays. */
int main(void) {
    NeoPDFDeviceGrid *grid = neopdf_device_grid_new("NNPDF40_nnlo_as_01180", 0);
    if (grid == NULL) { return EXIT_FAILURE; }

    size_t   num_members = neopdf_device_grid_num_members(grid);
    size_t   num_pids    = neopdf_device_grid_num_pids(grid);
    int32_t *pids        = malloc(num_pids * sizeof(int32_t));
    if (neopdf_device_grid_pids(grid, pids, num_pids) !=
        NEOPDF_CUDA_RESULT_SUCCESS) {
        return EXIT_FAILURE;
    }
    size_t gluon = 0, up = 0;
    for (size_t i = 0; i < num_pids; ++i) {
        if (pids[i] == 21) gluon = i;
        if (pids[i] == 2) up = i;
    }

    double  xs[]       = {1e-5, 1e-3, 1e-1, 5e-1};
    double  q2s[]      = {1e1, 1e2, 1e4, 1e6};
    size_t  num_events = sizeof(xs) / sizeof(xs[0]);
    size_t  out_len    = neopdf_device_grid_output_len(grid, num_events);
    double *out        = malloc(out_len * sizeof(double));
    if (neopdf_device_grid_xfxq2_batch_host(grid,
            xs,
            q2s,
            num_events,
            out,
            out_len) != NEOPDF_CUDA_RESULT_SUCCESS) {
        return EXIT_FAILURE;
    }

    printf("members: %zu, flavors: %zu\n", num_members, num_pids);
    printf("              x              Q2              xg              xu    "
           "     alpha_s\n");
    printf("-------------------------------------------------------------------"
           "-------------\n");
    for (size_t e = 0; e < num_events; ++e) {
        /* Member 0 of event `e` starts at `e * num_members * (num_pids + 1)`.
         */
        const double *row = out + e * num_members * (num_pids + 1);
        printf("%15.8e %15.8e %15.8e %15.8e %15.8e\n",
            xs[e],
            q2s[e],
            row[gluon],
            row[up],
            row[num_pids]);
    }

    free(out);
    free(pids);
    neopdf_device_grid_free(grid);
    return EXIT_SUCCESS;
}
