"""Gluon replica uncertainty and alpha_s, evaluated for every member at once on the GPU."""

import numpy as np
from neopdf_cuda import DeviceGrid

# Every member of the set is uploaded once, and serves any number of batches.
grid = DeviceGrid("NNPDF40_nnlo_as_01180")

xs = np.array([1e-5, 1e-4, 1e-3, 1e-2, 1e-1, 0.3, 0.5, 0.8])
q2s = np.full_like(xs, 1e4)

# Shape (n_events, n_members, n_pids + 1): the last column holds alpha_s.
out = grid.xfxQ2_batch(xs, q2s)
gluon = out[:, :, grid.pids().index(21)]
uncertainty = gluon[:, 1:].std(axis=1, ddof=1)
alphas = out[:, 0, -1]

print(f"{'x':>10} {'Q2':>10} {'xg':>14} {'uncertainty':>14} {'alpha_s':>10}")
for x, q2, xg, unc, a_s in zip(xs, q2s, gluon[:, 0], uncertainty, alphas):
    print(f"{x:>10.1e} {q2:>10.1e} {xg:>14.6e} {unc:>14.6e} {a_s:>10.6f}")
