#!/usr/bin/env python3
"""
Generate figures/q-trajectory.pdf — Figure 2 of the paper.

Computes q(asset, t) for the four worked-example assets across
the 2026 → 2030 deadline window, holding observables fixed.

Run from the paper directory:
    python3 scripts/q-trajectory.py

Output: figures/q-trajectory.pdf

Dependencies: matplotlib, numpy.
"""

from datetime import date, timedelta
import matplotlib.pyplot as plt
import matplotlib.dates as mdates

D = date(2030, 1, 1)
H = 5.0  # horizon in years
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
    ("α  modern, agile, no QKD",     0.51, 0.00, 0.75, "tab:blue"),
    ("β  modern, FIPS-locked",       0.51, 0.00, 0.20, "tab:orange"),
    ("γ  legacy, pinned",            0.12, 0.00, 0.50, "tab:red"),
    ("δ  modern + QKD-hybrid PSK",   0.51, 0.70, 0.75, "tab:green"),
]


def main():
    start = date(2026, 1, 1)
    end = date(2030, 1, 1)
    days = (end - start).days
    dates = [start + timedelta(days=d) for d in range(0, days + 1, 14)]

    fig, ax = plt.subplots(figsize=(7, 4))

    for label, A, C, G, color in ASSETS:
        ys = [q(A, C, G, t) for t in dates]
        lw = 3.0 if label.startswith("β") else 2.0
        ls = "--" if label.startswith("β") else "-"
        ax.plot(dates, ys, label=label, color=color, linewidth=lw, linestyle=ls)

    ax.axhline(0.60, linestyle=":", color="gray", linewidth=1)
    ax.text(date(2026, 2, 1), 0.605, "must migrate", color="gray", fontsize=9, va="bottom")
    ax.axhline(0.30, linestyle=":", color="gray", linewidth=1)
    ax.text(date(2026, 2, 1), 0.305, "plan migration", color="gray", fontsize=9, va="bottom")

    ax.axvline(date(2029, 7, 1), linestyle=":", color="black", linewidth=0.8)
    ax.annotate("t₂ (worked example)",
                xy=(date(2029, 7, 1), 0.20),
                xytext=(date(2028, 9, 1), 0.10),
                fontsize=8, ha="center",
                arrowprops=dict(arrowstyle="-", color="gray", lw=0.6))

    ax.text(date(2029, 12, 28), 0.05, "D = 2030-01-01\n(deadline)",
            fontsize=9, ha="right", color="black", fontweight="bold")

    ax.set_xlim(start, end)
    ax.set_ylim(0, 1)
    ax.set_ylabel("q(asset, t)")
    ax.set_xlabel("date")
    ax.xaxis.set_major_locator(mdates.YearLocator())
    ax.xaxis.set_major_formatter(mdates.DateFormatter("%Y"))
    ax.grid(True, alpha=0.3)

    ax.legend(loc="upper left", framealpha=0.95, fontsize=9,
              title="Asset (β shown dashed: BLOCKED flag raised)",
              title_fontsize=9)

    plt.tight_layout()
    plt.savefig("figures/q-trajectory.pdf", bbox_inches="tight")
    plt.savefig("figures/q-trajectory.png", dpi=200, bbox_inches="tight")
    print("Wrote figures/q-trajectory.pdf and .png")


if __name__ == "__main__":
    main()
