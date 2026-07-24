#!/usr/bin/env python3
"""
Generate figures/sezar-architecture.pdf — Figure 3 of the paper.

Architectural diagram showing the five agents emitting crypto_inventory_event
records into sezar-server, with the shared sezar-core rollup library and
the React dashboard.

Dependencies: matplotlib only.
"""

import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch, FancyArrowPatch
from matplotlib.patches import ConnectionPatch

# ----- Style -----
AGENT_FACE      = "#E8F0FE"
AGENT_EDGE      = "#1A73E8"
LIBRARY_FACE    = "#FEF7E0"
LIBRARY_EDGE    = "#F9AB00"
SERVER_FACE     = "#E6F4EA"
SERVER_EDGE     = "#188038"
STORAGE_FACE    = "#F1F3F4"
STORAGE_EDGE    = "#5F6368"
DASHBOARD_FACE  = "#FCE8E6"
DASHBOARD_EDGE  = "#D93025"
PHASE_NOTE      = "#5F6368"


def box(ax, xy, w, h, label, subtitle=None, face=AGENT_FACE,
        edge=AGENT_EDGE, lw=1.5, fontsize=10, subtitle_size=8):
    x, y = xy
    rect = FancyBboxPatch((x, y), w, h,
                          boxstyle="round,pad=0.02,rounding_size=0.10",
                          linewidth=lw, edgecolor=edge, facecolor=face)
    ax.add_patch(rect)
    if subtitle:
        ax.text(x + w / 2, y + h * 0.62, label, ha="center", va="center",
                fontsize=fontsize, fontweight="bold")
        ax.text(x + w / 2, y + h * 0.28, subtitle, ha="center", va="center",
                fontsize=subtitle_size, color="#3C4043", style="italic")
    else:
        ax.text(x + w / 2, y + h / 2, label, ha="center", va="center",
                fontsize=fontsize, fontweight="bold")
    return (x, y, w, h)


def arrow(ax, src, dst, color="#5F6368", lw=1.2, style="-|>",
          connectionstyle="arc3,rad=0.0"):
    a = FancyArrowPatch(src, dst, arrowstyle=style,
                        mutation_scale=12, color=color, lw=lw,
                        connectionstyle=connectionstyle)
    ax.add_patch(a)


def main():
    fig, ax = plt.subplots(figsize=(12, 8))
    ax.set_xlim(0, 16)
    ax.set_ylim(0, 11)
    ax.axis("off")

    # ---------- Agents row (top) ----------
    agents = [
        ("sezar-net",    "eBPF: TLS / SSH / IPsec",     1.00, "V1"),
        ("sezar-qkd",    "ETSI GS QKD 014 REST",        3.50, "V1"),
        ("sezar-agility","Semgrep over source/pkg",     6.00, "V1"),
        ("sezar-cert",   "CT logs + host scan",         8.50, "V2"),
        ("sezar-chain",  "Public-chain RPC",            11.00, "V3"),
        ("sezar-id",     "HSM/KMS/Smart-card",          13.50, "V4"),
    ]

    agent_y = 8.5
    agent_h = 1.4
    agent_w = 2.0
    agent_positions = []

    for name, source, x, phase in agents:
        box(ax, (x, agent_y), agent_w, agent_h,
            f"{name}", subtitle=source,
            face=AGENT_FACE, edge=AGENT_EDGE, lw=1.5)
        # phase tag on top-right corner
        ax.text(x + agent_w - 0.05, agent_y + agent_h - 0.05, phase,
                ha="right", va="top", fontsize=7, color=PHASE_NOTE,
                fontweight="bold",
                bbox=dict(boxstyle="round,pad=0.15",
                          facecolor="white", edgecolor=PHASE_NOTE,
                          linewidth=0.5))
        agent_positions.append((x + agent_w / 2, agent_y))

    # ---------- Event bus ----------
    # A visible horizontal rail: every agent arrow terminates ON this line,
    # and the line itself feeds the collector below. Drawn with ax.plot so
    # it renders reliably (the earlier annotate-based line was invisible in
    # some backends, leaving the agent arrows dangling in whitespace).
    bus_y = 7.0
    bus_xs = [agent_positions[0][0] - 0.4, agent_positions[-1][0] + 0.4]
    ax.plot(bus_xs, [bus_y, bus_y], color="#3C4043", lw=2.5,
            solid_capstyle="round", zorder=1)
    # Label sits under the right half of the rail, clear of the vertical
    # drop to the collector and of the agent arrows above the rail.
    ax.text(12.3, bus_y - 0.30,
            "crypto_inventory_event v1.1   (JSON over HTTP)",
            ha="center", va="top", fontsize=10, fontweight="bold",
            color="#3C4043",
            bbox=dict(boxstyle="round,pad=0.3", facecolor="white",
                      edgecolor="#3C4043", linewidth=1))

    # connect agents to bus (vertical arrows, terminating on the rail)
    for ax_x, ay in agent_positions:
        arrow(ax, (ax_x, ay), (ax_x, bus_y + 0.02),
              color=AGENT_EDGE, lw=1.2)

    # ---------- Shared library (left side) ----------
    core_x, core_y = 0.3, 5.0
    core_w, core_h = 2.2, 1.4
    box(ax, (core_x, core_y), core_w, core_h,
        "sezar-core",
        subtitle="schema + rollup + classification table",
        face=LIBRARY_FACE, edge=LIBRARY_EDGE, lw=1.5)

    # show that every agent depends on sezar-core
    for ax_x, ay in agent_positions:
        ax.plot([core_x + core_w / 2, ax_x],
                [core_y + core_h, ay + agent_h / 2],
                color=LIBRARY_EDGE, lw=0.5, linestyle=":", alpha=0.4,
                zorder=0)
    ax.text(0.3 + core_w / 2, core_y - 0.30,
            "shared by every agent\n(local posture rollup)",
            ha="center", va="top", fontsize=8, color=LIBRARY_EDGE,
            style="italic")

    # ---------- Server ----------
    server_x, server_y = 5.5, 4.0
    server_w, server_h = 5.0, 1.6
    box(ax, (server_x, server_y), server_w, server_h,
        "sezar-server",
        subtitle="axum collector  •  REST API  •  schema validator",
        face=SERVER_FACE, edge=SERVER_EDGE, lw=1.8, fontsize=11)

    # bus → server: a single vertical drop from the rail into the collector,
    # aligned with the server's centre so the flow reads top-to-bottom.
    arrow(ax, (server_x + server_w / 2, bus_y - 0.02),
          (server_x + server_w / 2, server_y + server_h),
          color="#3C4043", lw=1.8)

    # ---------- Storage ----------
    db_y = 2.0
    box(ax, (server_x + 0.20, db_y), 2.10, 1.2,
        "Postgres", subtitle="config, weights,\nasset metadata",
        face=STORAGE_FACE, edge=STORAGE_EDGE, lw=1.2, fontsize=10,
        subtitle_size=8)
    box(ax, (server_x + 2.70, db_y), 2.10, 1.2,
        "Columnar", subtitle="event history,\ntime-series posture",
        face=STORAGE_FACE, edge=STORAGE_EDGE, lw=1.2, fontsize=10,
        subtitle_size=8)

    # server ↔ db
    arrow(ax, (server_x + 1.25, server_y), (server_x + 1.25, db_y + 1.2),
          color=STORAGE_EDGE, lw=1.0, style="<|-|>")
    arrow(ax, (server_x + 3.75, server_y), (server_x + 3.75, db_y + 1.2),
          color=STORAGE_EDGE, lw=1.0, style="<|-|>")

    # ---------- Dashboard ----------
    dash_x, dash_y = 11.5, 4.0
    dash_w, dash_h = 4.0, 1.6
    box(ax, (dash_x, dash_y), dash_w, dash_h,
        "React Dashboard",
        subtitle="three-axis matrix  •  priority queue  •  BLOCKED list",
        face=DASHBOARD_FACE, edge=DASHBOARD_EDGE, lw=1.5, fontsize=11)

    # server → dashboard
    arrow(ax, (server_x + server_w, server_y + server_h / 2),
          (dash_x, dash_y + dash_h / 2),
          color=DASHBOARD_EDGE, lw=1.2)
    ax.text((server_x + server_w + dash_x) / 2, server_y + server_h / 2 + 0.15,
            "REST", ha="center", va="bottom", fontsize=8,
            color=DASHBOARD_EDGE, style="italic")

    # ---------- Operator interaction (deadline D, weights) ----------
    op_y = 0.8
    ax.text(dash_x + dash_w / 2, op_y, "Operator",
            ha="center", va="center", fontsize=10, fontweight="bold")
    ax.text(dash_x + dash_w / 2, op_y - 0.40,
            "configures D (deadline), weights;\nconsumes priority + BLOCKED",
            ha="center", va="top", fontsize=8, color=PHASE_NOTE,
            style="italic")
    arrow(ax, (dash_x + dash_w / 2, op_y + 0.20),
          (dash_x + dash_w / 2, dash_y - 0.05),
          color=DASHBOARD_EDGE, lw=0.8, style="<|-|>")

    # ---------- External data sources (small labels under agents) ----------
    ext_labels = [
        ("kernel tracepoints",  agent_positions[0][0]),
        ("KME(s) over HTTPS",   agent_positions[1][0]),
        ("git / dpkg / rpm",    agent_positions[2][0]),
    ]
    for label, x in ext_labels:
        ax.text(x, agent_y + agent_h + 0.30, label,
                ha="center", va="bottom", fontsize=7.5,
                color="#5F6368", style="italic")
        ax.annotate("", xy=(x, agent_y + agent_h + 0.05),
                    xytext=(x, agent_y + agent_h + 0.28),
                    arrowprops=dict(arrowstyle="-|>", color="#5F6368", lw=0.7))

    # ---------- Title / version note ----------
    ax.text(8, 10.6, "Sezar — Reference Architecture (v1.1 event schema)",
            ha="center", va="top", fontsize=12, fontweight="bold",
            color="#202124")

    # ---------- Legend / phase note ----------
    ax.text(0.3, 0.5,
            "Phase tags (V1–V4) reflect the release plan.\n"
            "V1 ships sezar-net, sezar-qkd, sezar-agility, sezar-core, sezar-server.\n"
            "V2 adds sezar-cert; V3 adds sezar-chain; V4 adds sezar-id.\n"
            "All agents emit the same event shape into the same collector.",
            fontsize=8, color=PHASE_NOTE, style="italic", va="bottom")

    plt.tight_layout()
    plt.savefig("figures/sezar-architecture.pdf", bbox_inches="tight")
    plt.savefig("figures/sezar-architecture.png", dpi=200, bbox_inches="tight")
    print("Wrote figures/sezar-architecture.pdf and .png")


if __name__ == "__main__":
    main()
