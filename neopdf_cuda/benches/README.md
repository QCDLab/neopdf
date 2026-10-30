# Batch throughput benchmarks

| File | Role |
|---|---|
| `bench_batch.rs` | Criterion benchmark: `DeviceGrid` kernel vs a `rayon` CPU baseline over every member of `NNPDF40_nnlo_as_01180`, for 1k/10k/100k events |
| `plot_batch.py` | Turns criterion's output into a results JSON (`collect`) and plots it (`plot`) |
| `results/<gpu>-<cpu>.json` | Collected results of one run (mean times + 95% CI per benchmark) |
| `results/<gpu>-<cpu>.{png,pdf}` | Figure for that run |

Benchmarks in `bench_batch.rs`, per number of events:

- `cpu_rayon`: `PDF::xfxq2_allpids` + `alphas_q2` for every member, parallel over events.
- `gpu_device`: `DeviceGrid::xfxq2_batch_device` with inputs and output already on the GPU (the
  GPU-native event-generator case); this is what the figure compares against `cpu_rayon`.
- `gpu_host`: `DeviceGrid::xfxq2_batch_host`, including the host/device transfers.

Throughput is counted in `(event, member)` pairs, each producing every flavor plus `alpha_s`.

## Reproducing

Needs an sm_80+ GPU with a CUDA 13 (R580+) driver, `cargo-oxide`, and `NEOPDF_DATA_PATH`
pointing at a directory containing `NNPDF40_nnlo_as_01180`.

```sh
cd neopdf_cuda
cargo oxide test -- --features cuda -j1 --release --bench bench_batch -- --bench

cd benches
python plot_batch.py collect ../target/criterion results/rtx4090-epyc7k62.json \
    --gpu "NVIDIA GeForce RTX 4090" --cpu "AMD EPYC 7K62 (96 threads)"
python plot_batch.py plot results/rtx4090-epyc7k62.json
```

`plot_batch.py` needs only `matplotlib`.
