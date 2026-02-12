"""Matplotlib/mpld3 plotting backend for NeoPDF GUI.

This module is embedded via include_str!() and executed through a Python
subprocess.  It receives pre-computed PlotData from Rust and renders
interactive plots using matplotlib + mpld3.
"""

import json

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import mpld3


# Color cycle matching the original Qt GUI
COLORS = [
    "#1f77b4",  # blue
    "#d62728",  # red
    "#2ca02c",  # green
    "#17becf",  # cyan
    "#9467bd",  # purple
    "#ff7f0e",  # orange
    "#7f7f7f",  # gray
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
    fig, ax = plt.subplots(figsize=(9, 6))

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

    ax.set_xlabel(x_label, fontsize=12)
    ax.set_ylabel(y_label, fontsize=12)
    if title:
        ax.set_title(title, fontsize=14)
    ax.legend(fontsize=10)
    ax.grid(True, alpha=0.3)
    fig.tight_layout()

    return fig


def render_plot(data, x_label, y_label, x_log, y_log, title):
    """Return the mpld3 figure specification as a JSON string.

    The frontend will call ``mpld3.draw_figure()`` directly using this
    spec, so we only need the dict representation (not a full HTML page).
    """
    fig = _build_figure(data, x_label, y_label, x_log, y_log, title)
    spec = mpld3.fig_to_dict(fig)
    plt.close(fig)
    return json.dumps(spec)


def save_plot(data, x_label, y_label, x_log, y_log, title, output_path):
    """Save the plot to a file (PNG, PDF, SVG, etc.)."""
    fig = _build_figure(data, x_label, y_label, x_log, y_log, title)
    fig.savefig(output_path, dpi=150, bbox_inches="tight")
    plt.close(fig)
