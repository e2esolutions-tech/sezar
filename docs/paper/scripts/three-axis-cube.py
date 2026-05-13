#!/usr/bin/env python3
"""
Generate figures/three-axis-cube.pdf — Figure 1 of the paper.

A 3D scatter of the four worked-example assets in (A, C, G) space.
q-value is encoded by marker colour at the t1 (2026-05-13) snapshot.

Dependencies: matplotlib, numpy.
"""

from datetime import date
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D  # noqa: F401 (registers projection)
from matplotlib import cm
from matplotlib.colors import Normalize

D = date(2030, 1, 1)
H = 5.0
ALPHA, BETA, GAMMA_MAX = 0.5, 0.2, 0.3


def tau(t: date) -> float:
    dt_years = (D - t).days / 365.25
    return max(0.0, min(1.0, 1.0 - dt_years / H))


def q(A: float, C: float, G: float, t: date) -> float:
    tv = tau(t)
    g = GAMMA_MAX * (1.0 - tv)
    s = ALPHA + BETA + g
    a, b, gn = ALPHA / s, BETA / s, g / s
    return 1.0 - (a * A + b * C + gn * G)


ASSETS = [
    ("α", 0.51, 0.00, 0.75),
    ("β", 0.51, 0.00, 0.20),
    ("γ", 0.12, 0.00, 0.50),
    ("δ", 0.51, 0.70, 0.75),
]


def main():
    t_now = date(2026, 5, 13)
    qs = [q(A, C, G, t_now) for _, A, C, G in ASSETS]

    fig = plt.figure(figsize=(9, 7))
    ax = fig.add_subplot(111, projection="3d")

    # Cube edges
    for s, e in [
        ((0, 0, 0), (1, 0, 0)), ((0, 0, 0), (0, 1, 0)), ((0, 0, 0), (0, 0, 1)),
        ((1, 0, 0), (1, 1, 0)), ((1, 0, 0), (1, 0, 1)),
        ((0, 1, 0), (1, 1, 0)), ((0, 1, 0), (0, 1, 1)),
        ((0, 0, 1), (1, 0, 1)), ((0, 0, 1), (0, 1, 1)),
        ((1, 1, 0), (1, 1, 1)), ((1, 0, 1), (1, 1, 1)), ((0, 1, 1), (1, 1, 1)),
    ]:
        ax.plot3D(*zip(s, e), color="gray", linewidth=0.4, alpha=0.5)

    # BLOCKED region: G ≤ 0.20 (any A, any C). Render as the
    # translucent upper boundary of the slab at G=0.20.
    import numpy as np
    A_grid, C_grid = np.meshgrid(np.linspace(0, 1, 8), np.linspace(0, 1, 8))
    G_grid = np.full_like(A_grid, 0.20)
    ax.plot_surface(A_grid, C_grid, G_grid, color="red", alpha=0.08,
                    edgecolor="red", linewidth=0.2)

    # Asset scatter
    norm = Normalize(vmin=0.3, vmax=0.9)
    cmap = plt.get_cmap("RdYlGn_r")
    for (lbl, A, C, G), qv in zip(ASSETS, qs):
        col = cmap(norm(qv))
        ax.scatter(A, C, G, color=col, s=200, edgecolor="black", linewidth=1.0,
                   depthshade=False)
        ax.text(A + 0.03, C + 0.03, G + 0.03, f"{lbl}  (q={qv:.2f})",
                fontsize=10, fontweight="bold")

    # Annotate BLOCKED region with text (bottom slab G ≤ 0.20)
    ax.text(0.85, 0.85, 0.10, "BLOCKED slab\n(G ≤ 0.20)",
            color="darkred", fontsize=9, fontweight="bold",
            ha="center")

    ax.set_xlabel("A — algorithmic resistance", labelpad=8)
    ax.set_ylabel("C — channel protection", labelpad=8)
    ax.set_zlabel("G — migration agility", labelpad=8)
    ax.set_xlim(0, 1)
    ax.set_ylim(0, 1)
    ax.set_zlim(0, 1)
    ax.view_init(elev=18, azim=-58)

    # Colour bar for q
    mappable = cm.ScalarMappable(norm=norm, cmap=cmap)
    mappable.set_array([])
    cbar = fig.colorbar(mappable, ax=ax, shrink=0.6, pad=0.10)
    cbar.set_label("q(asset, t = 2026-05-13)", fontsize=9)

    plt.tight_layout()
    plt.savefig("figures/three-axis-cube.pdf", bbox_inches="tight")
    plt.savefig("figures/three-axis-cube.png", dpi=200, bbox_inches="tight")
    print("Wrote figures/three-axis-cube.pdf and .png")


if __name__ == "__main__":
    main()
