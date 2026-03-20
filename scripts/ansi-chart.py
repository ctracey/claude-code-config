#!/usr/bin/env python3
"""ANSI terminal chart library for Claude Code Bash tool output.

Usage:
    import ansi-chart as chart  # or exec() / subprocess

    # Bar chart
    python3 ansi-chart.py bar "Mon=12,Tue=28,Wed=45,Thu=38,Fri=52"

    # Line chart from values
    python3 ansi-chart.py line "10,25,40,55,70,60,45,30,15,20,35,50"

    # Stacked bar (proportional segments)
    python3 ansi-chart.py stack "System=22,Apps=35,Media=18,Docs=10,Free=15"

    # Sparkline (compact single-line)
    python3 ansi-chart.py spark "10,25,40,55,70,60,45,30,15,20"

All output uses 24-bit ANSI true color escape codes.
"""

import sys
import math

# ── Color palettes ──────────────────────────────────────────

PALETTE = [
    (99, 179, 237),   # blue
    (78, 205, 196),   # teal
    (126, 211, 33),   # green
    (255, 234, 167),  # yellow
    (253, 203, 110),  # orange
    (255, 118, 117),  # red/pink
    (162, 155, 254),  # purple
    (253, 121, 168),  # magenta
    (116, 185, 255),  # sky
    (85, 239, 196),   # mint
]

def fg(r, g, b):
    return f'\033[38;2;{r};{g};{b}m'

def bg(r, g, b):
    return f'\033[48;2;{r};{g};{b}m'

RST = '\033[0m'
BOLD = '\033[1m'
DIM = '\033[2m'


def gradient_color(pct):
    """Map 0.0-1.0 to a blue→green→red gradient."""
    r = int(255 * pct)
    g = int(255 * (1 - abs(pct - 0.5) * 2))
    b = int(255 * (1 - pct))
    return r, g, b


# ── Bar chart ───────────────────────────────────────────────

def bar_chart(data, title=None, width=40):
    max_val = max(v for _, v in data)
    if max_val == 0:
        max_val = 1
    max_label = max(len(k) for k, _ in data)

    lines = []
    if title:
        lines.append(f'  {BOLD}{title}{RST}')
        lines.append('')

    for i, (label, val) in enumerate(data):
        c = PALETTE[i % len(PALETTE)]
        filled = int((val / max_val) * width)
        bar = fg(*c) + '█' * filled + RST
        lines.append(f'  {BOLD}{label:>{max_label}}{RST} {bar} {val}')

    lines.append('')
    return '\n'.join(lines)


# ── Line chart ──────────────────────────────────────────────

def line_chart(values, title=None, height=12, width=None):
    if width and len(values) > width:
        # Downsample
        step = len(values) / width
        values = [values[int(i * step)] for i in range(width)]

    min_v = min(values)
    max_v = max(values)
    if max_v == min_v:
        max_v = min_v + 1
    span = max_v - min_v
    label_w = 5

    lines = []
    if title:
        lines.append(f'  {BOLD}{title}{RST}')
        lines.append('')

    for row in range(height, -1, -1):
        # Y-axis label
        fmt = lambda v: f'{int(v):>4}' if v == int(v) else f'{v:>4.0f}'
        if row == height:
            label = f'{DIM}{fmt(max_v)}{RST} '
        elif row == 0:
            label = f'{DIM}{fmt(min_v)}{RST} '
        elif row == height // 2:
            mid = (max_v + min_v) / 2
            label = f'{DIM}{fmt(mid)}{RST} '
        else:
            label = '     '

        threshold = min_v + span * (row / height)
        line = ''
        for col in range(len(values)):
            val = values[col]
            pct = (val - min_v) / span
            r, g, b = gradient_color(pct)
            if abs(val - threshold) < span / (height * 2):
                line += fg(r, g, b) + '●' + RST
            elif val > threshold:
                line += fg(r, g, b) + '│' + RST
            else:
                line += ' '
        lines.append(label + line)

    lines.append('     ' + DIM + '─' * len(values) + RST)
    lines.append('')
    return '\n'.join(lines)


# ── Stacked bar ─────────────────────────────────────────────

def stacked_bar(data, title=None, width=50):
    total = sum(v for _, v in data)
    if total == 0:
        total = 1

    lines = []
    if title:
        lines.append(f'  {BOLD}{title}{RST}')
        lines.append('')

    bar = '  '
    for i, (name, val) in enumerate(data):
        c = PALETTE[i % len(PALETTE)]
        chars = max(1, int((val / total) * width))
        bar += fg(*c) + '█' * chars + RST
    lines.append(bar)
    lines.append('')

    for i, (name, val) in enumerate(data):
        c = PALETTE[i % len(PALETTE)]
        pct = val * 100 / total
        lines.append(f'  {fg(*c)}██{RST} {name:<8} {pct:.0f}%')

    lines.append('')
    return '\n'.join(lines)


# ── Sparkline ───────────────────────────────────────────────

def sparkline(values, title=None):
    blocks = ' ▁▂▃▄▅▆▇█'
    min_v = min(values)
    max_v = max(values)
    span = max_v - min_v if max_v != min_v else 1

    line = ''
    for val in values:
        pct = (val - min_v) / span
        idx = int(pct * (len(blocks) - 1))
        r, g, b = gradient_color(pct)
        line += fg(r, g, b) + blocks[idx] + RST

    parts = []
    if title:
        parts.append(f'  {BOLD}{title}{RST} ')
    parts.append(f'  {line}  {DIM}{min_v}–{max_v}{RST}')
    return '\n'.join(parts)


# ── CLI ─────────────────────────────────────────────────────

def _num(s):
    """Parse string to int if possible, else float."""
    f = float(s.strip())
    return int(f) if f == int(f) else f

def parse_kv(s):
    """Parse 'A=1,B=2,C=3' into [(A,1),(B,2),(C,3)]."""
    pairs = []
    for item in s.split(','):
        k, v = item.split('=')
        pairs.append((k.strip(), _num(v)))
    return pairs

def parse_values(s):
    """Parse '1,2,3,4' into [1,2,3,4]."""
    return [_num(x) for x in s.split(',')]

def usage():
    print("Usage: ansi-chart.py <type> <data> [--title TITLE] [--width N] [--height N]")
    print()
    print("Types:")
    print("  bar    key=val pairs    Bar chart")
    print("  line   comma values     Line chart with gradient")
    print("  stack  key=val pairs    Stacked/proportional bar")
    print("  spark  comma values     Single-line sparkline")
    print()
    print("Example:")
    print('  ansi-chart.py bar "Mon=12,Tue=28,Wed=45" --title "Weekly"')
    sys.exit(1)

def main():
    if len(sys.argv) < 3:
        usage()

    chart_type = sys.argv[1]
    data_str = sys.argv[2]

    # Parse optional flags
    title = None
    width = None
    height = None
    args = sys.argv[3:]
    i = 0
    while i < len(args):
        if args[i] == '--title' and i + 1 < len(args):
            title = args[i + 1]
            i += 2
        elif args[i] == '--width' and i + 1 < len(args):
            width = int(args[i + 1])
            i += 2
        elif args[i] == '--height' and i + 1 < len(args):
            height = int(args[i + 1])
            i += 2
        else:
            i += 1

    if chart_type == 'bar':
        kw = {}
        if title: kw['title'] = title
        if width: kw['width'] = width
        print(bar_chart(parse_kv(data_str), **kw))

    elif chart_type == 'line':
        kw = {}
        if title: kw['title'] = title
        if width: kw['width'] = width
        if height: kw['height'] = height
        print(line_chart(parse_values(data_str), **kw))

    elif chart_type == 'stack':
        kw = {}
        if title: kw['title'] = title
        if width: kw['width'] = width
        print(stacked_bar(parse_kv(data_str), **kw))

    elif chart_type == 'spark':
        print(sparkline(parse_values(data_str), title=title))

    else:
        print(f"Unknown chart type: {chart_type}")
        usage()

if __name__ == '__main__':
    main()
