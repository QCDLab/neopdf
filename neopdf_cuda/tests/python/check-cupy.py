"""A GPU-resident workflow with CuPy: phase-space points are generated, the PDFs
evaluated and the weights reduced on the device, all on one stream. Only the final
per-member results are copied back. Computes the gluon-gluon luminosity:

    dL/dtau = 1/tau * int dy xg(x1, Q2) xg(x2, Q2),  x1,2 = sqrt(tau) e^(+-y),  Q2 = tau s,

for every member of the set, hence with its replica uncertainty.
"""

import cupy as cp
import numpy as np
from neopdf_cuda import DeviceGrid

SQRT_S = 13600.0
N = 4096

grid = DeviceGrid("NNPDF40_nnlo_as_01180")
gluon = grid.pids().index(21)
pdfs = cp.empty((2 * N, grid.num_members(), len(grid.pids()) + 1))

print("     M [GeV]   dL/dtau (central)   replica std dev")
print("---------------------------------------------------")
with cp.cuda.Stream(non_blocking=True) as stream:
    for mass in [125.0, 500.0, 1000.0, 3000.0]:
        tau = (mass / SQRT_S) ** 2
        # Midpoint rule in rapidity: events [0, N) carry x1, events [N, 2N) carry x2.
        y_max = -0.5 * np.log(tau)
        y = -y_max + 2.0 * y_max * (cp.arange(N) + 0.5) / N
        xs = cp.sqrt(tau) * cp.exp(cp.concatenate([y, -y]))
        q2s = cp.full(2 * N, tau * SQRT_S**2)

        grid.xfxQ2_batch_device(xs, q2s, pdfs, stream.ptr)

        xg = pdfs[:, :, gluon]
        lumi = (-np.log(tau) / N / tau) * (xg[:N] * xg[N:]).sum(axis=0)
        result = lumi.get(stream=stream)
        print(f"{mass:12.1f} {result[0]:19.6e} {result[1:].std(ddof=1):17.6e}")
