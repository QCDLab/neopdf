"""Matplotlib plotting backend for NeoPDF GUI.

This module is embedded via include_str!() and executed through a Python
subprocess. It receives pre-computed PlotData from Rust and renders plots
using matplotlib. The resulting figures are returned either as base64
encoded PNG images (for the GUI) or written directly to disk.
"""

import base64
import io

import matplotlib
import matplotlib.pyplot as plt
from matplotlib import cycler

matplotlib.use("Agg")

# Global style configuration to match the requested matplotlib styling
plt.rcParams.update(
    {
        # Axes
        "axes.grid": True,
        "axes.titlesize": "medium",
        "axes.prop_cycle": cycler(
            color=[
                "#66c2a5",
                "#fc8d62",
                "#8da0cb",
                "#e78ac3",
                "#a6d854",
                "#ffd92f",
                "#e5c494",
                "#b3b3b3",
                "#c37892",
                "#789895",
            ]
        ),
        "axes.labelsize": "small",
        "axes.formatter.limits": (-5, 5),
        "axes.formatter.use_mathtext": True,
        "axes.formatter.useoffset": False,
        "axes.spines.top": False,
        "axes.spines.right": False,
        # Errorbar
        "errorbar.capsize": 2,
        # Fonts / text
        "font.size": 14,
        "text.usetex": False,
        "mathtext.default": "regular",
        # Figure
        "figure.figsize": (7, 4.3),
        # Grid
        "grid.color": "#cccccc",
        "grid.linestyle": "-",
        "grid.linewidth": 0.05,
        # Legend
        "legend.fontsize": "xx-small",
        "legend.numpoints": 1,
        "legend.scatterpoints": 1,
        "legend.loc": "best",
        "legend.fancybox": True,
        "legend.framealpha": 0.8,
        # Lines
        "lines.markersize": 4,
        # Ticks
        "xtick.labelsize": "small",
        "ytick.labelsize": "small",
        "xtick.top": False,
        "ytick.right": False,
        # SVG
        "svg.fonttype": "none",
    }
)

# Explicit color list mirroring the axes.prop_cycle.
COLORS = [
    "#66c2a5",
    "#fc8d62",
    "#8da0cb",
    "#e78ac3",
    "#a6d854",
    "#ffd92f",
    "#e5c494",
    "#b3b3b3",
    "#c37892",
    "#789895",
]


def _build_figure(data, x_label, y_label, x_log, y_log, title):
    """Create a matplotlib figure from plot data.

    Parameters
    ----------
    data : list of dict
        Each dict has keys: set_name, x_values, mean_values, upper_band, lower_band.
    x_label : str
    y_label : str
    x_log : bool
    y_log : bool
    title : str

    Returns
    -------
    fig : matplotlib.figure.Figure
    """
    fig, ax = plt.subplots(figsize=(7, 4.3))

    for i, d in enumerate(data):
        color = COLORS[i % len(COLORS)]
        xs = d["x_values"]
        mean = d["mean_values"]
        upper = d["upper_band"]
        lower = d["lower_band"]
        name = d["set_name"]

        ax.plot(xs, mean, color=color, linewidth=1.5, label=name)
        ax.fill_between(xs, lower, upper, color=color, alpha=0.15)

    if x_log:
        ax.set_xscale("log")
    if y_log:
        ax.set_yscale("log")
    if title:
        ax.set_title(title, fontsize=14)

    ax.set_xlabel(x_label, fontsize=12)
    ax.set_ylabel(y_label, fontsize=12)
    ax.set_xlim(left=xs[0], right=xs[-1])
    ax.legend(fontsize=10)
    fig.tight_layout()

    return fig


def render_plot(data, x_label, y_label, x_log, y_log, title):
    """Return the plot as a base64-encoded PNG string."""
    fig = _build_figure(data, x_label, y_label, x_log, y_log, title)

    buf = io.BytesIO()
    fig.savefig(buf, format="png", dpi=350, bbox_inches="tight")
    plt.close(fig)
    buf.seek(0)

    return base64.b64encode(buf.getvalue()).decode("ascii")


def save_plot(data, x_label, y_label, x_log, y_log, title, output_path):
    """Save the plot to a file (PNG, PDF, SVG, etc.)."""
    fig = _build_figure(data, x_label, y_label, x_log, y_log, title)
    fig.savefig(output_path, dpi=350, bbox_inches="tight")
    plt.close(fig)
