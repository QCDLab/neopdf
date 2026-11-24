#!/usr/bin/env python3
"""Collect and plot the `bench_batch` results (device-resident GPU vs `rayon` CPU).

    # criterion's raw output -> a compact, versionable results file
    python plot_batch.py collect ../target/criterion results/<name>.json \
        --gpu "NVIDIA GeForce RTX 4090" --cpu "AMD EPYC 7K62, 96 threads"

    # results file -> results/<name>.{png,pdf}
    python plot_batch.py plot results/<name>.json
"""

import argparse
import json
from pathlib import Path

import matplotlib.pyplot as plt
from matplotlib.ticker import FixedLocator, FuncFormatter, NullLocator

GROUP = "xfxq2_batch"
WORKLOAD = (
    "NNPDF40_nnlo_as_01180: 101 members, 11 flavors + alpha_s per (event, member)"
)
SERIES = {
    "gpu_device": ("GPU (device-resident)", "#2a78d6"),
    "cpu_rayon": ("CPU (rayon)", "#eb6834"),
}
INK, INK_2, MUTED, GRID, AXIS = (
    "#0b0b0b",
    "#52514e",
    "#898781",
    "#e1e0d9",
    "#c3c2b7",
)


def collect(criterion_dir, gpu, cpu):
    """Reads every `GROUP/<function>/<n_events>/new` criterion estimate."""
    benchmarks = {}
    for estimates in Path(criterion_dir).glob(f"{GROUP}/*/*/new/estimates.json"):
        meta = json.loads((estimates.parent / "benchmark.json").read_text())
        mean = json.loads(estimates.read_text())["mean"]
        ci = mean["confidence_interval"]
        benchmarks.setdefault(meta["function_id"], []).append(
            {
                "n_events": int(meta["value_str"]),
                "pairs": meta["throughput"]["Elements"],
                "time_ns": mean["point_estimate"],
                "time_ns_ci95": [ci["lower_bound"], ci["upper_bound"]],
            }
        )
    for runs in benchmarks.values():
        runs.sort(key=lambda run: run["n_events"])
    return {
        "gpu": gpu,
        "cpu": cpu,
        "workload": WORKLOAD,
        "benchmarks": benchmarks,
    }


def throughput(run):
    """Millions of (event, member) pairs per second."""
    return run["pairs"] / run["time_ns"] * 1e3


def style(ax, title):
    ax.set_title(title, loc="left", color=INK, fontsize=11, pad=10)
    ax.spines[["top", "right"]].set_visible(False)
    ax.spines[["left", "bottom"]].set_color(AXIS)
    ax.tick_params(colors=MUTED, labelcolor=INK_2, labelsize=9)
    ax.grid(axis="y", color=GRID, linewidth=0.8)
    ax.set_axisbelow(True)


def events_label(n):
    return f"{n // 1000}k" if n >= 1000 else str(n)


def plot(results, output):
    runs = results["benchmarks"]
    events = [run["n_events"] for run in runs["gpu_device"]]
    fig, (ax_rate, ax_speedup) = plt.subplots(
        1, 2, figsize=(10, 4.2), facecolor="#fcfcfb"
    )

    for key, (label, color) in SERIES.items():
        rates = [throughput(run) for run in runs[key]]
        ax_rate.plot(
            events,
            rates,
            color=color,
            linewidth=2,
            marker="o",
            markersize=7,
            markeredgecolor="#fcfcfb",
            markeredgewidth=2,
            label=label,
        )
        ax_rate.annotate(
            f"{rates[-1]:.0f}",
            (events[-1], rates[-1]),
            xytext=(8, 0),
            textcoords="offset points",
            va="center",
            color=INK_2,
            fontsize=9,
        )
    ax_rate.set_xscale("log")
    ax_rate.set_ylim(bottom=0)
    ax_rate.set_ylabel("M (event, member) pairs / s", color=INK_2, fontsize=9)
    ax_rate.legend(frameon=False, fontsize=9, labelcolor=INK_2, loc="center right")
    style(ax_rate, "Throughput")

    speedups = [
        cpu["time_ns"] / gpu["time_ns"]
        for gpu, cpu in zip(runs["gpu_device"], runs["cpu_rayon"])
    ]
    positions = range(len(events))
    ax_speedup.bar(positions, speedups, width=0.55, color=SERIES["gpu_device"][1])
    for x, speedup in zip(positions, speedups):
        ax_speedup.annotate(
            f"{speedup:.1f}×",
            (x, speedup),
            xytext=(0, 4),
            textcoords="offset points",
            ha="center",
            color=INK,
            fontsize=10,
        )
    ax_speedup.set_xticks(list(positions), [events_label(n) for n in events])
    ax_speedup.set_ylim(0, max(speedups) * 1.15)
    ax_speedup.set_ylabel("CPU time / GPU time", color=INK_2, fontsize=9)
    style(ax_speedup, "GPU speedup over CPU")

    for ax in (ax_rate, ax_speedup):
        ax.set_facecolor("#fcfcfb")
        ax.set_xlabel("events per batch", color=INK_2, fontsize=9)
    ax_rate.xaxis.set_major_locator(FixedLocator(events))
    ax_rate.xaxis.set_major_formatter(FuncFormatter(lambda n, _: events_label(int(n))))
    ax_rate.xaxis.set_minor_locator(NullLocator())

    fig.suptitle(
        "neopdf_cuda batch evaluation, data resident on the GPU",
        x=0.01,
        ha="left",
        color=INK,
        fontsize=13,
        fontweight="bold",
    )
    fig.text(
        0.01,
        0.905,
        f"{results['gpu']} vs {results['cpu']}  ·  {results['workload']}",
        color=INK_2,
        fontsize=9,
    )
    fig.tight_layout(rect=(0, 0, 1, 0.95))
    for suffix in ("png", "pdf"):
        fig.savefig(
            output.with_suffix(f".{suffix}"),
            dpi=200,
            facecolor=fig.get_facecolor(),
        )


def main():
    parser = argparse.ArgumentParser(
        description=__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    commands = parser.add_subparsers(dest="command", required=True)
    collect_cmd = commands.add_parser(
        "collect", help="criterion output -> results JSON"
    )
    collect_cmd.add_argument("criterion_dir", type=Path)
    collect_cmd.add_argument("results", type=Path)
    collect_cmd.add_argument("--gpu", required=True)
    collect_cmd.add_argument("--cpu", required=True)
    plot_cmd = commands.add_parser("plot", help="results JSON -> PNG and PDF figure")
    plot_cmd.add_argument("results", type=Path)
    plot_cmd.add_argument(
        "--output",
        type=Path,
        help="figure path without suffix (default: next to results)",
    )
    args = parser.parse_args()

    if args.command == "collect":
        args.results.parent.mkdir(parents=True, exist_ok=True)
        args.results.write_text(
            json.dumps(collect(args.criterion_dir, args.gpu, args.cpu), indent=2) + "\n"
        )
    else:
        plot(
            json.loads(args.results.read_text()),
            args.output or args.results.with_suffix(""),
        )


if __name__ == "__main__":
    main()
