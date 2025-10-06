# LHAPDF Sets Benchmark

The following records the benchmark of `NeoPDF` against `LHAPDF` for all the available
`LHAPDF` sets using the same Cubic Hermite Splines interpolation. The benchmark are
done in the entire range of $(x, Q^2)$ for all PDF flavours and members. Sucess ✅ means
that the results are **exactly** the same, otherwise it is flagged as failed ❌.

=== "$xf_i (x, Q^2)$ comparison"

    !!! danger "Comments on failed runs"

        The failures in the computations of $xf_i (x, Q^2)$ are not genuine and the failed
        runs need to be re-generated in order for them to succeed. For instance, the
        `Items are not equal` error is a result of how negative values are addressed. For
        these PDF sets, negative values in `LHAPDF` are clipped to zero as harcoded in the
        metadata `ForcePositive: 1` while `NeoPDF` uses by default `ForcePositive::NoClipping`
        unless `ForcePositive::ClipNegative` is enforced. Upon doing so, the results will be
        exactly the same. Similarly, the error `Process failed: thread` is because, for some
        reasons, the sets were not available due to failures in downloading them.

    | PDF Set | Benchmark | Error Details |
    |---------|----------|---------------|
    | ABMP15_3_nnlo | ✅|  |
    | ABMP15_3_nnlo | ✅|  |
    | ABMP15_4_nnlo | ✅|  |
    | ABMP15_4_nnlo | ✅|  |
    | ABMP15_5_nnlo | ✅|  |
    | ABMP15_5_nnlo | ✅|  |
    | ABMP16_3_nlo | ✅|  |
    | ABMP16_3_nlo | ✅|  |
    | ABMP16_3_nnlo | ✅|  |
    | ABMP16_3_nnlo | ✅|  |
    | ABMP16_4_nlo | ✅|  |
    | ABMP16_4_nlo | ✅|  |
    | ABMP16_4_nnlo | ✅|  |
    | ABMP16_4_nnlo | ✅|  |
    | ABMP16_5_nlo | ✅|  |
    | ABMP16_5_nlo | ✅|  |
    | ABMP16_5_nnlo | ✅|  |
    | ABMP16_5_nnlo | ✅|  |
    | ABMP16_5_nnlo_high_x | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | ABMP16_5_nnlo_high_x | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | ABMP16als112_5_nnlo | ✅|  |
    | ABMP16als112_5_nnlo | ✅|  |
    | ABMP16als113_5_nnlo | ✅|  |
    | ABMP16als113_5_nnlo | ✅|  |
    | ABMP16als114_5_nlo | ✅|  |
    | ABMP16als114_5_nlo | ✅|  |
    | ABMP16als114_5_nnlo | ✅|  |
    | ABMP16als114_5_nnlo | ✅|  |
    | ABMP16als115_5_nlo | ✅|  |
    | ABMP16als115_5_nlo | ✅|  |
    | ABMP16als115_5_nnlo | ✅|  |
    | ABMP16als115_5_nnlo | ✅|  |
    | ABMP16als116_5_nlo | ✅|  |
    | ABMP16als116_5_nlo | ✅|  |
    | ABMP16als116_5_nnlo | ✅|  |
    | ABMP16als116_5_nnlo | ✅|  |
    | ABMP16als117_5_nlo | ✅|  |
    | ABMP16als117_5_nlo | ✅|  |
    | ABMP16als117_5_nnlo | ✅|  |
    | ABMP16als117_5_nnlo | ✅|  |
    | ABMP16als118_5_nlo | ✅|  |
    | ABMP16als118_5_nlo | ✅|  |
    | ABMP16als118_5_nnlo | ✅|  |
    | ABMP16als118_5_nnlo | ✅|  |
    | ABMP16als119_5_nlo | ✅|  |
    | ABMP16als119_5_nlo | ✅|  |
    | ABMP16als119_5_nnlo | ✅|  |
    | ABMP16als119_5_nnlo | ✅|  |
    | ABMP16als120_5_nlo | ✅|  |
    | ABMP16als120_5_nlo | ✅|  |
    | ABMP16als120_5_nnlo | ✅|  |
    | ABMP16als120_5_nnlo | ✅|  |
    | ABMP16als121_5_nlo | ✅|  |
    | ABMP16als121_5_nlo | ✅|  |
    | ABMP16als122_5_nlo | ✅|  |
    | ABMP16als122_5_nlo | ✅|  |
    | ABMP16als123_5_nlo | ✅|  |
    | ABMP16als123_5_nlo | ✅|  |
    | ABMP16free_3_nlo | ✅|  |
    | ABMP16free_3_nlo | ✅|  |
    | ABMP16free_4_nlo | ✅|  |
    | ABMP16free_4_nlo | ✅|  |
    | ABMP16free_5_nlo | ✅|  |
    | ABMP16free_5_nlo | ✅|  |
    | CT09MC1 | ❌| Items are not equal: ACTUAL: -0.012... |
    | CT09MC1 | ❌| Items are not equal: ACTUAL: -0.012... |
    | CT09MC2 | ❌| Items are not equal: ACTUAL: -1.961... |
    | CT09MC2 | ❌| Items are not equal: ACTUAL: -1.961... |
    | CT09MCS | ❌| Items are not equal: ACTUAL: -0.136... |
    | CT09MCS | ❌| Items are not equal: ACTUAL: -0.136... |
    | CT10 | ❌| Items are not equal: ACTUAL: -7.693... |
    | CT10 | ❌| Items are not equal: ACTUAL: -0.077... |
    | CT10as | ❌| Items are not equal: ACTUAL: -0.089... |
    | CT10as | ❌| Items are not equal: ACTUAL: -0.089... |
    | CT10f3 | ❌| Items are not equal: ACTUAL: -3.223... |
    | CT10f3 | ❌| Items are not equal: ACTUAL: -3.223... |
    | CT10f4 | ❌| Items are not equal: ACTUAL: -2.390... |
    | CT10f4 | ❌| Items are not equal: ACTUAL: -2.390... |
    | CT10nlo | ❌| Items are not equal: ACTUAL: -0.082... |
    | CT10nlo | ❌| Items are not equal: ACTUAL: -0.077... |
    | CT10nlo_as_0112 | ❌| Items are not equal: ACTUAL: -0.091... |
    | CT10nlo_as_0112 | ❌| Items are not equal: ACTUAL: -0.091... |
    | CT10nlo_as_0113 | ❌| Items are not equal: ACTUAL: -0.095... |
    | CT10nlo_as_0113 | ❌| Items are not equal: ACTUAL: -0.095... |
    | CT10nlo_as_0114 | ❌| Items are not equal: ACTUAL: -0.099... |
    | CT10nlo_as_0114 | ❌| Items are not equal: ACTUAL: -0.099... |
    | CT10nlo_as_0115 | ❌| Items are not equal: ACTUAL: -0.103... |
    | CT10nlo_as_0115 | ❌| Items are not equal: ACTUAL: -0.103... |
    | CT10nlo_as_0116 | ❌| Items are not equal: ACTUAL: -0.107... |
    | CT10nlo_as_0116 | ❌| Items are not equal: ACTUAL: -0.107... |
    | CT10nlo_as_0117 | ❌| Items are not equal: ACTUAL: -0.111... |
    | CT10nlo_as_0117 | ❌| Items are not equal: ACTUAL: -0.111... |
    | CT10nlo_as_0118 | ❌| Items are not equal: ACTUAL: -0.115... |
    | CT10nlo_as_0118 | ❌| Items are not equal: ACTUAL: -0.115... |
    | CT10nlo_as_0119 | ❌| Items are not equal: ACTUAL: -0.120... |
    | CT10nlo_as_0119 | ❌| Items are not equal: ACTUAL: -0.120... |
    | CT10nlo_as_0120 | ❌| Items are not equal: ACTUAL: -0.125... |
    | CT10nlo_as_0120 | ❌| Items are not equal: ACTUAL: -0.125... |
    | CT10nlo_as_0121 | ❌| Items are not equal: ACTUAL: -0.129... |
    | CT10nlo_as_0121 | ❌| Items are not equal: ACTUAL: -0.129... |
    | CT10nlo_as_0122 | ❌| Items are not equal: ACTUAL: -0.134... |
    | CT10nlo_as_0122 | ❌| Items are not equal: ACTUAL: -0.134... |
    | CT10nlo_as_0123 | ❌| Items are not equal: ACTUAL: -0.139... |
    | CT10nlo_as_0123 | ❌| Items are not equal: ACTUAL: -0.139... |
    | CT10nlo_as_0124 | ❌| Items are not equal: ACTUAL: -0.144... |
    | CT10nlo_as_0124 | ❌| Items are not equal: ACTUAL: -0.144... |
    | CT10nlo_as_0125 | ❌| Items are not equal: ACTUAL: -0.150... |
    | CT10nlo_as_0125 | ❌| Items are not equal: ACTUAL: -0.150... |
    | CT10nlo_as_0126 | ❌| Items are not equal: ACTUAL: -0.155... |
    | CT10nlo_as_0126 | ❌| Items are not equal: ACTUAL: -0.155... |
    | CT10nlo_as_0127 | ❌| Items are not equal: ACTUAL: -0.160... |
    | CT10nlo_as_0127 | ❌| Items are not equal: ACTUAL: -0.160... |
    | CT10nlo_nf3 | ❌| Items are not equal: ACTUAL: -1.609... |
    | CT10nlo_nf3 | ❌| Items are not equal: ACTUAL: -1.609... |
    | CT10nlo_nf4 | ❌| Items are not equal: ACTUAL: -2.414... |
    | CT10nlo_nf4 | ❌| Items are not equal: ACTUAL: -2.414... |
    | CT10nnlo | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0110 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0110 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0111 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0111 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0112 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0112 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0113 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0113 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0114 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0114 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0115 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0115 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0116 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0116 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0117 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0117 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0118 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0118 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0119 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0119 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0120 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0120 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0121 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0121 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT10nnlo_as_0122 | ❌| Items are not equal: ACTUAL: -0.000... |
    | CT10nnlo_as_0122 | ❌| Items are not equal: ACTUAL: -0.000... |
    | CT10nnlo_as_0123 | ❌| Items are not equal: ACTUAL: -0.000... |
    | CT10nnlo_as_0123 | ❌| Items are not equal: ACTUAL: -0.000... |
    | CT10nnlo_as_0124 | ❌| Items are not equal: ACTUAL: -0.000... |
    | CT10nnlo_as_0124 | ❌| Items are not equal: ACTUAL: -0.000... |
    | CT10nnlo_as_0125 | ❌| Items are not equal: ACTUAL: -0.000... |
    | CT10nnlo_as_0125 | ❌| Items are not equal: ACTUAL: -0.000... |
    | CT10nnlo_as_0126 | ❌| Items are not equal: ACTUAL: -0.000... |
    | CT10nnlo_as_0126 | ❌| Items are not equal: ACTUAL: -0.000... |
    | CT10nnlo_as_0127 | ❌| Items are not equal: ACTUAL: -0.000... |
    | CT10nnlo_as_0127 | ❌| Items are not equal: ACTUAL: -0.000... |
    | CT10nnlo_as_0128 | ❌| Items are not equal: ACTUAL: -0.000... |
    | CT10nnlo_as_0128 | ❌| Items are not equal: ACTUAL: -0.000... |
    | CT10nnlo_as_0129 | ❌| Items are not equal: ACTUAL: -7.094... |
    | CT10nnlo_as_0129 | ❌| Items are not equal: ACTUAL: -7.094... |
    | CT10nnlo_as_0130 | ❌| Items are not equal: ACTUAL: -2.418... |
    | CT10nnlo_as_0130 | ❌| Items are not equal: ACTUAL: -2.418... |
    | CT10w | ❌| Items are not equal: ACTUAL: -0.080... |
    | CT10w | ❌| Items are not equal: ACTUAL: -0.080... |
    | CT10was | ❌| Items are not equal: ACTUAL: -0.096... |
    | CT10was | ❌| Items are not equal: ACTUAL: -0.077... |
    | CT10wf3 | ❌| Items are not equal: ACTUAL: -1.421... |
    | CT10wf3 | ❌| Items are not equal: ACTUAL: -1.421... |
    | CT10wf4 | ❌| Items are not equal: ACTUAL: -1.514... |
    | CT10wf4 | ❌| Items are not equal: ACTUAL: -1.514... |
    | CT10wnlo | ❌| Items are not equal: ACTUAL: -0.081... |
    | CT10wnlo | ❌| Items are not equal: ACTUAL: -0.080... |
    | CT10wnlo_as_0112 | ❌| Items are not equal: ACTUAL: -0.092... |
    | CT10wnlo_as_0112 | ❌| Items are not equal: ACTUAL: -0.092... |
    | CT10wnlo_as_0113 | ❌| Items are not equal: ACTUAL: -0.095... |
    | CT10wnlo_as_0113 | ❌| Items are not equal: ACTUAL: -0.095... |
    | CT10wnlo_as_0114 | ❌| Items are not equal: ACTUAL: -0.099... |
    | CT10wnlo_as_0114 | ❌| Items are not equal: ACTUAL: -0.099... |
    | CT10wnlo_as_0115 | ❌| Items are not equal: ACTUAL: -0.103... |
    | CT10wnlo_as_0115 | ❌| Items are not equal: ACTUAL: -0.103... |
    | CT10wnlo_as_0116 | ❌| Items are not equal: ACTUAL: -0.108... |
    | CT10wnlo_as_0116 | ❌| Items are not equal: ACTUAL: -0.108... |
    | CT10wnlo_as_0117 | ❌| Items are not equal: ACTUAL: -0.112... |
    | CT10wnlo_as_0117 | ❌| Items are not equal: ACTUAL: -0.112... |
    | CT10wnlo_as_0118 | ❌| Items are not equal: ACTUAL: -0.116... |
    | CT10wnlo_as_0118 | ❌| Items are not equal: ACTUAL: -0.116... |
    | CT10wnlo_as_0119 | ❌| Items are not equal: ACTUAL: -0.121... |
    | CT10wnlo_as_0119 | ❌| Items are not equal: ACTUAL: -0.121... |
    | CT10wnlo_as_0120 | ❌| Items are not equal: ACTUAL: -0.126... |
    | CT10wnlo_as_0120 | ❌| Items are not equal: ACTUAL: -0.126... |
    | CT10wnlo_as_0121 | ❌| Items are not equal: ACTUAL: -0.130... |
    | CT10wnlo_as_0121 | ❌| Items are not equal: ACTUAL: -0.130... |
    | CT10wnlo_as_0122 | ❌| Items are not equal: ACTUAL: -0.135... |
    | CT10wnlo_as_0122 | ❌| Items are not equal: ACTUAL: -0.135... |
    | CT10wnlo_as_0123 | ❌| Items are not equal: ACTUAL: -0.140... |
    | CT10wnlo_as_0123 | ❌| Items are not equal: ACTUAL: -0.140... |
    | CT10wnlo_as_0124 | ❌| Items are not equal: ACTUAL: -0.145... |
    | CT10wnlo_as_0124 | ❌| Items are not equal: ACTUAL: -0.145... |
    | CT10wnlo_as_0125 | ❌| Items are not equal: ACTUAL: -0.150... |
    | CT10wnlo_as_0125 | ❌| Items are not equal: ACTUAL: -0.150... |
    | CT10wnlo_as_0126 | ❌| Items are not equal: ACTUAL: -0.155... |
    | CT10wnlo_as_0126 | ❌| Items are not equal: ACTUAL: -0.155... |
    | CT10wnlo_as_0127 | ❌| Items are not equal: ACTUAL: -0.161... |
    | CT10wnlo_as_0127 | ❌| Items are not equal: ACTUAL: -0.161... |
    | CT10wnlo_nf3 | ❌| Items are not equal: ACTUAL: -1.610... |
    | CT10wnlo_nf3 | ❌| Items are not equal: ACTUAL: -2.583... |
    | CT10wnlo_nf4 | ❌| Items are not equal: ACTUAL: -5.355... |
    | CT10wnlo_nf4 | ❌| Items are not equal: ACTUAL: -5.355... |
    | CT14MC1nlo | ✅|  |
    | CT14MC1nlo | ✅|  |
    | CT14MC1nnlo | ✅|  |
    | CT14MC1nnlo | ✅|  |
    | CT14MC2nlo | ✅|  |
    | CT14MC2nlo | ✅|  |
    | CT14MC2nnlo | ✅|  |
    | CT14MC2nnlo | ✅|  |
    | CT14llo | ❌| Items are not equal: ACTUAL: -0.016... |
    | CT14llo | ❌| Items are not equal: ACTUAL: -0.016... |
    | CT14llo_NF3 | ❌| Items are not equal: ACTUAL: -1.840... |
    | CT14llo_NF3 | ❌| Items are not equal: ACTUAL: -1.840... |
    | CT14llo_NF4 | ❌| Items are not equal: ACTUAL: -1.208... |
    | CT14llo_NF4 | ❌| Items are not equal: ACTUAL: -1.208... |
    | CT14llo_NF6 | ❌| Items are not equal: ACTUAL: -7.309... |
    | CT14llo_NF6 | ❌| Items are not equal: ACTUAL: -7.309... |
    | CT14lo | ❌| Items are not equal: ACTUAL: -0.011... |
    | CT14lo | ❌| Items are not equal: ACTUAL: -0.011... |
    | CT14lo_NF3 | ❌| Items are not equal: ACTUAL: -3.081... |
    | CT14lo_NF3 | ❌| Items are not equal: ACTUAL: -3.081... |
    | CT14lo_NF4 | ❌| Items are not equal: ACTUAL: -9.629... |
    | CT14lo_NF4 | ❌| Items are not equal: ACTUAL: -9.629... |
    | CT14lo_NF6 | ❌| Items are not equal: ACTUAL: -4.983... |
    | CT14lo_NF6 | ❌| Items are not equal: ACTUAL: -4.983... |
    | CT14nlo | ❌| Items are not equal: ACTUAL: -0.013... |
    | CT14nlo | ❌| Items are not equal: ACTUAL: -0.013... |
    | CT14nlo_Ag108 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT14nlo_Al27 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT14nlo_Au197 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT14nlo_Be9 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT14nlo_C12 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT14nlo_Ca40 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT14nlo_Cu64 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT14nlo_Fe56 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT14nlo_He4 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT14nlo_Li6 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT14nlo_NF3 | ❌| Items are not equal: ACTUAL: -9.131... |
    | CT14nlo_NF3 | ❌| Items are not equal: ACTUAL: -9.131... |
    | CT14nlo_NF4 | ❌| Items are not equal: ACTUAL: -8.834... |
    | CT14nlo_NF4 | ❌| Items are not equal: ACTUAL: -8.834... |
    | CT14nlo_NF6 | ❌| Items are not equal: ACTUAL: -4.300... |
    | CT14nlo_NF6 | ❌| Items are not equal: ACTUAL: -4.300... |
    | CT14nlo_Pb208 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT14nlo_Pt195 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT14nlo_Sn119 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT14nlo_W184 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT14nlo_as_0111 | ❌| Items are not equal: ACTUAL: -0.010... |
    | CT14nlo_as_0111 | ❌| Items are not equal: ACTUAL: -0.010... |
    | CT14nlo_as_0112 | ❌| Items are not equal: ACTUAL: -0.010... |
    | CT14nlo_as_0112 | ❌| Items are not equal: ACTUAL: -0.010... |
    | CT14nlo_as_0113 | ❌| Items are not equal: ACTUAL: -0.011... |
    | CT14nlo_as_0113 | ❌| Items are not equal: ACTUAL: -0.011... |
    | CT14nlo_as_0114 | ❌| Items are not equal: ACTUAL: -0.011... |
    | CT14nlo_as_0114 | ❌| Items are not equal: ACTUAL: -0.011... |
    | CT14nlo_as_0115 | ❌| Items are not equal: ACTUAL: -0.011... |
    | CT14nlo_as_0115 | ❌| Items are not equal: ACTUAL: -0.011... |
    | CT14nlo_as_0116 | ❌| Items are not equal: ACTUAL: -0.012... |
    | CT14nlo_as_0116 | ❌| Items are not equal: ACTUAL: -0.012... |
    | CT14nlo_as_0117 | ❌| Items are not equal: ACTUAL: -0.012... |
    | CT14nlo_as_0117 | ❌| Items are not equal: ACTUAL: -0.012... |
    | CT14nlo_as_0118 | ❌| Items are not equal: ACTUAL: -0.013... |
    | CT14nlo_as_0118 | ❌| Items are not equal: ACTUAL: -0.013... |
    | CT14nlo_as_0119 | ❌| Items are not equal: ACTUAL: -0.014... |
    | CT14nlo_as_0119 | ❌| Items are not equal: ACTUAL: -0.014... |
    | CT14nlo_as_0120 | ❌| Items are not equal: ACTUAL: -0.014... |
    | CT14nlo_as_0120 | ❌| Items are not equal: ACTUAL: -0.014... |
    | CT14nlo_as_0121 | ❌| Items are not equal: ACTUAL: -0.015... |
    | CT14nlo_as_0121 | ❌| Items are not equal: ACTUAL: -0.015... |
    | CT14nlo_as_0122 | ❌| Items are not equal: ACTUAL: -0.016... |
    | CT14nlo_as_0122 | ❌| Items are not equal: ACTUAL: -0.016... |
    | CT14nlo_as_0123 | ❌| Items are not equal: ACTUAL: -0.016... |
    | CT14nlo_as_0123 | ❌| Items are not equal: ACTUAL: -0.016... |
    | CT14nnlo | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnloIC | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnloIC | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_NF3 | ❌| Items are not equal: ACTUAL: -2.524... |
    | CT14nnlo_NF3 | ❌| Items are not equal: ACTUAL: -2.524... |
    | CT14nnlo_NF4 | ❌| Items are not equal: ACTUAL: -2.436... |
    | CT14nnlo_NF4 | ❌| Items are not equal: ACTUAL: -2.436... |
    | CT14nnlo_NF6 | ❌| Items are not equal: ACTUAL: -2.478... |
    | CT14nnlo_NF6 | ❌| Items are not equal: ACTUAL: -2.478... |
    | CT14nnlo_as_0111 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0111 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0112 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0112 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0113 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0113 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0114 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0114 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0115 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0115 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0116 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0116 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0117 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0117 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0118 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0118 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0119 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0119 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0120 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0120 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0121 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0121 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0122 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0122 | ❌| Items are not equal: ACTUAL: -0.001... |
    | CT14nnlo_as_0123 | ❌| Items are not equal: ACTUAL: -0.000... |
    | CT14nnlo_as_0123 | ❌| Items are not equal: ACTUAL: -0.000... |
    | CT14qed_inc_neutron | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT14qed_inc_neutron | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT14qed_inc_proton | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT14qed_inc_proton | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT14qed_neutron | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT14qed_neutron | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT14qed_proton | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT14qed_proton | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT18ANLO | ✅|  |
    | CT18ANLO | ✅|  |
    | CT18ANLO_as_0110 | ✅|  |
    | CT18ANLO_as_0110 | ✅|  |
    | CT18ANLO_as_0111 | ✅|  |
    | CT18ANLO_as_0111 | ✅|  |
    | CT18ANLO_as_0112 | ✅|  |
    | CT18ANLO_as_0112 | ✅|  |
    | CT18ANLO_as_0113 | ✅|  |
    | CT18ANLO_as_0113 | ✅|  |
    | CT18ANLO_as_0114 | ✅|  |
    | CT18ANLO_as_0114 | ✅|  |
    | CT18ANLO_as_0115 | ✅|  |
    | CT18ANLO_as_0115 | ✅|  |
    | CT18ANLO_as_0116 | ✅|  |
    | CT18ANLO_as_0116 | ✅|  |
    | CT18ANLO_as_0117 | ✅|  |
    | CT18ANLO_as_0117 | ✅|  |
    | CT18ANLO_as_0118 | ✅|  |
    | CT18ANLO_as_0118 | ✅|  |
    | CT18ANLO_as_0119 | ✅|  |
    | CT18ANLO_as_0119 | ✅|  |
    | CT18ANLO_as_0120 | ✅|  |
    | CT18ANLO_as_0120 | ✅|  |
    | CT18ANLO_as_0121 | ✅|  |
    | CT18ANLO_as_0121 | ✅|  |
    | CT18ANLO_as_0122 | ✅|  |
    | CT18ANLO_as_0122 | ✅|  |
    | CT18ANLO_as_0123 | ✅|  |
    | CT18ANLO_as_0123 | ✅|  |
    | CT18ANLO_as_0124 | ✅|  |
    | CT18ANLO_as_0124 | ✅|  |
    | CT18ANNLO | ✅|  |
    | CT18ANNLO | ✅|  |
    | CT18ANNLO_as_0110 | ✅|  |
    | CT18ANNLO_as_0110 | ✅|  |
    | CT18ANNLO_as_0111 | ✅|  |
    | CT18ANNLO_as_0111 | ✅|  |
    | CT18ANNLO_as_0112 | ✅|  |
    | CT18ANNLO_as_0112 | ✅|  |
    | CT18ANNLO_as_0113 | ✅|  |
    | CT18ANNLO_as_0113 | ✅|  |
    | CT18ANNLO_as_0114 | ✅|  |
    | CT18ANNLO_as_0114 | ✅|  |
    | CT18ANNLO_as_0115 | ✅|  |
    | CT18ANNLO_as_0115 | ✅|  |
    | CT18ANNLO_as_0116 | ✅|  |
    | CT18ANNLO_as_0116 | ✅|  |
    | CT18ANNLO_as_0117 | ✅|  |
    | CT18ANNLO_as_0117 | ✅|  |
    | CT18ANNLO_as_0118 | ✅|  |
    | CT18ANNLO_as_0118 | ✅|  |
    | CT18ANNLO_as_0119 | ✅|  |
    | CT18ANNLO_as_0119 | ✅|  |
    | CT18ANNLO_as_0120 | ✅|  |
    | CT18ANNLO_as_0120 | ✅|  |
    | CT18ANNLO_as_0121 | ✅|  |
    | CT18ANNLO_as_0121 | ✅|  |
    | CT18ANNLO_as_0122 | ✅|  |
    | CT18ANNLO_as_0122 | ✅|  |
    | CT18ANNLO_as_0123 | ✅|  |
    | CT18ANNLO_as_0123 | ✅|  |
    | CT18ANNLO_as_0124 | ✅|  |
    | CT18ANNLO_as_0124 | ✅|  |
    | CT18Anlo_Ag108 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT18Anlo_Al27 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT18Anlo_Ar40 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT18Anlo_Au197 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT18Anlo_Be9 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT18Anlo_C12 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT18Anlo_Ca40 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT18Anlo_Cu64 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT18Anlo_Fe56 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT18Anlo_He3 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT18Anlo_He4 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT18Anlo_Li6 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT18Anlo_O16 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT18Anlo_Pb208 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT18Anlo_Pt195 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT18Anlo_Sn119 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT18Anlo_W184 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | CT18FC | ✅|  |
    | CT18FC | ✅|  |
    | CT18LO | ✅|  |
    | CT18LO | ✅|  |
    | CT18NLO | ✅|  |
    | CT18NLO | ✅|  |
    | CT18NLO_NF3 | ✅|  |
    | CT18NLO_NF3 | ✅|  |
    | CT18NLO_NF4 | ✅|  |
    | CT18NLO_NF4 | ✅|  |
    | CT18NLO_NF6 | ✅|  |
    | CT18NLO_NF6 | ✅|  |
    | CT18NLO_as_0110 | ✅|  |
    | CT18NLO_as_0110 | ✅|  |
    | CT18NLO_as_0111 | ✅|  |
    | CT18NLO_as_0111 | ✅|  |
    | CT18NLO_as_0112 | ✅|  |
    | CT18NLO_as_0112 | ✅|  |
    | CT18NLO_as_0113 | ✅|  |
    | CT18NLO_as_0113 | ✅|  |
    | CT18NLO_as_0114 | ✅|  |
    | CT18NLO_as_0114 | ✅|  |
    | CT18NLO_as_0115 | ✅|  |
    | CT18NLO_as_0115 | ✅|  |
    | CT18NLO_as_0116 | ✅|  |
    | CT18NLO_as_0116 | ✅|  |
    | CT18NLO_as_0117 | ✅|  |
    | CT18NLO_as_0117 | ✅|  |
    | CT18NLO_as_0118 | ✅|  |
    | CT18NLO_as_0118 | ✅|  |
    | CT18NLO_as_0119 | ✅|  |
    | CT18NLO_as_0119 | ✅|  |
    | CT18NLO_as_0120 | ✅|  |
    | CT18NLO_as_0120 | ✅|  |
    | CT18NLO_as_0121 | ✅|  |
    | CT18NLO_as_0121 | ✅|  |
    | CT18NLO_as_0122 | ✅|  |
    | CT18NLO_as_0122 | ✅|  |
    | CT18NLO_as_0123 | ✅|  |
    | CT18NLO_as_0123 | ✅|  |
    | CT18NLO_as_0124 | ✅|  |
    | CT18NLO_as_0124 | ✅|  |
    | CT18NNLO | ✅|  |
    | CT18NNLO | ✅|  |
    | CT18NNLO_NF3 | ✅|  |
    | CT18NNLO_NF3 | ✅|  |
    | CT18NNLO_NF4 | ✅|  |
    | CT18NNLO_NF4 | ✅|  |
    | CT18NNLO_NF6 | ✅|  |
    | CT18NNLO_NF6 | ✅|  |
    | CT18NNLO_as_0110 | ✅|  |
    | CT18NNLO_as_0110 | ✅|  |
    | CT18NNLO_as_0111 | ✅|  |
    | CT18NNLO_as_0111 | ✅|  |
    | CT18NNLO_as_0112 | ✅|  |
    | CT18NNLO_as_0112 | ✅|  |
    | CT18NNLO_as_0113 | ✅|  |
    | CT18NNLO_as_0113 | ✅|  |
    | CT18NNLO_as_0114 | ✅|  |
    | CT18NNLO_as_0114 | ✅|  |
    | CT18NNLO_as_0115 | ✅|  |
    | CT18NNLO_as_0115 | ✅|  |
    | CT18NNLO_as_0116 | ✅|  |
    | CT18NNLO_as_0116 | ✅|  |
    | CT18NNLO_as_0117 | ✅|  |
    | CT18NNLO_as_0117 | ✅|  |
    | CT18NNLO_as_0118 | ✅|  |
    | CT18NNLO_as_0118 | ✅|  |
    | CT18NNLO_as_0119 | ✅|  |
    | CT18NNLO_as_0119 | ✅|  |
    | CT18NNLO_as_0120 | ✅|  |
    | CT18NNLO_as_0120 | ✅|  |
    | CT18NNLO_as_0121 | ✅|  |
    | CT18NNLO_as_0121 | ✅|  |
    | CT18NNLO_as_0122 | ✅|  |
    | CT18NNLO_as_0122 | ✅|  |
    | CT18NNLO_as_0123 | ✅|  |
    | CT18NNLO_as_0123 | ✅|  |
    | CT18NNLO_as_0124 | ✅|  |
    | CT18NNLO_as_0124 | ✅|  |
    | CT18XNLO | ✅|  |
    | CT18XNLO | ✅|  |
    | CT18XNLO_as_0110 | ✅|  |
    | CT18XNLO_as_0110 | ✅|  |
    | CT18XNLO_as_0111 | ✅|  |
    | CT18XNLO_as_0111 | ✅|  |
    | CT18XNLO_as_0112 | ✅|  |
    | CT18XNLO_as_0112 | ✅|  |
    | CT18XNLO_as_0113 | ✅|  |
    | CT18XNLO_as_0113 | ✅|  |
    | CT18XNLO_as_0114 | ✅|  |
    | CT18XNLO_as_0114 | ✅|  |
    | CT18XNLO_as_0115 | ✅|  |
    | CT18XNLO_as_0115 | ✅|  |
    | CT18XNLO_as_0116 | ✅|  |
    | CT18XNLO_as_0116 | ✅|  |
    | CT18XNLO_as_0117 | ✅|  |
    | CT18XNLO_as_0117 | ✅|  |
    | CT18XNLO_as_0118 | ✅|  |
    | CT18XNLO_as_0118 | ✅|  |
    | CT18XNLO_as_0119 | ✅|  |
    | CT18XNLO_as_0119 | ✅|  |
    | CT18XNLO_as_0120 | ✅|  |
    | CT18XNLO_as_0120 | ✅|  |
    | CT18XNLO_as_0121 | ✅|  |
    | CT18XNLO_as_0121 | ✅|  |
    | CT18XNLO_as_0122 | ✅|  |
    | CT18XNLO_as_0122 | ✅|  |
    | CT18XNLO_as_0123 | ✅|  |
    | CT18XNLO_as_0123 | ✅|  |
    | CT18XNLO_as_0124 | ✅|  |
    | CT18XNLO_as_0124 | ✅|  |
    | CT18XNNLO | ✅|  |
    | CT18XNNLO | ✅|  |
    | CT18XNNLO_as_0110 | ✅|  |
    | CT18XNNLO_as_0110 | ✅|  |
    | CT18XNNLO_as_0111 | ✅|  |
    | CT18XNNLO_as_0111 | ✅|  |
    | CT18XNNLO_as_0112 | ✅|  |
    | CT18XNNLO_as_0112 | ✅|  |
    | CT18XNNLO_as_0113 | ✅|  |
    | CT18XNNLO_as_0113 | ✅|  |
    | CT18XNNLO_as_0114 | ✅|  |
    | CT18XNNLO_as_0114 | ✅|  |
    | CT18XNNLO_as_0115 | ✅|  |
    | CT18XNNLO_as_0115 | ✅|  |
    | CT18XNNLO_as_0116 | ✅|  |
    | CT18XNNLO_as_0116 | ✅|  |
    | CT18XNNLO_as_0117 | ✅|  |
    | CT18XNNLO_as_0117 | ✅|  |
    | CT18XNNLO_as_0118 | ✅|  |
    | CT18XNNLO_as_0118 | ✅|  |
    | CT18XNNLO_as_0119 | ✅|  |
    | CT18XNNLO_as_0119 | ✅|  |
    | CT18XNNLO_as_0120 | ✅|  |
    | CT18XNNLO_as_0120 | ✅|  |
    | CT18XNNLO_as_0121 | ✅|  |
    | CT18XNNLO_as_0121 | ✅|  |
    | CT18XNNLO_as_0122 | ✅|  |
    | CT18XNNLO_as_0122 | ✅|  |
    | CT18XNNLO_as_0123 | ✅|  |
    | CT18XNNLO_as_0123 | ✅|  |
    | CT18XNNLO_as_0124 | ✅|  |
    | CT18XNNLO_as_0124 | ✅|  |
    | CT18ZNLO | ✅|  |
    | CT18ZNLO | ✅|  |
    | CT18ZNLO_NF3 | ✅|  |
    | CT18ZNLO_NF3 | ✅|  |
    | CT18ZNLO_NF4 | ✅|  |
    | CT18ZNLO_NF4 | ✅|  |
    | CT18ZNLO_NF6 | ✅|  |
    | CT18ZNLO_NF6 | ✅|  |
    | CT18ZNLO_as_0110 | ✅|  |
    | CT18ZNLO_as_0110 | ✅|  |
    | CT18ZNLO_as_0111 | ✅|  |
    | CT18ZNLO_as_0111 | ✅|  |
    | CT18ZNLO_as_0112 | ✅|  |
    | CT18ZNLO_as_0112 | ✅|  |
    | CT18ZNLO_as_0113 | ✅|  |
    | CT18ZNLO_as_0113 | ✅|  |
    | CT18ZNLO_as_0114 | ✅|  |
    | CT18ZNLO_as_0114 | ✅|  |
    | CT18ZNLO_as_0115 | ✅|  |
    | CT18ZNLO_as_0115 | ✅|  |
    | CT18ZNLO_as_0116 | ✅|  |
    | CT18ZNLO_as_0116 | ✅|  |
    | CT18ZNLO_as_0117 | ✅|  |
    | CT18ZNLO_as_0117 | ✅|  |
    | CT18ZNLO_as_0118 | ✅|  |
    | CT18ZNLO_as_0118 | ✅|  |
    | CT18ZNLO_as_0119 | ✅|  |
    | CT18ZNLO_as_0119 | ✅|  |
    | CT18ZNLO_as_0120 | ✅|  |
    | CT18ZNLO_as_0120 | ✅|  |
    | CT18ZNLO_as_0121 | ✅|  |
    | CT18ZNLO_as_0121 | ✅|  |
    | CT18ZNLO_as_0122 | ✅|  |
    | CT18ZNLO_as_0122 | ✅|  |
    | CT18ZNLO_as_0123 | ✅|  |
    | CT18ZNLO_as_0123 | ✅|  |
    | CT18ZNLO_as_0124 | ✅|  |
    | CT18ZNLO_as_0124 | ✅|  |
    | CT18ZNNLO | ✅|  |
    | CT18ZNNLO | ✅|  |
    | CT18ZNNLO_NF3 | ✅|  |
    | CT18ZNNLO_NF3 | ✅|  |
    | CT18ZNNLO_NF4 | ✅|  |
    | CT18ZNNLO_NF4 | ✅|  |
    | CT18ZNNLO_NF6 | ✅|  |
    | CT18ZNNLO_NF6 | ✅|  |
    | CT18ZNNLO_as_0110 | ✅|  |
    | CT18ZNNLO_as_0110 | ✅|  |
    | CT18ZNNLO_as_0111 | ✅|  |
    | CT18ZNNLO_as_0111 | ✅|  |
    | CT18ZNNLO_as_0112 | ✅|  |
    | CT18ZNNLO_as_0112 | ✅|  |
    | CT18ZNNLO_as_0113 | ✅|  |
    | CT18ZNNLO_as_0113 | ✅|  |
    | CT18ZNNLO_as_0114 | ✅|  |
    | CT18ZNNLO_as_0114 | ✅|  |
    | CT18ZNNLO_as_0115 | ✅|  |
    | CT18ZNNLO_as_0115 | ✅|  |
    | CT18ZNNLO_as_0116 | ✅|  |
    | CT18ZNNLO_as_0116 | ✅|  |
    | CT18ZNNLO_as_0117 | ✅|  |
    | CT18ZNNLO_as_0117 | ✅|  |
    | CT18ZNNLO_as_0118 | ✅|  |
    | CT18ZNNLO_as_0118 | ✅|  |
    | CT18ZNNLO_as_0119 | ✅|  |
    | CT18ZNNLO_as_0119 | ✅|  |
    | CT18ZNNLO_as_0120 | ✅|  |
    | CT18ZNNLO_as_0120 | ✅|  |
    | CT18ZNNLO_as_0121 | ✅|  |
    | CT18ZNNLO_as_0121 | ✅|  |
    | CT18ZNNLO_as_0122 | ✅|  |
    | CT18ZNNLO_as_0122 | ✅|  |
    | CT18ZNNLO_as_0123 | ✅|  |
    | CT18ZNNLO_as_0123 | ✅|  |
    | CT18ZNNLO_as_0124 | ✅|  |
    | CT18ZNNLO_as_0124 | ✅|  |
    | CT18qed_neutron | ✅|  |
    | CT18qed_neutron | ✅|  |
    | CT18qed_neutron_elastic | ✅|  |
    | CT18qed_neutron_elastic | ✅|  |
    | CT18qed_neutron_inelastic | ✅|  |
    | CT18qed_neutron_inelastic | ✅|  |
    | CT18qed_proton | ✅|  |
    | CT18qed_proton | ✅|  |
    | CT18qed_proton_elastic | ✅|  |
    | CT18qed_proton_elastic | ✅|  |
    | CT18qed_proton_inelastic | ✅|  |
    | CT18qed_proton_inelastic | ✅|  |
    | EPPS16_B_90CL_Au_hess | ✅|  |
    | EPPS16_B_90CL_Au_hess | ✅|  |
    | EPPS16_B_90CL_Pb_hess | ✅|  |
    | EPPS16_B_90CL_Pb_hess | ✅|  |
    | EPPS16_B_c_90CL_Au_hess | ✅|  |
    | EPPS16_B_c_90CL_Au_hess | ✅|  |
    | EPPS16_B_c_90CL_Pb_hess | ✅|  |
    | EPPS16_B_c_90CL_Pb_hess | ✅|  |
    | EPPS16_B_d_90CL_Au_hess | ✅|  |
    | EPPS16_B_d_90CL_Au_hess | ✅|  |
    | EPPS16_B_d_90CL_Pb_hess | ✅|  |
    | EPPS16_B_d_90CL_Pb_hess | ✅|  |
    | EPPS16_B_u_90CL_Au_hess | ✅|  |
    | EPPS16_B_u_90CL_Au_hess | ✅|  |
    | EPPS16_B_u_90CL_Pb_hess | ✅|  |
    | EPPS16_B_u_90CL_Pb_hess | ✅|  |
    | EPPS16_D_90CL_Au_hess | ✅|  |
    | EPPS16_D_90CL_Au_hess | ✅|  |
    | EPPS16_D_90CL_Pb_hess | ✅|  |
    | EPPS16_D_90CL_Pb_hess | ✅|  |
    | EPPS16_D_c_90CL_Au_hess | ✅|  |
    | EPPS16_D_c_90CL_Au_hess | ✅|  |
    | EPPS16_D_c_90CL_Pb_hess | ✅|  |
    | EPPS16_D_c_90CL_Pb_hess | ✅|  |
    | EPPS16_D_d_90CL_Au_hess | ✅|  |
    | EPPS16_D_d_90CL_Au_hess | ✅|  |
    | EPPS16_D_d_90CL_Pb_hess | ✅|  |
    | EPPS16_D_d_90CL_Pb_hess | ✅|  |
    | EPPS16_D_u_90CL_Au_hess | ✅|  |
    | EPPS16_D_u_90CL_Au_hess | ✅|  |
    | EPPS16_D_u_90CL_Pb_hess | ✅|  |
    | EPPS16_D_u_90CL_Pb_hess | ✅|  |
    | EPPS16_Jpsi_90CL_Au_hess | ✅|  |
    | EPPS16_Jpsi_90CL_Au_hess | ✅|  |
    | EPPS16_Jpsi_90CL_Pb_hess | ✅|  |
    | EPPS16_Jpsi_90CL_Pb_hess | ✅|  |
    | EPPS16_Jpsi_c_90CL_Au_hess | ✅|  |
    | EPPS16_Jpsi_c_90CL_Au_hess | ✅|  |
    | EPPS16_Jpsi_c_90CL_Pb_hess | ✅|  |
    | EPPS16_Jpsi_c_90CL_Pb_hess | ✅|  |
    | EPPS16_Jpsi_d_90CL_Au_hess | ✅|  |
    | EPPS16_Jpsi_d_90CL_Au_hess | ✅|  |
    | EPPS16_Jpsi_d_90CL_Pb_hess | ✅|  |
    | EPPS16_Jpsi_d_90CL_Pb_hess | ✅|  |
    | EPPS16_Jpsi_u_90CL_Au_hess | ✅|  |
    | EPPS16_Jpsi_u_90CL_Au_hess | ✅|  |
    | EPPS16_Jpsi_u_90CL_Pb_hess | ✅|  |
    | EPPS16_Jpsi_u_90CL_Pb_hess | ✅|  |
    | EPPS16nlo_CT14nlo_Ag108 | ✅|  |
    | EPPS16nlo_CT14nlo_Ag108 | ✅|  |
    | EPPS16nlo_CT14nlo_Al27 | ✅|  |
    | EPPS16nlo_CT14nlo_Al27 | ✅|  |
    | EPPS16nlo_CT14nlo_Au197 | ✅|  |
    | EPPS16nlo_CT14nlo_Au197 | ✅|  |
    | EPPS16nlo_CT14nlo_Be9 | ✅|  |
    | EPPS16nlo_CT14nlo_Be9 | ✅|  |
    | EPPS16nlo_CT14nlo_C12 | ✅|  |
    | EPPS16nlo_CT14nlo_C12 | ✅|  |
    | EPPS16nlo_CT14nlo_Ca40 | ✅|  |
    | EPPS16nlo_CT14nlo_Ca40 | ✅|  |
    | EPPS16nlo_CT14nlo_Cu64 | ✅|  |
    | EPPS16nlo_CT14nlo_Cu64 | ✅|  |
    | EPPS16nlo_CT14nlo_Fe56 | ✅|  |
    | EPPS16nlo_CT14nlo_Fe56 | ✅|  |
    | EPPS16nlo_CT14nlo_He4 | ✅|  |
    | EPPS16nlo_CT14nlo_He4 | ✅|  |
    | EPPS16nlo_CT14nlo_Li6 | ✅|  |
    | EPPS16nlo_CT14nlo_Li6 | ✅|  |
    | EPPS16nlo_CT14nlo_Pb208 | ✅|  |
    | EPPS16nlo_CT14nlo_Pb208 | ✅|  |
    | EPPS16nlo_CT14nlo_Pt195 | ✅|  |
    | EPPS16nlo_CT14nlo_Pt195 | ✅|  |
    | EPPS16nlo_CT14nlo_Sn119 | ✅|  |
    | EPPS16nlo_CT14nlo_Sn119 | ✅|  |
    | EPPS16nlo_CT14nlo_W184 | ✅|  |
    | EPPS16nlo_CT14nlo_W184 | ✅|  |
    | EPPS21nlo_CT18Anlo_Ag108 | ✅|  |
    | EPPS21nlo_CT18Anlo_Ag108 | ✅|  |
    | EPPS21nlo_CT18Anlo_Al27 | ✅|  |
    | EPPS21nlo_CT18Anlo_Al27 | ✅|  |
    | EPPS21nlo_CT18Anlo_Ar40 | ✅|  |
    | EPPS21nlo_CT18Anlo_Ar40 | ✅|  |
    | EPPS21nlo_CT18Anlo_Au197 | ✅|  |
    | EPPS21nlo_CT18Anlo_Au197 | ✅|  |
    | EPPS21nlo_CT18Anlo_Be9 | ✅|  |
    | EPPS21nlo_CT18Anlo_Be9 | ✅|  |
    | EPPS21nlo_CT18Anlo_C12 | ✅|  |
    | EPPS21nlo_CT18Anlo_C12 | ✅|  |
    | EPPS21nlo_CT18Anlo_Ca40 | ✅|  |
    | EPPS21nlo_CT18Anlo_Ca40 | ✅|  |
    | EPPS21nlo_CT18Anlo_Cu64 | ✅|  |
    | EPPS21nlo_CT18Anlo_Cu64 | ✅|  |
    | EPPS21nlo_CT18Anlo_Fe56 | ✅|  |
    | EPPS21nlo_CT18Anlo_Fe56 | ✅|  |
    | EPPS21nlo_CT18Anlo_He3 | ✅|  |
    | EPPS21nlo_CT18Anlo_He3 | ✅|  |
    | EPPS21nlo_CT18Anlo_He4 | ✅|  |
    | EPPS21nlo_CT18Anlo_He4 | ✅|  |
    | EPPS21nlo_CT18Anlo_Li6 | ✅|  |
    | EPPS21nlo_CT18Anlo_Li6 | ✅|  |
    | EPPS21nlo_CT18Anlo_O16 | ✅|  |
    | EPPS21nlo_CT18Anlo_O16 | ✅|  |
    | EPPS21nlo_CT18Anlo_Pb208 | ✅|  |
    | EPPS21nlo_CT18Anlo_Pb208 | ✅|  |
    | EPPS21nlo_CT18Anlo_Pt195 | ✅|  |
    | EPPS21nlo_CT18Anlo_Pt195 | ✅|  |
    | EPPS21nlo_CT18Anlo_Sn119 | ✅|  |
    | EPPS21nlo_CT18Anlo_Sn119 | ✅|  |
    | EPPS21nlo_CT18Anlo_W184 | ✅|  |
    | EPPS21nlo_CT18Anlo_W184 | ✅|  |
    | MMHT2014lo68cl | ✅|  |
    | MMHT2014lo68cl | ✅|  |
    | MMHT2014lo_asmzsmallrange | ✅|  |
    | MMHT2014lo_asmzsmallrange | ✅|  |
    | MMHT2014nlo68cl | ✅|  |
    | MMHT2014nlo68cl | ✅|  |
    | MMHT2014nlo68cl_nf3 | ✅|  |
    | MMHT2014nlo68cl_nf3 | ✅|  |
    | MMHT2014nlo68cl_nf4 | ✅|  |
    | MMHT2014nlo68cl_nf4 | ✅|  |
    | MMHT2014nlo68cl_nf4as5 | ✅|  |
    | MMHT2014nlo68cl_nf4as5 | ✅|  |
    | MMHT2014nlo68clas118 | ✅|  |
    | MMHT2014nlo68clas118 | ✅|  |
    | MMHT2014nlo68clas118_nf3 | ✅|  |
    | MMHT2014nlo68clas118_nf3 | ✅|  |
    | MMHT2014nlo68clas118_nf4 | ✅|  |
    | MMHT2014nlo68clas118_nf4 | ✅|  |
    | MMHT2014nlo68clas118_nf4as5 | ✅|  |
    | MMHT2014nlo68clas118_nf4as5 | ✅|  |
    | MMHT2014nlo_asmzlargerange | ✅|  |
    | MMHT2014nlo_asmzlargerange | ✅|  |
    | MMHT2014nlo_asmzsmallrange | ✅|  |
    | MMHT2014nlo_asmzsmallrange | ✅|  |
    | MMHT2014nlo_asmzsmallrange_nf3 | ✅|  |
    | MMHT2014nlo_asmzsmallrange_nf3 | ✅|  |
    | MMHT2014nlo_asmzsmallrange_nf4 | ✅|  |
    | MMHT2014nlo_asmzsmallrange_nf4 | ✅|  |
    | MMHT2014nlo_mbrange_nf3 | ✅|  |
    | MMHT2014nlo_mbrange_nf3 | ✅|  |
    | MMHT2014nlo_mbrange_nf4 | ✅|  |
    | MMHT2014nlo_mbrange_nf4 | ✅|  |
    | MMHT2014nlo_mbrange_nf5 | ✅|  |
    | MMHT2014nlo_mbrange_nf5 | ✅|  |
    | MMHT2014nlo_mcrange_nf3 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | MMHT2014nlo_mcrange_nf3 | ✅|  |
    | MMHT2014nlo_mcrange_nf4 | ✅|  |
    | MMHT2014nlo_mcrange_nf4 | ✅|  |
    | MMHT2014nlo_mcrange_nf5 | ✅|  |
    | MMHT2014nlo_mcrange_nf5 | ✅|  |
    | MMHT2014nloas118_mbrange_nf3 | ✅|  |
    | MMHT2014nloas118_mbrange_nf3 | ✅|  |
    | MMHT2014nloas118_mbrange_nf4 | ✅|  |
    | MMHT2014nloas118_mbrange_nf4 | ✅|  |
    | MMHT2014nloas118_mbrange_nf5 | ✅|  |
    | MMHT2014nloas118_mbrange_nf5 | ✅|  |
    | MMHT2014nloas118_mcrange_nf3 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | MMHT2014nloas118_mcrange_nf3 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | MMHT2014nloas118_mcrange_nf4 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | MMHT2014nloas118_mcrange_nf4 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | MMHT2014nloas118_mcrange_nf5 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | MMHT2014nloas118_mcrange_nf5 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | MMHT2014nnlo68cl | ✅|  |
    | MMHT2014nnlo68cl | ✅|  |
    | MMHT2014nnlo68cl_nf3 | ✅|  |
    | MMHT2014nnlo68cl_nf3 | ✅|  |
    | MMHT2014nnlo68cl_nf4 | ✅|  |
    | MMHT2014nnlo68cl_nf4 | ✅|  |
    | MMHT2014nnlo68cl_nf4as5 | ✅|  |
    | MMHT2014nnlo68cl_nf4as5 | ✅|  |
    | MMHT2014nnlo_asmzlargerange | ✅|  |
    | MMHT2014nnlo_asmzlargerange | ✅|  |
    | MMHT2014nnlo_asmzsmallrange | ✅|  |
    | MMHT2014nnlo_asmzsmallrange | ✅|  |
    | MMHT2014nnlo_asmzsmallrange_nf3 | ✅|  |
    | MMHT2014nnlo_asmzsmallrange_nf3 | ✅|  |
    | MMHT2014nnlo_asmzsmallrange_nf4 | ✅|  |
    | MMHT2014nnlo_asmzsmallrange_nf4 | ✅|  |
    | MMHT2014nnlo_mbrange_nf3 | ✅|  |
    | MMHT2014nnlo_mbrange_nf3 | ✅|  |
    | MMHT2014nnlo_mbrange_nf4 | ✅|  |
    | MMHT2014nnlo_mbrange_nf4 | ✅|  |
    | MMHT2014nnlo_mbrange_nf5 | ✅|  |
    | MMHT2014nnlo_mbrange_nf5 | ✅|  |
    | MMHT2014nnlo_mcrange_nf3 | ✅|  |
    | MMHT2014nnlo_mcrange_nf3 | ✅|  |
    | MMHT2014nnlo_mcrange_nf4 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | MMHT2014nnlo_mcrange_nf4 | ✅|  |
    | MMHT2014nnlo_mcrange_nf5 | ✅|  |
    | MMHT2014nnlo_mcrange_nf5 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | MMHT2015qed_nlo | ✅|  |
    | MMHT2015qed_nlo | ✅|  |
    | MMHT2015qed_nlo_elastic | ✅|  |
    | MMHT2015qed_nlo_elastic | ✅|  |
    | MMHT2015qed_nlo_inelastic | ✅|  |
    | MMHT2015qed_nlo_inelastic | ✅|  |
    | MMHT2015qed_nnlo | ✅|  |
    | MMHT2015qed_nnlo | ✅|  |
    | MMHT2015qed_nnlo_elastic | ✅|  |
    | MMHT2015qed_nnlo_elastic | ✅|  |
    | MMHT2015qed_nnlo_inelastic | ✅|  |
    | MMHT2015qed_nnlo_inelastic | ✅|  |
    | MSHT20an3lo_as118 | ✅|  |
    | MSHT20an3lo_as118 | ✅|  |
    | MSHT20an3lo_as118_Kcorr | ✅|  |
    | MSHT20an3lo_as118_Kcorr | ✅|  |
    | MSHT20an3lo_as_smallrange | ✅|  |
    | MSHT20an3lo_as_smallrange | ✅|  |
    | MSHT20lo_as130 | ✅|  |
    | MSHT20lo_as130 | ✅|  |
    | MSHT20nlo_as118 | ✅|  |
    | MSHT20nlo_as118 | ✅|  |
    | MSHT20nlo_as120 | ✅|  |
    | MSHT20nlo_as120 | ✅|  |
    | MSHT20nlo_as120_mbrange_nf3 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | MSHT20nlo_as120_mbrange_nf3 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | MSHT20nlo_as120_mbrange_nf4 | ✅|  |
    | MSHT20nlo_as120_mbrange_nf4 | ✅|  |
    | MSHT20nlo_as120_mbrange_nf5 | ✅|  |
    | MSHT20nlo_as120_mbrange_nf5 | ✅|  |
    | MSHT20nlo_as120_mcrange_nf3 | ✅|  |
    | MSHT20nlo_as120_mcrange_nf3 | ✅|  |
    | MSHT20nlo_as120_mcrange_nf4 | ✅|  |
    | MSHT20nlo_as120_mcrange_nf4 | ✅|  |
    | MSHT20nlo_as120_mcrange_nf5 | ✅|  |
    | MSHT20nlo_as120_mcrange_nf5 | ✅|  |
    | MSHT20nlo_as120_nf3 | ✅|  |
    | MSHT20nlo_as120_nf3 | ✅|  |
    | MSHT20nlo_as120_nf4 | ✅|  |
    | MSHT20nlo_as120_nf4 | ✅|  |
    | MSHT20nlo_as_largerange | ✅|  |
    | MSHT20nlo_as_largerange | ✅|  |
    | MSHT20nlo_as_smallrange | ✅|  |
    | MSHT20nlo_as_smallrange | ✅|  |
    | MSHT20nlo_as_smallrange_nf3 | ✅|  |
    | MSHT20nlo_as_smallrange_nf3 | ✅|  |
    | MSHT20nlo_as_smallrange_nf4 | ✅|  |
    | MSHT20nlo_as_smallrange_nf4 | ✅|  |
    | MSHT20nlo_mbrange_nf3 | ✅|  |
    | MSHT20nlo_mbrange_nf3 | ✅|  |
    | MSHT20nlo_mbrange_nf4 | ✅|  |
    | MSHT20nlo_mbrange_nf4 | ✅|  |
    | MSHT20nlo_mbrange_nf5 | ✅|  |
    | MSHT20nlo_mbrange_nf5 | ✅|  |
    | MSHT20nlo_mcrange_nf3 | ✅|  |
    | MSHT20nlo_mcrange_nf3 | ✅|  |
    | MSHT20nlo_mcrange_nf4 | ✅|  |
    | MSHT20nlo_mcrange_nf4 | ✅|  |
    | MSHT20nlo_mcrange_nf5 | ✅|  |
    | MSHT20nlo_mcrange_nf5 | ✅|  |
    | MSHT20nlo_nf3 | ✅|  |
    | MSHT20nlo_nf3 | ✅|  |
    | MSHT20nlo_nf4 | ✅|  |
    | MSHT20nlo_nf4 | ✅|  |
    | MSHT20nnlo_as118 | ✅|  |
    | MSHT20nnlo_as118 | ✅|  |
    | MSHT20nnlo_as_largerange | ✅|  |
    | MSHT20nnlo_as_largerange | ✅|  |
    | MSHT20nnlo_as_smallrange | ✅|  |
    | MSHT20nnlo_as_smallrange | ✅|  |
    | MSHT20nnlo_as_smallrange_nf3 | ✅|  |
    | MSHT20nnlo_as_smallrange_nf3 | ✅|  |
    | MSHT20nnlo_as_smallrange_nf4 | ✅|  |
    | MSHT20nnlo_as_smallrange_nf4 | ✅|  |
    | MSHT20nnlo_mbrange_nf3 | ✅|  |
    | MSHT20nnlo_mbrange_nf3 | ✅|  |
    | MSHT20nnlo_mbrange_nf4 | ✅|  |
    | MSHT20nnlo_mbrange_nf4 | ✅|  |
    | MSHT20nnlo_mbrange_nf5 | ✅|  |
    | MSHT20nnlo_mbrange_nf5 | ✅|  |
    | MSHT20nnlo_mcrange_nf3 | ✅|  |
    | MSHT20nnlo_mcrange_nf3 | ✅|  |
    | MSHT20nnlo_mcrange_nf4 | ✅|  |
    | MSHT20nnlo_mcrange_nf4 | ✅|  |
    | MSHT20nnlo_mcrange_nf5 | ✅|  |
    | MSHT20nnlo_mcrange_nf5 | ✅|  |
    | MSHT20nnlo_nf3 | ✅|  |
    | MSHT20nnlo_nf3 | ✅|  |
    | MSHT20nnlo_nf4 | ✅|  |
    | MSHT20nnlo_nf4 | ✅|  |
    | MSHT20qed_an3lo | ✅|  |
    | MSHT20qed_an3lo | ✅|  |
    | MSHT20qed_an3lo_elastic | ✅|  |
    | MSHT20qed_an3lo_elastic | ✅|  |
    | MSHT20qed_an3lo_inelastic | ✅|  |
    | MSHT20qed_an3lo_inelastic | ✅|  |
    | MSHT20qed_lo | ✅|  |
    | MSHT20qed_lo | ✅|  |
    | MSHT20qed_lo_elastic | ✅|  |
    | MSHT20qed_lo_elastic | ✅|  |
    | MSHT20qed_lo_inelastic | ✅|  |
    | MSHT20qed_lo_inelastic | ✅|  |
    | MSHT20qed_nnlo | ✅|  |
    | MSHT20qed_nnlo | ✅|  |
    | MSHT20qed_nnlo_elastic | ✅|  |
    | MSHT20qed_nnlo_elastic | ✅|  |
    | MSHT20qed_nnlo_inelastic | ✅|  |
    | MSHT20qed_nnlo_inelastic | ✅|  |
    | MSHT20qed_nnlo_neutron | ✅|  |
    | MSHT20qed_nnlo_neutron | ✅|  |
    | MSHT20qed_nnlo_neutron_elastic | ✅|  |
    | MSHT20qed_nnlo_neutron_elastic | ✅|  |
    | MSHT20qed_nnlo_neutron_inelastic | ✅|  |
    | MSHT20qed_nnlo_neutron_inelastic | ✅|  |
    | MSTW2008CPdeutnlo68cl | ✅|  |
    | MSTW2008CPdeutnlo68cl | ✅|  |
    | MSTW2008CPdeutnnlo68cl | ✅|  |
    | MSTW2008CPdeutnnlo68cl | ✅|  |
    | MSTW2008lo68cl | ❌| Items are not equal: ACTUAL: -1.061... |
    | MSTW2008lo68cl | ❌| Items are not equal: ACTUAL: -2.161... |
    | MSTW2008lo68cl_nf3 | ❌| Items are not equal: ACTUAL: -0.000... |
    | MSTW2008lo68cl_nf3 | ❌| Items are not equal: ACTUAL: -0.000... |
    | MSTW2008lo68cl_nf4 | ❌| Items are not equal: ACTUAL: -2.399... |
    | MSTW2008lo68cl_nf4 | ❌| Items are not equal: ACTUAL: -7.284... |
    | MSTW2008lo68cl_nf4as5 | ❌| Items are not equal: ACTUAL: -4.766... |
    | MSTW2008lo68cl_nf4as5 | ❌| Items are not equal: ACTUAL: -5.468... |
    | MSTW2008lo90cl | ❌| Items are not equal: ACTUAL: -1.305... |
    | MSTW2008lo90cl | ❌| Items are not equal: ACTUAL: -1.916... |
    | MSTW2008lo90cl_nf3 | ❌| Items are not equal: ACTUAL: -5.454... |
    | MSTW2008lo90cl_nf3 | ❌| Items are not equal: ACTUAL: -0.000... |
    | MSTW2008lo90cl_nf4 | ❌| Items are not equal: ACTUAL: -5.367... |
    | MSTW2008lo90cl_nf4 | ❌| Items are not equal: ACTUAL: -2.532... |
    | MSTW2008lo90cl_nf4as5 | ❌| Items are not equal: ACTUAL: -4.709... |
    | MSTW2008lo90cl_nf4as5 | ❌| Items are not equal: ACTUAL: -1.991... |
    | MSTW2008nlo68cl | ✅|  |
    | MSTW2008nlo68cl | ✅|  |
    | MSTW2008nlo68cl_asmz+68cl | ✅|  |
    | MSTW2008nlo68cl_asmz+68cl | ✅|  |
    | MSTW2008nlo68cl_asmz+68clhalf | ✅|  |
    | MSTW2008nlo68cl_asmz+68clhalf | ✅|  |
    | MSTW2008nlo68cl_asmz-68cl | ✅|  |
    | MSTW2008nlo68cl_asmz-68cl | ✅|  |
    | MSTW2008nlo68cl_asmz-68clhalf | ✅|  |
    | MSTW2008nlo68cl_asmz-68clhalf | ✅|  |
    | MSTW2008nlo68cl_nf3 | ✅|  |
    | MSTW2008nlo68cl_nf3 | ✅|  |
    | MSTW2008nlo68cl_nf4 | ✅|  |
    | MSTW2008nlo68cl_nf4 | ✅|  |
    | MSTW2008nlo68cl_nf4as5 | ✅|  |
    | MSTW2008nlo68cl_nf4as5 | ✅|  |
    | MSTW2008nlo90cl | ✅|  |
    | MSTW2008nlo90cl | ✅|  |
    | MSTW2008nlo90cl_asmz+90cl | ✅|  |
    | MSTW2008nlo90cl_asmz+90cl | ✅|  |
    | MSTW2008nlo90cl_asmz+90clhalf | ✅|  |
    | MSTW2008nlo90cl_asmz+90clhalf | ✅|  |
    | MSTW2008nlo90cl_asmz-90cl | ✅|  |
    | MSTW2008nlo90cl_asmz-90cl | ✅|  |
    | MSTW2008nlo90cl_asmz-90clhalf | ✅|  |
    | MSTW2008nlo90cl_asmz-90clhalf | ✅|  |
    | MSTW2008nlo90cl_nf3 | ✅|  |
    | MSTW2008nlo90cl_nf3 | ✅|  |
    | MSTW2008nlo90cl_nf4 | ✅|  |
    | MSTW2008nlo90cl_nf4 | ✅|  |
    | MSTW2008nlo90cl_nf4as5 | ✅|  |
    | MSTW2008nlo90cl_nf4as5 | ✅|  |
    | MSTW2008nlo_asmzrange | ✅|  |
    | MSTW2008nlo_asmzrange | ✅|  |
    | MSTW2008nlo_mbrange | ✅|  |
    | MSTW2008nlo_mbrange | ✅|  |
    | MSTW2008nlo_mbrange_nf4 | ✅|  |
    | MSTW2008nlo_mbrange_nf4 | ✅|  |
    | MSTW2008nlo_mcrange | ✅|  |
    | MSTW2008nlo_mcrange | ✅|  |
    | MSTW2008nlo_mcrange_fixasmz | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | MSTW2008nlo_mcrange_fixasmz | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | MSTW2008nlo_mcrange_fixasmz_nf3 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | MSTW2008nlo_mcrange_fixasmz_nf3 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | MSTW2008nlo_mcrange_nf3 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | MSTW2008nlo_mcrange_nf3 | ✅|  |
    | MSTW2008nnlo68cl | ✅|  |
    | MSTW2008nnlo68cl | ✅|  |
    | MSTW2008nnlo68cl_asmz+68cl | ✅|  |
    | MSTW2008nnlo68cl_asmz+68cl | ✅|  |
    | MSTW2008nnlo68cl_asmz+68clhalf | ✅|  |
    | MSTW2008nnlo68cl_asmz+68clhalf | ✅|  |
    | MSTW2008nnlo68cl_asmz-68cl | ✅|  |
    | MSTW2008nnlo68cl_asmz-68cl | ✅|  |
    | MSTW2008nnlo68cl_asmz-68clhalf | ✅|  |
    | MSTW2008nnlo68cl_asmz-68clhalf | ✅|  |
    | MSTW2008nnlo68cl_nf3 | ✅|  |
    | MSTW2008nnlo68cl_nf3 | ✅|  |
    | MSTW2008nnlo68cl_nf4 | ✅|  |
    | MSTW2008nnlo68cl_nf4 | ✅|  |
    | MSTW2008nnlo68cl_nf4as5 | ✅|  |
    | MSTW2008nnlo68cl_nf4as5 | ✅|  |
    | MSTW2008nnlo90cl | ✅|  |
    | MSTW2008nnlo90cl | ✅|  |
    | MSTW2008nnlo90cl_asmz+90cl | ✅|  |
    | MSTW2008nnlo90cl_asmz+90cl | ✅|  |
    | MSTW2008nnlo90cl_asmz+90clhalf | ✅|  |
    | MSTW2008nnlo90cl_asmz+90clhalf | ✅|  |
    | MSTW2008nnlo90cl_asmz-90cl | ✅|  |
    | MSTW2008nnlo90cl_asmz-90cl | ✅|  |
    | MSTW2008nnlo90cl_asmz-90clhalf | ✅|  |
    | MSTW2008nnlo90cl_asmz-90clhalf | ✅|  |
    | MSTW2008nnlo90cl_nf3 | ✅|  |
    | MSTW2008nnlo90cl_nf3 | ✅|  |
    | MSTW2008nnlo90cl_nf4 | ✅|  |
    | MSTW2008nnlo90cl_nf4 | ✅|  |
    | MSTW2008nnlo90cl_nf4as5 | ✅|  |
    | MSTW2008nnlo90cl_nf4as5 | ✅|  |
    | MSTW2008nnlo_asmzrange | ✅|  |
    | MSTW2008nnlo_asmzrange | ✅|  |
    | MSTW2008nnlo_mbrange | ✅|  |
    | MSTW2008nnlo_mbrange | ✅|  |
    | MSTW2008nnlo_mbrange_nf4 | ✅|  |
    | MSTW2008nnlo_mbrange_nf4 | ✅|  |
    | MSTW2008nnlo_mcrange | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | MSTW2008nnlo_mcrange | ✅|  |
    | MSTW2008nnlo_mcrange_fixasmz | ✅|  |
    | MSTW2008nnlo_mcrange_fixasmz | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | MSTW2008nnlo_mcrange_fixasmz_nf3 | ✅|  |
    | MSTW2008nnlo_mcrange_fixasmz_nf3 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | MSTW2008nnlo_mcrange_nf3 | ✅|  |
    | MSTW2008nnlo_mcrange_nf3 | ✅|  |
    | NNPDF10_nlo_as_0118_Ag108 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nlo_as_0118_Al27 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nlo_as_0118_Au197 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nlo_as_0118_Be9 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nlo_as_0118_C12 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nlo_as_0118_Ca40 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nlo_as_0118_Cu64 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nlo_as_0118_D2 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nlo_as_0118_Fe56 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nlo_as_0118_He4 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nlo_as_0118_Li6 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nlo_as_0118_N1 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nlo_as_0118_N14 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nlo_as_0118_Pb208 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nlo_as_0118_Sn119 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nlo_as_0118_Xe131 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nnlo_as_0118_Ag108 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nnlo_as_0118_Al27 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nnlo_as_0118_Au197 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nnlo_as_0118_Be9 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nnlo_as_0118_C12 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nnlo_as_0118_Ca40 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nnlo_as_0118_Cu64 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nnlo_as_0118_D2 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nnlo_as_0118_Fe56 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nnlo_as_0118_He4 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nnlo_as_0118_Li6 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nnlo_as_0118_N1 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nnlo_as_0118_N14 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nnlo_as_0118_Pb208 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nnlo_as_0118_Sn119 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF10_nnlo_as_0118_Xe131 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF20_nlo_as_0118_Ag108 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF20_nlo_as_0118_Al27 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF20_nlo_as_0118_Au197 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF20_nlo_as_0118_Be9 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF20_nlo_as_0118_C12 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF20_nlo_as_0118_Ca40 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF20_nlo_as_0118_Cu64 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF20_nlo_as_0118_D2 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF20_nlo_as_0118_Fe56 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF20_nlo_as_0118_He4 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF20_nlo_as_0118_Li6 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF20_nlo_as_0118_N1 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF20_nlo_as_0118_N14 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF20_nlo_as_0118_O16 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF20_nlo_as_0118_Pb208 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF20_nlo_as_0118_Sn119 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF20_nlo_as_0118_W184 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF20_nlo_as_0118_Xe131 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF21_lo_as_0119_100 | ✅|  |
    | NNPDF21_lo_as_0119_100 | ✅|  |
    | NNPDF21_lo_as_0130_100 | ✅|  |
    | NNPDF21_lo_as_0130_100 | ✅|  |
    | NNPDF21_lostar_as_0119_100 | ✅|  |
    | NNPDF21_lostar_as_0119_100 | ✅|  |
    | NNPDF21_lostar_as_0130_100 | ✅|  |
    | NNPDF21_lostar_as_0130_100 | ✅|  |
    | NNPDF23_lo_as_0119_qed | ✅|  |
    | NNPDF23_lo_as_0119_qed | ✅|  |
    | NNPDF23_lo_as_0130_qed | ✅|  |
    | NNPDF23_lo_as_0130_qed | ✅|  |
    | NNPDF23_nlo_FFN_NF4_as_0116 | ✅|  |
    | NNPDF23_nlo_FFN_NF4_as_0116 | ✅|  |
    | NNPDF23_nlo_FFN_NF4_as_0116_mc | ❌| Items are not equal: ACTUAL: 8.7016... |
    | NNPDF23_nlo_FFN_NF4_as_0116_mc | ❌| Items are not equal: ACTUAL: 1.5273... |
    | NNPDF23_nlo_FFN_NF4_as_0117 | ✅|  |
    | NNPDF23_nlo_FFN_NF4_as_0117 | ✅|  |
    | NNPDF23_nlo_FFN_NF4_as_0117_mc | ❌| Items are not equal: ACTUAL: 6.1374... |
    | NNPDF23_nlo_FFN_NF4_as_0117_mc | ❌| Items are not equal: ACTUAL: 6.1374... |
    | NNPDF23_nlo_FFN_NF4_as_0118 | ✅|  |
    | NNPDF23_nlo_FFN_NF4_as_0118 | ✅|  |
    | NNPDF23_nlo_FFN_NF4_as_0118_mc | ❌| Items are not equal: ACTUAL: 1.5323... |
    | NNPDF23_nlo_FFN_NF4_as_0118_mc | ❌| Items are not equal: ACTUAL: 4.4838... |
    | NNPDF23_nlo_FFN_NF4_as_0119 | ✅|  |
    | NNPDF23_nlo_FFN_NF4_as_0119 | ✅|  |
    | NNPDF23_nlo_FFN_NF4_as_0119_mc | ❌| Items are not equal: ACTUAL: 7.8774... |
    | NNPDF23_nlo_FFN_NF4_as_0119_mc | ❌| Items are not equal: ACTUAL: 2.8421... |
    | NNPDF23_nlo_FFN_NF4_as_0120 | ✅|  |
    | NNPDF23_nlo_FFN_NF4_as_0120 | ✅|  |
    | NNPDF23_nlo_FFN_NF4_as_0120_mc | ❌| Items are not equal: ACTUAL: 1.0368... |
    | NNPDF23_nlo_FFN_NF4_as_0120_mc | ❌| Items are not equal: ACTUAL: 2.9019... |
    | NNPDF23_nlo_FFN_NF5_as_0116 | ✅|  |
    | NNPDF23_nlo_FFN_NF5_as_0116 | ✅|  |
    | NNPDF23_nlo_FFN_NF5_as_0116_mc | ❌| Items are not equal: ACTUAL: 6.1374... |
    | NNPDF23_nlo_FFN_NF5_as_0116_mc | ❌| Items are not equal: ACTUAL: 3.6355... |
    | NNPDF23_nlo_FFN_NF5_as_0117 | ✅|  |
    | NNPDF23_nlo_FFN_NF5_as_0117 | ✅|  |
    | NNPDF23_nlo_FFN_NF5_as_0117_mc | ❌| Items are not equal: ACTUAL: 2.4429... |
    | NNPDF23_nlo_FFN_NF5_as_0117_mc | ❌| Items are not equal: ACTUAL: 3.8451... |
    | NNPDF23_nlo_FFN_NF5_as_0118 | ✅|  |
    | NNPDF23_nlo_FFN_NF5_as_0118 | ✅|  |
    | NNPDF23_nlo_FFN_NF5_as_0118_mc | ❌| Items are not equal: ACTUAL: 1.7905... |
    | NNPDF23_nlo_FFN_NF5_as_0118_mc | ❌| Items are not equal: ACTUAL: -1.137... |
    | NNPDF23_nlo_FFN_NF5_as_0119 | ✅|  |
    | NNPDF23_nlo_FFN_NF5_as_0119 | ✅|  |
    | NNPDF23_nlo_FFN_NF5_as_0119_mc | ❌| Items are not equal: ACTUAL: 1.6044... |
    | NNPDF23_nlo_FFN_NF5_as_0119_mc | ❌| Items are not equal: ACTUAL: 2.8204... |
    | NNPDF23_nlo_FFN_NF5_as_0120 | ✅|  |
    | NNPDF23_nlo_FFN_NF5_as_0120 | ✅|  |
    | NNPDF23_nlo_FFN_NF5_as_0120_mc | ❌| Items are not equal: ACTUAL: 6.1374... |
    | NNPDF23_nlo_FFN_NF5_as_0120_mc | ❌| Items are not equal: ACTUAL: 6.8937... |
    | NNPDF23_nlo_as_0114 | ✅|  |
    | NNPDF23_nlo_as_0114 | ✅|  |
    | NNPDF23_nlo_as_0115 | ✅|  |
    | NNPDF23_nlo_as_0115 | ✅|  |
    | NNPDF23_nlo_as_0116 | ✅|  |
    | NNPDF23_nlo_as_0116 | ✅|  |
    | NNPDF23_nlo_as_0116_mc | ❌| Items are not equal: ACTUAL: -1.263... |
    | NNPDF23_nlo_as_0116_mc | ❌| Items are not equal: ACTUAL: -1.177... |
    | NNPDF23_nlo_as_0117 | ✅|  |
    | NNPDF23_nlo_as_0117 | ✅|  |
    | NNPDF23_nlo_as_0117_mc | ❌| Items are not equal: ACTUAL: -1.470... |
    | NNPDF23_nlo_as_0117_mc | ❌| Items are not equal: ACTUAL: -1.248... |
    | NNPDF23_nlo_as_0117_qed | ✅|  |
    | NNPDF23_nlo_as_0117_qed | ✅|  |
    | NNPDF23_nlo_as_0117_qed_neutron | ✅|  |
    | NNPDF23_nlo_as_0117_qed_neutron | ✅|  |
    | NNPDF23_nlo_as_0118 | ✅|  |
    | NNPDF23_nlo_as_0118 | ✅|  |
    | NNPDF23_nlo_as_0118_mc | ❌| Items are not equal: ACTUAL: -2.122... |
    | NNPDF23_nlo_as_0118_mc | ❌| Items are not equal: ACTUAL: -1.461... |
    | NNPDF23_nlo_as_0118_qed | ✅|  |
    | NNPDF23_nlo_as_0118_qed | ✅|  |
    | NNPDF23_nlo_as_0118_qed_neutron | ✅|  |
    | NNPDF23_nlo_as_0118_qed_neutron | ✅|  |
    | NNPDF23_nlo_as_0119 | ✅|  |
    | NNPDF23_nlo_as_0119 | ✅|  |
    | NNPDF23_nlo_as_0119_mc | ❌| Items are not equal: ACTUAL: -1.185... |
    | NNPDF23_nlo_as_0119_mc | ❌| Items are not equal: ACTUAL: -4.122... |
    | NNPDF23_nlo_as_0119_qed | ✅|  |
    | NNPDF23_nlo_as_0119_qed | ✅|  |
    | NNPDF23_nlo_as_0119_qed_mc | ❌| Items are not equal: ACTUAL: 0.0 D... |
    | NNPDF23_nlo_as_0119_qed_mc | ❌| Items are not equal: ACTUAL: 0.0 D... |
    | NNPDF23_nlo_as_0119_qed_neutron | ✅|  |
    | NNPDF23_nlo_as_0119_qed_neutron | ✅|  |
    | NNPDF23_nlo_as_0120 | ✅|  |
    | NNPDF23_nlo_as_0120 | ✅|  |
    | NNPDF23_nlo_as_0120_mc | ❌| Items are not equal: ACTUAL: -1.141... |
    | NNPDF23_nlo_as_0120_mc | ❌| Items are not equal: ACTUAL: -1.418... |
    | NNPDF23_nlo_as_0121 | ✅|  |
    | NNPDF23_nlo_as_0121 | ✅|  |
    | NNPDF23_nlo_as_0122 | ✅|  |
    | NNPDF23_nlo_as_0122 | ✅|  |
    | NNPDF23_nlo_as_0123 | ✅|  |
    | NNPDF23_nlo_as_0123 | ✅|  |
    | NNPDF23_nlo_as_0124 | ✅|  |
    | NNPDF23_nlo_as_0124 | ✅|  |
    | NNPDF23_nlo_collider_as_0116 | ✅|  |
    | NNPDF23_nlo_collider_as_0116 | ✅|  |
    | NNPDF23_nlo_collider_as_0117 | ✅|  |
    | NNPDF23_nlo_collider_as_0117 | ✅|  |
    | NNPDF23_nlo_collider_as_0118 | ✅|  |
    | NNPDF23_nlo_collider_as_0118 | ✅|  |
    | NNPDF23_nlo_collider_as_0119 | ✅|  |
    | NNPDF23_nlo_collider_as_0119 | ✅|  |
    | NNPDF23_nlo_collider_as_0120 | ✅|  |
    | NNPDF23_nlo_collider_as_0120 | ✅|  |
    | NNPDF23_nlo_noLHC_as_0116 | ✅|  |
    | NNPDF23_nlo_noLHC_as_0116 | ✅|  |
    | NNPDF23_nlo_noLHC_as_0117 | ✅|  |
    | NNPDF23_nlo_noLHC_as_0117 | ✅|  |
    | NNPDF23_nlo_noLHC_as_0118 | ✅|  |
    | NNPDF23_nlo_noLHC_as_0118 | ✅|  |
    | NNPDF23_nlo_noLHC_as_0119 | ✅|  |
    | NNPDF23_nlo_noLHC_as_0119 | ✅|  |
    | NNPDF23_nlo_noLHC_as_0120 | ✅|  |
    | NNPDF23_nlo_noLHC_as_0120 | ✅|  |
    | NNPDF23_nnlo_FFN_NF4_as_0116 | ✅|  |
    | NNPDF23_nnlo_FFN_NF4_as_0116 | ✅|  |
    | NNPDF23_nnlo_FFN_NF4_as_0117 | ✅|  |
    | NNPDF23_nnlo_FFN_NF4_as_0117 | ✅|  |
    | NNPDF23_nnlo_FFN_NF4_as_0118 | ✅|  |
    | NNPDF23_nnlo_FFN_NF4_as_0118 | ✅|  |
    | NNPDF23_nnlo_FFN_NF4_as_0119 | ✅|  |
    | NNPDF23_nnlo_FFN_NF4_as_0119 | ✅|  |
    | NNPDF23_nnlo_FFN_NF4_as_0120 | ✅|  |
    | NNPDF23_nnlo_FFN_NF4_as_0120 | ✅|  |
    | NNPDF23_nnlo_FFN_NF5_as_0116 | ✅|  |
    | NNPDF23_nnlo_FFN_NF5_as_0116 | ✅|  |
    | NNPDF23_nnlo_FFN_NF5_as_0117 | ✅|  |
    | NNPDF23_nnlo_FFN_NF5_as_0117 | ✅|  |
    | NNPDF23_nnlo_FFN_NF5_as_0118 | ✅|  |
    | NNPDF23_nnlo_FFN_NF5_as_0118 | ✅|  |
    | NNPDF23_nnlo_FFN_NF5_as_0119 | ✅|  |
    | NNPDF23_nnlo_FFN_NF5_as_0119 | ✅|  |
    | NNPDF23_nnlo_FFN_NF5_as_0120 | ✅|  |
    | NNPDF23_nnlo_FFN_NF5_as_0120 | ✅|  |
    | NNPDF23_nnlo_as_0114 | ✅|  |
    | NNPDF23_nnlo_as_0114 | ✅|  |
    | NNPDF23_nnlo_as_0115 | ✅|  |
    | NNPDF23_nnlo_as_0115 | ✅|  |
    | NNPDF23_nnlo_as_0116 | ✅|  |
    | NNPDF23_nnlo_as_0116 | ✅|  |
    | NNPDF23_nnlo_as_0117 | ✅|  |
    | NNPDF23_nnlo_as_0117 | ✅|  |
    | NNPDF23_nnlo_as_0117_qed | ✅|  |
    | NNPDF23_nnlo_as_0117_qed | ✅|  |
    | NNPDF23_nnlo_as_0117_qed_neutron | ✅|  |
    | NNPDF23_nnlo_as_0117_qed_neutron | ✅|  |
    | NNPDF23_nnlo_as_0118 | ✅|  |
    | NNPDF23_nnlo_as_0118 | ✅|  |
    | NNPDF23_nnlo_as_0118_qed | ✅|  |
    | NNPDF23_nnlo_as_0118_qed | ✅|  |
    | NNPDF23_nnlo_as_0118_qed_neutron | ✅|  |
    | NNPDF23_nnlo_as_0118_qed_neutron | ✅|  |
    | NNPDF23_nnlo_as_0119 | ✅|  |
    | NNPDF23_nnlo_as_0119 | ✅|  |
    | NNPDF23_nnlo_as_0119_qed | ✅|  |
    | NNPDF23_nnlo_as_0119_qed | ✅|  |
    | NNPDF23_nnlo_as_0119_qed_mc | ❌| Items are not equal: ACTUAL: 0.0 D... |
    | NNPDF23_nnlo_as_0119_qed_mc | ❌| Items are not equal: ACTUAL: 0.0 D... |
    | NNPDF23_nnlo_as_0119_qed_neutron | ✅|  |
    | NNPDF23_nnlo_as_0119_qed_neutron | ✅|  |
    | NNPDF23_nnlo_as_0120 | ✅|  |
    | NNPDF23_nnlo_as_0120 | ✅|  |
    | NNPDF23_nnlo_as_0121 | ✅|  |
    | NNPDF23_nnlo_as_0121 | ✅|  |
    | NNPDF23_nnlo_as_0122 | ✅|  |
    | NNPDF23_nnlo_as_0122 | ✅|  |
    | NNPDF23_nnlo_as_0123 | ✅|  |
    | NNPDF23_nnlo_as_0123 | ✅|  |
    | NNPDF23_nnlo_as_0124 | ✅|  |
    | NNPDF23_nnlo_as_0124 | ✅|  |
    | NNPDF23_nnlo_collider_as_0116 | ✅|  |
    | NNPDF23_nnlo_collider_as_0116 | ✅|  |
    | NNPDF23_nnlo_collider_as_0117 | ✅|  |
    | NNPDF23_nnlo_collider_as_0117 | ✅|  |
    | NNPDF23_nnlo_collider_as_0118 | ✅|  |
    | NNPDF23_nnlo_collider_as_0118 | ✅|  |
    | NNPDF23_nnlo_collider_as_0119 | ✅|  |
    | NNPDF23_nnlo_collider_as_0119 | ✅|  |
    | NNPDF23_nnlo_collider_as_0120 | ✅|  |
    | NNPDF23_nnlo_collider_as_0120 | ✅|  |
    | NNPDF23_nnlo_noLHC_as_0116 | ✅|  |
    | NNPDF23_nnlo_noLHC_as_0116 | ✅|  |
    | NNPDF23_nnlo_noLHC_as_0117 | ✅|  |
    | NNPDF23_nnlo_noLHC_as_0117 | ✅|  |
    | NNPDF23_nnlo_noLHC_as_0118 | ✅|  |
    | NNPDF23_nnlo_noLHC_as_0118 | ✅|  |
    | NNPDF23_nnlo_noLHC_as_0119 | ✅|  |
    | NNPDF23_nnlo_noLHC_as_0119 | ✅|  |
    | NNPDF23_nnlo_noLHC_as_0120 | ✅|  |
    | NNPDF23_nnlo_noLHC_as_0120 | ✅|  |
    | NNPDF30_lo_as_0118 | ❌| Items are not equal: ACTUAL: -1.637... |
    | NNPDF30_nlo_as_0115 | ✅|  |
    | NNPDF30_nlo_as_0115_nf_3 | ✅|  |
    | NNPDF30_nlo_as_0115_nf_3 | ✅|  |
    | NNPDF30_nlo_as_0115_nf_4 | ✅|  |
    | NNPDF30_nlo_as_0115_nf_4 | ✅|  |
    | NNPDF30_nlo_as_0115_nf_6 | ✅|  |
    | NNPDF30_nlo_as_0115_nf_6 | ✅|  |
    | NNPDF30_nlo_as_0117 | ✅|  |
    | NNPDF30_nlo_as_0117 | ✅|  |
    | NNPDF30_nlo_as_0117_atlas | ✅|  |
    | NNPDF30_nlo_as_0117_atlas | ✅|  |
    | NNPDF30_nlo_as_0117_cms | ✅|  |
    | NNPDF30_nlo_as_0117_cms | ✅|  |
    | NNPDF30_nlo_as_0117_cons | ✅|  |
    | NNPDF30_nlo_as_0117_cons | ✅|  |
    | NNPDF30_nlo_as_0117_hera | ✅|  |
    | NNPDF30_nlo_as_0117_hera | ✅|  |
    | NNPDF30_nlo_as_0117_nf_3 | ✅|  |
    | NNPDF30_nlo_as_0117_nf_3 | ✅|  |
    | NNPDF30_nlo_as_0117_nf_4 | ✅|  |
    | NNPDF30_nlo_as_0117_nf_4 | ✅|  |
    | NNPDF30_nlo_as_0117_nf_6 | ✅|  |
    | NNPDF30_nlo_as_0117_nf_6 | ✅|  |
    | NNPDF30_nlo_as_0117_nojet | ✅|  |
    | NNPDF30_nlo_as_0117_nojet | ✅|  |
    | NNPDF30_nlo_as_0117_nolhc | ✅|  |
    | NNPDF30_nlo_as_0117_nolhc | ✅|  |
    | NNPDF30_nlo_as_0118 | ✅|  |
    | NNPDF30_nlo_as_0118 | ✅|  |
    | NNPDF30_nlo_as_0118_1000 | ✅|  |
    | NNPDF30_nlo_as_0118_1000 | ✅|  |
    | NNPDF30_nlo_as_0118_A108_Z54 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF30_nlo_as_0118_A119_Z59 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF30_nlo_as_0118_A12_Z6 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF30_nlo_as_0118_A131_Z54 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF30_nlo_as_0118_A14_Z7 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF30_nlo_as_0118_A16_Z8 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF30_nlo_as_0118_A184_Z74 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF30_nlo_as_0118_A197_Z79 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF30_nlo_as_0118_A208_Z82 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF30_nlo_as_0118_A27_Z13 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF30_nlo_as_0118_A2_Z1 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF30_nlo_as_0118_A31_Z15 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF30_nlo_as_0118_A40_Z20 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF30_nlo_as_0118_A4_Z2 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF30_nlo_as_0118_A56_Z26 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF30_nlo_as_0118_A64_Z29 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF30_nlo_as_0118_A6_Z3 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF30_nlo_as_0118_A9_Z4 | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF30_nlo_as_0118_atlas | ✅|  |
    | NNPDF30_nlo_as_0118_atlas | ✅|  |
    | NNPDF30_nlo_as_0118_cms | ✅|  |
    | NNPDF30_nlo_as_0118_cms | ✅|  |
    | NNPDF30_nlo_as_0118_cons | ✅|  |
    | NNPDF30_nlo_as_0118_cons | ✅|  |
    | NNPDF30_nlo_as_0118_hera | ✅|  |
    | NNPDF30_nlo_as_0118_hera | ✅|  |
    | NNPDF30_nlo_as_0118_hera_1000 | ✅|  |
    | NNPDF30_nlo_as_0118_hera_1000 | ✅|  |
    | NNPDF30_nlo_as_0118_hessian | ✅|  |
    | NNPDF30_nlo_as_0118_hessian | ✅|  |
    | NNPDF30_nlo_as_0118_mc | ✅|  |
    | NNPDF30_nlo_as_0118_mc | ✅|  |
    | NNPDF30_nlo_as_0118_nf_3 | ✅|  |
    | NNPDF30_nlo_as_0118_nf_3 | ✅|  |
    | NNPDF30_nlo_as_0118_nf_4 | ✅|  |
    | NNPDF30_nlo_as_0118_nf_4 | ✅|  |
    | NNPDF30_nlo_as_0118_nf_6 | ✅|  |
    | NNPDF30_nlo_as_0118_nf_6 | ✅|  |
    | NNPDF30_nlo_as_0118_nojet | ✅|  |
    | NNPDF30_nlo_as_0118_nojet | ✅|  |
    | NNPDF30_nlo_as_0118_nolhc | ✅|  |
    | NNPDF30_nlo_as_0118_nolhc | ✅|  |
    | NNPDF30_nlo_as_0118_nolhc_1000 | ✅|  |
    | NNPDF30_nlo_as_0118_nolhc_1000 | ✅|  |
    | NNPDF30_nlo_as_0118_p | ❌| Process failed: thread '<unnamed>' panicked at ne... |
    | NNPDF30_nlo_as_0119 | ✅|  |
    | NNPDF30_nlo_as_0119 | ✅|  |
    | NNPDF30_nlo_as_0119_atlas | ✅|  |
    | NNPDF30_nlo_as_0119_atlas | ✅|  |
    | NNPDF30_nlo_as_0119_cms | ✅|  |
    | NNPDF30_nlo_as_0119_cms | ✅|  |
    | NNPDF30_nlo_as_0119_cons | ✅|  |
    | NNPDF30_nlo_as_0119_cons | ✅|  |
    | NNPDF30_nlo_as_0119_hera | ✅|  |
    | NNPDF30_nlo_as_0119_hera | ✅|  |
    | NNPDF30_nlo_as_0119_nf_3 | ✅|  |
    | NNPDF30_nlo_as_0119_nf_3 | ✅|  |
    | NNPDF30_nlo_as_0119_nf_4 | ✅|  |
    | NNPDF30_nlo_as_0119_nf_4 | ✅|  |
    | NNPDF30_nlo_as_0119_nf_6 | ✅|  |
    | NNPDF30_nlo_as_0119_nf_6 | ✅|  |
    | NNPDF30_nlo_as_0119_nojet | ✅|  |
    | NNPDF30_nlo_as_0119_nojet | ✅|  |
    | NNPDF30_nlo_as_0119_nolhc | ✅|  |
    | NNPDF30_nlo_as_0119_nolhc | ✅|  |
    | NNPDF30_nlo_as_0121 | ✅|  |
    | NNPDF30_nlo_as_0121 | ✅|  |
    | NNPDF30_nlo_as_0121_nf_3 | ✅|  |
    | NNPDF30_nlo_as_0121_nf_3 | ✅|  |
    | NNPDF30_nlo_as_0121_nf_4 | ✅|  |
    | NNPDF30_nlo_as_0121_nf_4 | ✅|  |
    | NNPDF30_nlo_as_0121_nf_6 | ✅|  |
    | NNPDF30_nlo_as_0121_nf_6 | ✅|  |
    | NNPDF30_nlo_nf_4_pdfas | ✅|  |
    | NNPDF30_nlo_nf_4_pdfas | ✅|  |
    | NNPDF30_nlo_nf_5_pdfas | ✅|  |
    | NNPDF30_nlo_nf_5_pdfas | ✅|  |
    | NNPDF30_nnlo_as_0115 | ✅|  |
    | NNPDF30_nnlo_as_0115 | ✅|  |
    | NNPDF30_nnlo_as_0115_nf_3 | ✅|  |
    | NNPDF30_nnlo_as_0115_nf_3 | ✅|  |
    | NNPDF30_nnlo_as_0115_nf_4 | ✅|  |
    | NNPDF30_nnlo_as_0115_nf_4 | ✅|  |
    | NNPDF30_nnlo_as_0115_nf_6 | ✅|  |
    | NNPDF30_nnlo_as_0115_nf_6 | ✅|  |
    | NNPDF30_nnlo_as_0117 | ✅|  |
    | NNPDF30_nnlo_as_0117 | ✅|  |
    | NNPDF30_nnlo_as_0117_atlas | ✅|  |
    | NNPDF30_nnlo_as_0117_atlas | ✅|  |
    | NNPDF30_nnlo_as_0117_cms | ✅|  |
    | NNPDF30_nnlo_as_0117_cms | ✅|  |
    | NNPDF30_nnlo_as_0117_cons | ✅|  |
    | NNPDF30_nnlo_as_0117_cons | ✅|  |
    | NNPDF30_nnlo_as_0117_hera | ✅|  |
    | NNPDF30_nnlo_as_0117_hera | ✅|  |
    | NNPDF30_nnlo_as_0117_nf_3 | ✅|  |
    | NNPDF30_nnlo_as_0117_nf_3 | ✅|  |
    | NNPDF30_nnlo_as_0117_nf_4 | ✅|  |
    | NNPDF30_nnlo_as_0117_nf_4 | ✅|  |
    | NNPDF30_nnlo_as_0117_nf_6 | ✅|  |
    | NNPDF30_nnlo_as_0117_nf_6 | ✅|  |
    | NNPDF30_nnlo_as_0117_nojet | ✅|  |
    | NNPDF30_nnlo_as_0117_nojet | ✅|  |
    | NNPDF30_nnlo_as_0117_nolhc | ✅|  |
    | NNPDF30_nnlo_as_0117_nolhc | ✅|  |
    | NNPDF30_nnlo_as_0118 | ✅|  |
    | NNPDF30_nnlo_as_0118 | ✅|  |
    | NNPDF30_nnlo_as_0118_1000 | ✅|  |
    | NNPDF30_nnlo_as_0118_1000 | ✅|  |
    | NNPDF30_nnlo_as_0118_atlas | ✅|  |
    | NNPDF30_nnlo_as_0118_atlas | ✅|  |
    | NNPDF30_nnlo_as_0118_cms | ✅|  |
    | NNPDF30_nnlo_as_0118_cms | ✅|  |
    | NNPDF30_nnlo_as_0118_cons | ✅|  |
    | NNPDF30_nnlo_as_0118_cons | ✅|  |
    | NNPDF30_nnlo_as_0118_hera | ✅|  |
    | NNPDF30_nnlo_as_0118_hera | ✅|  |
    | NNPDF30_nnlo_as_0118_hera_1000 | ✅|  |
    | NNPDF30_nnlo_as_0118_hera_1000 | ✅|  |
    | NNPDF30_nnlo_as_0118_hessian | ✅|  |
    | NNPDF30_nnlo_as_0118_hessian | ✅|  |
    | NNPDF30_nnlo_as_0118_mc | ✅|  |
    | NNPDF30_nnlo_as_0118_mc | ✅|  |
    | NNPDF30_nnlo_as_0118_nf_3 | ✅|  |
    | NNPDF30_nnlo_as_0118_nf_3 | ✅|  |
    | NNPDF30_nnlo_as_0118_nf_4 | ✅|  |
    | NNPDF30_nnlo_as_0118_nf_4 | ✅|  |
    | NNPDF30_nnlo_as_0118_nf_6 | ✅|  |
    | NNPDF30_nnlo_as_0118_nf_6 | ✅|  |
    | NNPDF30_nnlo_as_0118_nojet | ✅|  |
    | NNPDF30_nnlo_as_0118_nojet | ✅|  |
    | NNPDF30_nnlo_as_0118_nolhc | ✅|  |
    | NNPDF30_nnlo_as_0118_nolhc | ✅|  |
    | NNPDF30_nnlo_as_0118_nolhc_1000 | ✅|  |
    | NNPDF30_nnlo_as_0118_nolhc_1000 | ✅|  |
    | NNPDF30_nnlo_as_0119 | ✅|  |
    | NNPDF30_nnlo_as_0119 | ✅|  |
    | NNPDF30_nnlo_as_0119_atlas | ✅|  |
    | NNPDF30_nnlo_as_0119_atlas | ✅|  |
    | NNPDF30_nnlo_as_0119_cms | ✅|  |
    | NNPDF30_nnlo_as_0119_cms | ✅|  |
    | NNPDF30_nnlo_as_0119_cons | ✅|  |
    | NNPDF30_nnlo_as_0119_cons | ✅|  |
    | NNPDF30_nnlo_as_0119_hera | ✅|  |
    | NNPDF30_nnlo_as_0119_hera | ✅|  |
    | NNPDF30_nnlo_as_0119_nf_3 | ✅|  |
    | NNPDF30_nnlo_as_0119_nf_3 | ✅|  |
    | NNPDF30_nnlo_as_0119_nf_4 | ✅|  |
    | NNPDF30_nnlo_as_0119_nf_4 | ✅|  |
    | NNPDF30_nnlo_as_0119_nf_6 | ✅|  |
    | NNPDF30_nnlo_as_0119_nf_6 | ✅|  |
    | NNPDF30_nnlo_as_0119_nojet | ✅|  |
    | NNPDF30_nnlo_as_0119_nojet | ✅|  |
    | NNPDF30_nnlo_as_0119_nolhc | ✅|  |
    | NNPDF30_nnlo_as_0119_nolhc | ✅|  |
    | NNPDF30_nnlo_as_0121 | ✅|  |
    | NNPDF30_nnlo_as_0121 | ✅|  |
    | NNPDF30_nnlo_as_0121_nf_3 | ✅|  |
    | NNPDF30_nnlo_as_0121_nf_3 | ✅|  |
    | NNPDF30_nnlo_as_0121_nf_4 | ✅|  |
    | NNPDF30_nnlo_as_0121_nf_4 | ✅|  |
    | NNPDF30_nnlo_as_0121_nf_6 | ✅|  |
    | NNPDF30_nnlo_as_0121_nf_6 | ✅|  |
    | NNPDF30_nnlo_nf_4_pdfas | ✅|  |
    | NNPDF30_nnlo_nf_4_pdfas | ✅|  |
    | NNPDF30_nnlo_nf_5_pdfas | ✅|  |
    | NNPDF30_nnlo_nf_5_pdfas | ✅|  |
    | NNPDF31_lo_as_0118 | ❌| Items are not equal: ACTUAL: -1.000... |
    | NNPDF31_lo_as_0118 | ❌| Items are not equal: ACTUAL: -1.985... |
    | NNPDF31_lo_as_0130 | ❌| Items are not equal: ACTUAL: -5.944... |
    | NNPDF31_lo_as_0130 | ❌| Items are not equal: ACTUAL: -9.536... |
    | NNPDF31_lo_pch_as_0118 | ❌| Items are not equal: ACTUAL: -3.177... |
    | NNPDF31_lo_pch_as_0118 | ❌| Items are not equal: ACTUAL: -1.164... |
    | NNPDF31_lo_pch_as_0130 | ❌| Items are not equal: ACTUAL: -1.192... |
    | NNPDF31_lo_pch_as_0130 | ❌| Items are not equal: ACTUAL: -3.972... |
    | NNPDF31_nlo_as_0116 | ✅|  |
    | NNPDF31_nlo_as_0116 | ✅|  |
    | NNPDF31_nlo_as_0118 | ✅|  |
    | NNPDF31_nlo_as_0118 | ✅|  |
    | NNPDF31_nlo_as_0118_1000 | ✅|  |
    | NNPDF31_nlo_as_0118_1000 | ✅|  |
    | NNPDF31_nlo_as_0118_C1p6 | ✅|  |
    | NNPDF31_nlo_as_0118_C1p6 | ✅|  |
    | NNPDF31_nlo_as_0118_hessian | ✅|  |
    | NNPDF31_nlo_as_0118_hessian | ✅|  |
    | NNPDF31_nlo_as_0118_luxqed | ✅|  |
    | NNPDF31_nlo_as_0118_luxqed | ✅|  |
    | NNPDF31_nlo_as_0118_mc | ✅|  |
    | NNPDF31_nlo_as_0118_mc | ✅|  |
    | NNPDF31_nlo_as_0118_nf_4 | ✅|  |
    | NNPDF31_nlo_as_0118_nf_4 | ✅|  |
    | NNPDF31_nlo_as_0118_nf_6 | ✅|  |
    | NNPDF31_nlo_as_0118_nf_6 | ✅|  |
    | NNPDF31_nlo_as_0120 | ✅|  |
    | NNPDF31_nlo_as_0120 | ✅|  |
    | NNPDF31_nlo_hessian_pdfas | ✅|  |
    | NNPDF31_nlo_hessian_pdfas | ✅|  |
    | NNPDF31_nlo_pch_as_0116 | ✅|  |
    | NNPDF31_nlo_pch_as_0116 | ✅|  |
    | NNPDF31_nlo_pch_as_0118 | ✅|  |
    | NNPDF31_nlo_pch_as_0118 | ✅|  |
    | NNPDF31_nlo_pch_as_0118_1000 | ✅|  |
    | NNPDF31_nlo_pch_as_0118_1000 | ✅|  |
    | NNPDF31_nlo_pch_as_0118_hessian | ✅|  |
    | NNPDF31_nlo_pch_as_0118_hessian | ✅|  |
    | NNPDF31_nlo_pch_as_0118_mc | ✅|  |
    | NNPDF31_nlo_pch_as_0118_mc | ✅|  |
    | NNPDF31_nlo_pch_as_0118_nf_3 | ✅|  |
    | NNPDF31_nlo_pch_as_0118_nf_3 | ✅|  |
    | NNPDF31_nlo_pch_as_0118_nf_4 | ✅|  |
    | NNPDF31_nlo_pch_as_0118_nf_4 | ✅|  |
    | NNPDF31_nlo_pch_as_0118_nf_6 | ✅|  |
    | NNPDF31_nlo_pch_as_0118_nf_6 | ✅|  |
    | NNPDF31_nlo_pch_as_0120 | ✅|  |
    | NNPDF31_nlo_pch_as_0120 | ✅|  |
    | NNPDF31_nlo_pch_hessian_pdfas | ✅|  |
    | NNPDF31_nlo_pch_hessian_pdfas | ✅|  |
    | NNPDF31_nlo_pch_pdfas | ✅|  |
    | NNPDF31_nlo_pch_pdfas | ✅|  |
    | NNPDF31_nlo_pdfas | ✅|  |
    | NNPDF31_nlo_pdfas | ✅|  |
    | NNPDF31_nnlo_as_0108 | ✅|  |
    | NNPDF31_nnlo_as_0108 | ✅|  |
    | NNPDF31_nnlo_as_0110 | ✅|  |
    | NNPDF31_nnlo_as_0110 | ✅|  |
    | NNPDF31_nnlo_as_0112 | ✅|  |
    | NNPDF31_nnlo_as_0112 | ✅|  |
    | NNPDF31_nnlo_as_0114 | ✅|  |
    | NNPDF31_nnlo_as_0114 | ✅|  |
    | NNPDF31_nnlo_as_0116 | ✅|  |
    | NNPDF31_nnlo_as_0116 | ✅|  |
    | NNPDF31_nnlo_as_0117 | ✅|  |
    | NNPDF31_nnlo_as_0117 | ✅|  |
    | NNPDF31_nnlo_as_0118 | ✅|  |
    | NNPDF31_nnlo_as_0118 | ✅|  |
    | NNPDF31_nnlo_as_0118_1000 | ✅|  |
    | NNPDF31_nnlo_as_0118_1000 | ✅|  |
    | NNPDF31_nnlo_as_0118_CMSW1 | ✅|  |
    | NNPDF31_nnlo_as_0118_CMSW1 | ✅|  |
    | NNPDF31_nnlo_as_0118_CMSW1_hessian_100 | ✅|  |
    | NNPDF31_nnlo_as_0118_CMSW1_hessian_100 | ✅|  |
    | NNPDF31_nnlo_as_0118_CMSW2 | ✅|  |
    | NNPDF31_nnlo_as_0118_CMSW2 | ✅|  |
    | NNPDF31_nnlo_as_0118_CMSW2_hessian_100 | ✅|  |
    | NNPDF31_nnlo_as_0118_CMSW2_hessian_100 | ✅|  |
    | NNPDF31_nnlo_as_0118_CMSW3 | ✅|  |
    | NNPDF31_nnlo_as_0118_CMSW3 | ✅|  |
    | NNPDF31_nnlo_as_0118_CMSW3_hessian_100 | ✅|  |
    | NNPDF31_nnlo_as_0118_CMSW3_hessian_100 | ✅|  |
    | NNPDF31_nnlo_as_0118_CMSW4 | ✅|  |
    | NNPDF31_nnlo_as_0118_CMSW4 | ✅|  |
    | NNPDF31_nnlo_as_0118_CMSW4_hessian_100 | ✅|  |
    | NNPDF31_nnlo_as_0118_CMSW4_hessian_100 | ✅|  |
    | NNPDF31_nnlo_as_0118_collider | ✅|  |
    | NNPDF31_nnlo_as_0118_collider | ✅|  |
    | NNPDF31_nnlo_as_0118_hessian | ✅|  |
    | NNPDF31_nnlo_as_0118_hessian | ✅|  |
    | NNPDF31_nnlo_as_0118_luxqed | ✅|  |
    | NNPDF31_nnlo_as_0118_luxqed | ✅|  |
    | NNPDF31_nnlo_as_0118_mc | ✅|  |
    | NNPDF31_nnlo_as_0118_mc | ✅|  |
    | NNPDF31_nnlo_as_0118_mc_138 | ✅|  |
    | NNPDF31_nnlo_as_0118_mc_138 | ✅|  |
    | NNPDF31_nnlo_as_0118_mc_164 | ✅|  |
    | NNPDF31_nnlo_as_0118_mc_164 | ✅|  |
    | NNPDF31_nnlo_as_0118_mc_hessian_pdfas | ✅|  |
    | NNPDF31_nnlo_as_0118_mc_hessian_pdfas | ✅|  |
    | NNPDF31_nnlo_as_0118_nf_4 | ✅|  |
    | NNPDF31_nnlo_as_0118_nf_4 | ✅|  |
    | NNPDF31_nnlo_as_0118_nf_4_mc_hessian | ✅|  |
    | NNPDF31_nnlo_as_0118_nf_4_mc_hessian | ✅|  |
    | NNPDF31_nnlo_as_0118_nf_6 | ✅|  |
    | NNPDF31_nnlo_as_0118_nf_6 | ✅|  |
    | NNPDF31_nnlo_as_0118_noLHC | ✅|  |
    | NNPDF31_nnlo_as_0118_noLHC | ✅|  |
    | NNPDF31_nnlo_as_0118_noZpt | ✅|  |
    | NNPDF31_nnlo_as_0118_noZpt | ✅|  |
    | NNPDF31_nnlo_as_0118_nojets | ✅|  |
    | NNPDF31_nnlo_as_0118_nojets | ✅|  |
    | NNPDF31_nnlo_as_0118_notop | ✅|  |
    | NNPDF31_nnlo_as_0118_notop | ✅|  |
    | NNPDF31_nnlo_as_0118_proton | ✅|  |
    | NNPDF31_nnlo_as_0118_proton | ✅|  |
    | NNPDF31_nnlo_as_0118_wEMC | ✅|  |
    | NNPDF31_nnlo_as_0118_wEMC | ✅|  |
    | NNPDF31_nnlo_as_0119 | ✅|  |
    | NNPDF31_nnlo_as_0119 | ✅|  |
    | NNPDF31_nnlo_as_0120 | ✅|  |
    | NNPDF31_nnlo_as_0120 | ✅|  |
    | NNPDF31_nnlo_as_0122 | ✅|  |
    | NNPDF31_nnlo_as_0122 | ✅|  |
    | NNPDF31_nnlo_as_0124 | ✅|  |
    | NNPDF31_nnlo_as_0124 | ✅|  |
    | NNPDF31_nnlo_hessian_pdfas | ✅|  |
    | NNPDF31_nnlo_hessian_pdfas | ✅|  |
    | NNPDF31_nnlo_pch_as_0116 | ✅|  |
    | NNPDF31_nnlo_pch_as_0116 | ✅|  |
    | NNPDF31_nnlo_pch_as_0118 | ✅|  |
    | NNPDF31_nnlo_pch_as_0118 | ✅|  |
    | NNPDF31_nnlo_pch_as_0118_1000 | ✅|  |
    | NNPDF31_nnlo_pch_as_0118_1000 | ✅|  |
    | NNPDF31_nnlo_pch_as_0118_hessian | ✅|  |
    | NNPDF31_nnlo_pch_as_0118_hessian | ✅|  |
    | NNPDF31_nnlo_pch_as_0118_mc | ✅|  |
    | NNPDF31_nnlo_pch_as_0118_mc | ✅|  |
    | NNPDF31_nnlo_pch_as_0118_mc_138 | ✅|  |
    | NNPDF31_nnlo_pch_as_0118_mc_138 | ✅|  |
    | NNPDF31_nnlo_pch_as_0118_mc_164 | ✅|  |
    | NNPDF31_nnlo_pch_as_0118_mc_164 | ✅|  |
    | NNPDF31_nnlo_pch_as_0118_nf_3 | ✅|  |
    | NNPDF31_nnlo_pch_as_0118_nf_3 | ✅|  |
    | NNPDF31_nnlo_pch_as_0118_nf_4 | ✅|  |
    | NNPDF31_nnlo_pch_as_0118_nf_4 | ✅|  |
    | NNPDF31_nnlo_pch_as_0118_nf_6 | ✅|  |
    | NNPDF31_nnlo_pch_as_0118_nf_6 | ✅|  |
    | NNPDF31_nnlo_pch_as_0120 | ✅|  |
    | NNPDF31_nnlo_pch_as_0120 | ✅|  |
    | NNPDF31_nnlo_pch_hessian_pdfas | ✅|  |
    | NNPDF31_nnlo_pch_hessian_pdfas | ✅|  |
    | NNPDF31_nnlo_pch_pdfas | ✅|  |
    | NNPDF31_nnlo_pch_pdfas | ✅|  |
    | NNPDF31_nnlo_pdfas | ✅|  |
    | NNPDF31_nnlo_pdfas | ✅|  |
    | NNPDF40MC_lo_as_01180 | ❌| Items are not equal: ACTUAL: -1.690... |
    | NNPDF40MC_lo_as_01180 | ❌| Items are not equal: ACTUAL: -1.690... |
    | NNPDF40MC_lo_as_01180_qed | ❌| Items are not equal: ACTUAL: -2.119... |
    | NNPDF40MC_lo_as_01180_qed | ❌| Items are not equal: ACTUAL: -2.119... |
    | NNPDF40MC_nlo_as_01180 | ❌| Items are not equal: ACTUAL: -1.694... |
    | NNPDF40MC_nlo_as_01180 | ❌| Items are not equal: ACTUAL: -1.694... |
    | NNPDF40MC_nlo_as_01180_qed | ❌| Items are not equal: ACTUAL: -2.068... |
    | NNPDF40MC_nlo_as_01180_qed | ❌| Items are not equal: ACTUAL: -2.068... |
    | NNPDF40MC_nnlo_as_01180 | ✅|  |
    | NNPDF40MC_nnlo_as_01180 | ✅|  |
    | NNPDF40MC_nnlo_as_01180_qed | ✅|  |
    | NNPDF40MC_nnlo_as_01180_qed | ✅|  |
    | NNPDF40_an3lo_as_01180 | ✅|  |
    | NNPDF40_an3lo_as_01180 | ✅|  |
    | NNPDF40_an3lo_as_01180_hessian | ✅|  |
    | NNPDF40_an3lo_as_01180_hessian | ✅|  |
    | NNPDF40_an3lo_as_01180_mhou | ✅|  |
    | NNPDF40_an3lo_as_01180_mhou | ✅|  |
    | NNPDF40_an3lo_as_01180_mhou_hessian | ✅|  |
    | NNPDF40_an3lo_as_01180_mhou_hessian | ✅|  |
    | NNPDF40_an3lo_as_01180_mhou_pdfas | ✅|  |
    | NNPDF40_an3lo_as_01180_mhou_pdfas | ✅|  |
    | NNPDF40_an3lo_as_01180_mhou_pdfas_hessian | ✅|  |
    | NNPDF40_an3lo_as_01180_mhou_pdfas_hessian | ✅|  |
    | NNPDF40_an3lo_as_01180_pdfas | ✅|  |
    | NNPDF40_an3lo_as_01180_pdfas | ✅|  |
    | NNPDF40_an3lo_as_01180_pdfas_hessian | ✅|  |
    | NNPDF40_an3lo_as_01180_pdfas_hessian | ✅|  |
    | NNPDF40_an3lo_as_01180_qed | ✅|  |
    | NNPDF40_an3lo_as_01180_qed | ✅|  |
    | NNPDF40_an3lo_as_01180_qed_mhou | ✅|  |
    | NNPDF40_an3lo_as_01180_qed_mhou | ✅|  |
    | NNPDF40_lo_as_01180 | ✅|  |
    | NNPDF40_lo_as_01180 | ✅|  |
    | NNPDF40_lo_pch_as_01180 | ✅|  |
    | NNPDF40_lo_pch_as_01180 | ✅|  |
    | NNPDF40_nlo_as_01170 | ✅|  |
    | NNPDF40_nlo_as_01170 | ✅|  |
    | NNPDF40_nlo_as_01180 | ✅|  |
    | NNPDF40_nlo_as_01180 | ✅|  |
    | NNPDF40_nlo_as_01180_mhou | ✅|  |
    | NNPDF40_nlo_as_01180_mhou | ✅|  |
    | NNPDF40_nlo_as_01180_nf_4 | ✅|  |
    | NNPDF40_nlo_as_01180_nf_4 | ✅|  |
    | NNPDF40_nlo_as_01180_nf_6 | ✅|  |
    | NNPDF40_nlo_as_01180_nf_6 | ✅|  |
    | NNPDF40_nlo_as_01180_qed | ✅|  |
    | NNPDF40_nlo_as_01180_qed | ✅|  |
    | NNPDF40_nlo_as_01190 | ✅|  |
    | NNPDF40_nlo_as_01190 | ✅|  |
    | NNPDF40_nlo_nf_4_pdfas | ✅|  |
    | NNPDF40_nlo_nf_4_pdfas | ✅|  |
    | NNPDF40_nlo_pch_as_01180 | ✅|  |
    | NNPDF40_nlo_pch_as_01180 | ✅|  |
    | NNPDF40_nlo_pch_as_01180_nf_3 | ✅|  |
    | NNPDF40_nlo_pch_as_01180_nf_3 | ✅|  |
    | NNPDF40_nnlo_as_01160 | ✅|  |
    | NNPDF40_nnlo_as_01160 | ✅|  |
    | NNPDF40_nnlo_as_01170 | ✅|  |
    | NNPDF40_nnlo_as_01170 | ✅|  |
    | NNPDF40_nnlo_as_01175 | ✅|  |
    | NNPDF40_nnlo_as_01175 | ✅|  |
    | NNPDF40_nnlo_as_01180 | ✅|  |
    | NNPDF40_nnlo_as_01180 | ✅|  |
    | NNPDF40_nnlo_as_01180_1000 | ✅|  |
    | NNPDF40_nnlo_as_01180_1000 | ✅|  |
    | NNPDF40_nnlo_as_01180_hessian | ✅|  |
    | NNPDF40_nnlo_as_01180_hessian | ✅|  |
    | NNPDF40_nnlo_as_01180_mhou | ✅|  |
    | NNPDF40_nnlo_as_01180_mhou | ✅|  |
    | NNPDF40_nnlo_as_01180_nf_4 | ✅|  |
    | NNPDF40_nnlo_as_01180_nf_4 | ✅|  |
    | NNPDF40_nnlo_as_01180_nf_6 | ✅|  |
    | NNPDF40_nnlo_as_01180_nf_6 | ✅|  |
    | NNPDF40_nnlo_as_01180_qed | ✅|  |
    | NNPDF40_nnlo_as_01180_qed | ✅|  |
    | NNPDF40_nnlo_as_01180_qed_mhou | ✅|  |
    | NNPDF40_nnlo_as_01180_qed_mhou | ✅|  |
    | NNPDF40_nnlo_as_01185 | ✅|  |
    | NNPDF40_nnlo_as_01185 | ✅|  |
    | NNPDF40_nnlo_as_01190 | ✅|  |
    | NNPDF40_nnlo_as_01190 | ✅|  |
    | NNPDF40_nnlo_as_01200 | ✅|  |
    | NNPDF40_nnlo_as_01200 | ✅|  |
    | NNPDF40_nnlo_hessian_pdfas | ✅|  |
    | NNPDF40_nnlo_hessian_pdfas | ✅|  |
    | NNPDF40_nnlo_nf_4_pdfas | ✅|  |
    | NNPDF40_nnlo_nf_4_pdfas | ✅|  |
    | NNPDF40_nnlo_pch_as_01180 | ✅|  |
    | NNPDF40_nnlo_pch_as_01180 | ✅|  |
    | NNPDF40_nnlo_pch_as_01180_nf_3 | ✅|  |
    | NNPDF40_nnlo_pch_as_01180_nf_3 | ✅|  |
    | NNPDF40_nnlo_pdfas | ✅|  |
    | NNPDF40_nnlo_pdfas | ✅|  |
    | NNPDFpol10_100 | ✅|  |
    | NNPDFpol10_100 | ✅|  |
    | NNPDFpol11_100 | ✅|  |
    | NNPDFpol11_100 | ✅|  |
    | PDF4LHC15_nlo_100 | ✅|  |
    | PDF4LHC15_nlo_100 | ✅|  |
    | PDF4LHC15_nlo_100_pdfas | ✅|  |
    | PDF4LHC15_nlo_100_pdfas | ✅|  |
    | PDF4LHC15_nlo_30 | ✅|  |
    | PDF4LHC15_nlo_30 | ✅|  |
    | PDF4LHC15_nlo_30_pdfas | ✅|  |
    | PDF4LHC15_nlo_30_pdfas | ✅|  |
    | PDF4LHC15_nlo_asvar | ✅|  |
    | PDF4LHC15_nlo_asvar | ✅|  |
    | PDF4LHC15_nlo_mc | ❌| Items are not equal: ACTUAL: 0.0 D... |
    | PDF4LHC15_nlo_mc | ❌| Items are not equal: ACTUAL: 0.0 D... |
    | PDF4LHC15_nlo_mc_pdfas | ❌| Items are not equal: ACTUAL: 0.0 D... |
    | PDF4LHC15_nlo_mc_pdfas | ❌| Items are not equal: ACTUAL: 0.0 D... |
    | PDF4LHC15_nlo_nf4_30 | ✅|  |
    | PDF4LHC15_nlo_nf4_30 | ✅|  |
    | PDF4LHC15_nnlo_100 | ✅|  |
    | PDF4LHC15_nnlo_100 | ✅|  |
    | PDF4LHC15_nnlo_100_pdfas | ✅|  |
    | PDF4LHC15_nnlo_100_pdfas | ✅|  |
    | PDF4LHC15_nnlo_30 | ✅|  |
    | PDF4LHC15_nnlo_30 | ✅|  |
    | PDF4LHC15_nnlo_30_pdfas | ✅|  |
    | PDF4LHC15_nnlo_30_pdfas | ✅|  |
    | PDF4LHC15_nnlo_asvar | ✅|  |
    | PDF4LHC15_nnlo_asvar | ✅|  |
    | PDF4LHC15_nnlo_mc | ❌| Items are not equal: ACTUAL: 0.0 D... |
    | PDF4LHC15_nnlo_mc | ✅|  |
    | PDF4LHC15_nnlo_mc_pdfas | ❌| Items are not equal: ACTUAL: 0.0 D... |
    | PDF4LHC15_nnlo_mc_pdfas | ❌| Items are not equal: ACTUAL: 0.0 D... |
    | PDF4LHC21_40 | ✅|  |
    | PDF4LHC21_40 | ✅|  |
    | PDF4LHC21_40_nf4 | ✅|  |
    | PDF4LHC21_40_nf4 | ✅|  |
    | PDF4LHC21_40_pdfas | ✅|  |
    | PDF4LHC21_40_pdfas | ✅|  |
    | PDF4LHC21_40_pdfas_nf4 | ✅|  |
    | PDF4LHC21_40_pdfas_nf4 | ✅|  |
    | PDF4LHC21_mc | ❌| Items are not equal: ACTUAL: nan D... |
    | PDF4LHC21_mc | ❌| Items are not equal: ACTUAL: nan D... |
    | PDF4LHC21_mc_nf4 | ✅|  |
    | PDF4LHC21_mc_nf4 | ✅|  |
    | PDF4LHC21_mc_pdfas | ❌| Items are not equal: ACTUAL: nan D... |
    | PDF4LHC21_mc_pdfas | ❌| Items are not equal: ACTUAL: nan D... |
    | PDF4LHC21_mc_pdfas_nf4 | ✅|  |
    | PDF4LHC21_mc_pdfas_nf4 | ✅|  |
    | TUJU19_nlo_119_50 | ✅|  |
    | TUJU19_nlo_119_50 | ✅|  |
    | TUJU19_nlo_12_6 | ✅|  |
    | TUJU19_nlo_12_6 | ✅|  |
    | TUJU19_nlo_131_54 | ✅|  |
    | TUJU19_nlo_131_54 | ✅|  |
    | TUJU19_nlo_197_79 | ✅|  |
    | TUJU19_nlo_197_79 | ✅|  |
    | TUJU19_nlo_1_1 | ✅|  |
    | TUJU19_nlo_1_1 | ✅|  |
    | TUJU19_nlo_208_82 | ✅|  |
    | TUJU19_nlo_208_82 | ✅|  |
    | TUJU19_nlo_27_13 | ✅|  |
    | TUJU19_nlo_27_13 | ✅|  |
    | TUJU19_nlo_2_1 | ✅|  |
    | TUJU19_nlo_2_1 | ✅|  |
    | TUJU19_nlo_3_2 | ✅|  |
    | TUJU19_nlo_3_2 | ✅|  |
    | TUJU19_nlo_40_20 | ✅|  |
    | TUJU19_nlo_40_20 | ✅|  |
    | TUJU19_nlo_4_2 | ✅|  |
    | TUJU19_nlo_4_2 | ✅|  |
    | TUJU19_nlo_56_26 | ✅|  |
    | TUJU19_nlo_56_26 | ✅|  |
    | TUJU19_nlo_64_29 | ✅|  |
    | TUJU19_nlo_64_29 | ✅|  |
    | TUJU19_nlo_7_3 | ✅|  |
    | TUJU19_nlo_7_3 | ✅|  |
    | TUJU19_nnlo_119_50 | ✅|  |
    | TUJU19_nnlo_119_50 | ✅|  |
    | TUJU19_nnlo_12_6 | ✅|  |
    | TUJU19_nnlo_12_6 | ✅|  |
    | TUJU19_nnlo_131_54 | ✅|  |
    | TUJU19_nnlo_131_54 | ✅|  |
    | TUJU19_nnlo_197_79 | ✅|  |
    | TUJU19_nnlo_197_79 | ✅|  |
    | TUJU19_nnlo_1_1 | ✅|  |
    | TUJU19_nnlo_1_1 | ✅|  |
    | TUJU19_nnlo_208_82 | ✅|  |
    | TUJU19_nnlo_208_82 | ✅|  |
    | TUJU19_nnlo_27_13 | ✅|  |
    | TUJU19_nnlo_27_13 | ✅|  |
    | TUJU19_nnlo_2_1 | ✅|  |
    | TUJU19_nnlo_2_1 | ✅|  |
    | TUJU19_nnlo_3_2 | ✅|  |
    | TUJU19_nnlo_3_2 | ✅|  |
    | TUJU19_nnlo_40_20 | ✅|  |
    | TUJU19_nnlo_40_20 | ✅|  |
    | TUJU19_nnlo_4_2 | ✅|  |
    | TUJU19_nnlo_4_2 | ✅|  |
    | TUJU19_nnlo_56_26 | ✅|  |
    | TUJU19_nnlo_56_26 | ✅|  |
    | TUJU19_nnlo_64_29 | ✅|  |
    | TUJU19_nnlo_64_29 | ✅|  |
    | TUJU19_nnlo_7_3 | ✅|  |
    | TUJU19_nnlo_7_3 | ✅|  |
    | TUJU21_nlo_12_6 | ✅|  |
    | TUJU21_nlo_12_6 | ✅|  |
    | TUJU21_nlo_131_54 | ✅|  |
    | TUJU21_nlo_131_54 | ✅|  |
    | TUJU21_nlo_14_7 | ✅|  |
    | TUJU21_nlo_14_7 | ✅|  |
    | TUJU21_nlo_16_8 | ✅|  |
    | TUJU21_nlo_16_8 | ✅|  |
    | TUJU21_nlo_197_79 | ✅|  |
    | TUJU21_nlo_197_79 | ✅|  |
    | TUJU21_nlo_1_1 | ✅|  |
    | TUJU21_nlo_1_1 | ✅|  |
    | TUJU21_nlo_208_82 | ✅|  |
    | TUJU21_nlo_208_82 | ✅|  |
    | TUJU21_nlo_27_13 | ✅|  |
    | TUJU21_nlo_27_13 | ✅|  |
    | TUJU21_nlo_2_1 | ✅|  |
    | TUJU21_nlo_2_1 | ✅|  |
    | TUJU21_nlo_4_2 | ✅|  |
    | TUJU21_nlo_4_2 | ✅|  |
    | TUJU21_nlo_56_26 | ✅|  |
    | TUJU21_nlo_56_26 | ✅|  |
    | TUJU21_nnlo_12_6 | ✅|  |
    | TUJU21_nnlo_12_6 | ✅|  |
    | TUJU21_nnlo_131_54 | ✅|  |
    | TUJU21_nnlo_131_54 | ✅|  |
    | TUJU21_nnlo_14_7 | ✅|  |
    | TUJU21_nnlo_14_7 | ✅|  |
    | TUJU21_nnlo_16_8 | ✅|  |
    | TUJU21_nnlo_16_8 | ✅|  |
    | TUJU21_nnlo_197_79 | ✅|  |
    | TUJU21_nnlo_197_79 | ✅|  |
    | TUJU21_nnlo_1_1 | ✅|  |
    | TUJU21_nnlo_1_1 | ✅|  |
    | TUJU21_nnlo_208_82 | ✅|  |
    | TUJU21_nnlo_208_82 | ✅|  |
    | TUJU21_nnlo_27_13 | ✅|  |
    | TUJU21_nnlo_27_13 | ✅|  |
    | TUJU21_nnlo_2_1 | ✅|  |
    | TUJU21_nnlo_2_1 | ✅|  |
    | TUJU21_nnlo_4_2 | ✅|  |
    | TUJU21_nnlo_4_2 | ✅|  |
    | TUJU21_nnlo_56_26 | ✅|  |
    | TUJU21_nnlo_56_26 | ✅|  |
    | nNNPDF10_nlo_as_0118_Ag108 | ✅|  |
    | nNNPDF10_nlo_as_0118_Ag108 | ✅|  |
    | nNNPDF10_nlo_as_0118_Al27 | ✅|  |
    | nNNPDF10_nlo_as_0118_Al27 | ✅|  |
    | nNNPDF10_nlo_as_0118_Au197 | ✅|  |
    | nNNPDF10_nlo_as_0118_Au197 | ✅|  |
    | nNNPDF10_nlo_as_0118_Be9 | ✅|  |
    | nNNPDF10_nlo_as_0118_Be9 | ✅|  |
    | nNNPDF10_nlo_as_0118_C12 | ✅|  |
    | nNNPDF10_nlo_as_0118_C12 | ✅|  |
    | nNNPDF10_nlo_as_0118_Ca40 | ✅|  |
    | nNNPDF10_nlo_as_0118_Ca40 | ✅|  |
    | nNNPDF10_nlo_as_0118_Cu64 | ✅|  |
    | nNNPDF10_nlo_as_0118_Cu64 | ✅|  |
    | nNNPDF10_nlo_as_0118_D2 | ✅|  |
    | nNNPDF10_nlo_as_0118_D2 | ✅|  |
    | nNNPDF10_nlo_as_0118_Fe56 | ✅|  |
    | nNNPDF10_nlo_as_0118_Fe56 | ✅|  |
    | nNNPDF10_nlo_as_0118_He4 | ✅|  |
    | nNNPDF10_nlo_as_0118_He4 | ✅|  |
    | nNNPDF10_nlo_as_0118_Li6 | ✅|  |
    | nNNPDF10_nlo_as_0118_Li6 | ✅|  |
    | nNNPDF10_nlo_as_0118_N1 | ✅|  |
    | nNNPDF10_nlo_as_0118_N1 | ✅|  |
    | nNNPDF10_nlo_as_0118_N14 | ✅|  |
    | nNNPDF10_nlo_as_0118_N14 | ✅|  |
    | nNNPDF10_nlo_as_0118_Pb208 | ✅|  |
    | nNNPDF10_nlo_as_0118_Pb208 | ✅|  |
    | nNNPDF10_nlo_as_0118_Sn119 | ✅|  |
    | nNNPDF10_nlo_as_0118_Sn119 | ✅|  |
    | nNNPDF10_nlo_as_0118_Xe131 | ✅|  |
    | nNNPDF10_nlo_as_0118_Xe131 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Ag108 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Ag108 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Al27 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Al27 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Au197 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Au197 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Be9 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Be9 | ✅|  |
    | nNNPDF10_nnlo_as_0118_C12 | ✅|  |
    | nNNPDF10_nnlo_as_0118_C12 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Ca40 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Ca40 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Cu64 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Cu64 | ✅|  |
    | nNNPDF10_nnlo_as_0118_D2 | ✅|  |
    | nNNPDF10_nnlo_as_0118_D2 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Fe56 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Fe56 | ✅|  |
    | nNNPDF10_nnlo_as_0118_He4 | ✅|  |
    | nNNPDF10_nnlo_as_0118_He4 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Li6 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Li6 | ✅|  |
    | nNNPDF10_nnlo_as_0118_N1 | ✅|  |
    | nNNPDF10_nnlo_as_0118_N1 | ✅|  |
    | nNNPDF10_nnlo_as_0118_N14 | ✅|  |
    | nNNPDF10_nnlo_as_0118_N14 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Pb208 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Pb208 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Sn119 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Sn119 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Xe131 | ✅|  |
    | nNNPDF10_nnlo_as_0118_Xe131 | ✅|  |
    | nNNPDF20_nlo_as_0118_Ag108 | ✅|  |
    | nNNPDF20_nlo_as_0118_Ag108 | ✅|  |
    | nNNPDF20_nlo_as_0118_Al27 | ✅|  |
    | nNNPDF20_nlo_as_0118_Al27 | ✅|  |
    | nNNPDF20_nlo_as_0118_Au197 | ✅|  |
    | nNNPDF20_nlo_as_0118_Au197 | ✅|  |
    | nNNPDF20_nlo_as_0118_Be9 | ✅|  |
    | nNNPDF20_nlo_as_0118_Be9 | ✅|  |
    | nNNPDF20_nlo_as_0118_C12 | ✅|  |
    | nNNPDF20_nlo_as_0118_C12 | ✅|  |
    | nNNPDF20_nlo_as_0118_Ca40 | ✅|  |
    | nNNPDF20_nlo_as_0118_Ca40 | ✅|  |
    | nNNPDF20_nlo_as_0118_Cu64 | ✅|  |
    | nNNPDF20_nlo_as_0118_Cu64 | ✅|  |
    | nNNPDF20_nlo_as_0118_D2 | ✅|  |
    | nNNPDF20_nlo_as_0118_D2 | ✅|  |
    | nNNPDF20_nlo_as_0118_Fe56 | ✅|  |
    | nNNPDF20_nlo_as_0118_Fe56 | ✅|  |
    | nNNPDF20_nlo_as_0118_He4 | ✅|  |
    | nNNPDF20_nlo_as_0118_He4 | ✅|  |
    | nNNPDF20_nlo_as_0118_Li6 | ✅|  |
    | nNNPDF20_nlo_as_0118_Li6 | ✅|  |
    | nNNPDF20_nlo_as_0118_N1 | ✅|  |
    | nNNPDF20_nlo_as_0118_N1 | ✅|  |
    | nNNPDF20_nlo_as_0118_N14 | ✅|  |
    | nNNPDF20_nlo_as_0118_N14 | ✅|  |
    | nNNPDF20_nlo_as_0118_O16 | ✅|  |
    | nNNPDF20_nlo_as_0118_O16 | ✅|  |
    | nNNPDF20_nlo_as_0118_Pb208 | ✅|  |
    | nNNPDF20_nlo_as_0118_Pb208 | ✅|  |
    | nNNPDF20_nlo_as_0118_Sn119 | ✅|  |
    | nNNPDF20_nlo_as_0118_Sn119 | ✅|  |
    | nNNPDF20_nlo_as_0118_W184 | ✅|  |
    | nNNPDF20_nlo_as_0118_W184 | ✅|  |
    | nNNPDF20_nlo_as_0118_Xe131 | ✅|  |
    | nNNPDF20_nlo_as_0118_Xe131 | ✅|  |
    | nNNPDF30_nlo_as_0118_A108_Z54 | ✅|  |
    | nNNPDF30_nlo_as_0118_A108_Z54 | ✅|  |
    | nNNPDF30_nlo_as_0118_A119_Z59 | ✅|  |
    | nNNPDF30_nlo_as_0118_A119_Z59 | ✅|  |
    | nNNPDF30_nlo_as_0118_A12_Z6 | ✅|  |
    | nNNPDF30_nlo_as_0118_A12_Z6 | ✅|  |
    | nNNPDF30_nlo_as_0118_A131_Z54 | ✅|  |
    | nNNPDF30_nlo_as_0118_A131_Z54 | ✅|  |
    | nNNPDF30_nlo_as_0118_A14_Z7 | ✅|  |
    | nNNPDF30_nlo_as_0118_A14_Z7 | ✅|  |
    | nNNPDF30_nlo_as_0118_A16_Z8 | ✅|  |
    | nNNPDF30_nlo_as_0118_A16_Z8 | ✅|  |
    | nNNPDF30_nlo_as_0118_A184_Z74 | ✅|  |
    | nNNPDF30_nlo_as_0118_A184_Z74 | ✅|  |
    | nNNPDF30_nlo_as_0118_A197_Z79 | ✅|  |
    | nNNPDF30_nlo_as_0118_A197_Z79 | ✅|  |
    | nNNPDF30_nlo_as_0118_A208_Z82 | ✅|  |
    | nNNPDF30_nlo_as_0118_A208_Z82 | ✅|  |
    | nNNPDF30_nlo_as_0118_A27_Z13 | ✅|  |
    | nNNPDF30_nlo_as_0118_A27_Z13 | ✅|  |
    | nNNPDF30_nlo_as_0118_A2_Z1 | ✅|  |
    | nNNPDF30_nlo_as_0118_A2_Z1 | ✅|  |
    | nNNPDF30_nlo_as_0118_A31_Z15 | ✅|  |
    | nNNPDF30_nlo_as_0118_A31_Z15 | ✅|  |
    | nNNPDF30_nlo_as_0118_A40_Z20 | ✅|  |
    | nNNPDF30_nlo_as_0118_A40_Z20 | ✅|  |
    | nNNPDF30_nlo_as_0118_A4_Z2 | ✅|  |
    | nNNPDF30_nlo_as_0118_A4_Z2 | ✅|  |
    | nNNPDF30_nlo_as_0118_A56_Z26 | ✅|  |
    | nNNPDF30_nlo_as_0118_A56_Z26 | ✅|  |
    | nNNPDF30_nlo_as_0118_A64_Z29 | ✅|  |
    | nNNPDF30_nlo_as_0118_A64_Z29 | ✅|  |
    | nNNPDF30_nlo_as_0118_A6_Z3 | ✅|  |
    | nNNPDF30_nlo_as_0118_A6_Z3 | ✅|  |
    | nNNPDF30_nlo_as_0118_A9_Z4 | ✅|  |
    | nNNPDF30_nlo_as_0118_A9_Z4 | ✅|  |
    | nNNPDF30_nlo_as_0118_p | ✅|  |
    | nNNPDF30_nlo_as_0118_p | ✅|  |

=== "$\alpha_s (Q^2)$ comparison"

    To be added soon!
