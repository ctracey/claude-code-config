#!/usr/bin/env python3
"""Render task status as ANSI progress dashboard.

Reads task data as arguments or stdin in a simple format:
    task-dashboard.py "1:in_progress:Build renderer" "2:pending:Create hook" "3:completed:Research"

Or pipe in:
    echo '1:completed:Build renderer
    2:in_progress:Create hook:40
    3:pending:Research' | task-dashboard.py

Format per line/arg: id:status:subject[:percent]
  - status: pending, in_progress, completed
  - percent: optional 0-100 override (otherwise derived from status)

Blocked-by can be indicated with: id:status:subject:percent:blocked_by_ids
  e.g., "3:pending:Deploy:0:1,2"
"""

import sys

# ── Colors ──────────────────────────────────────────────────

RST = '\033[0m'
BOLD = '\033[1m'
DIM = '\033[2m'

def fg(r, g, b):
    return f'\033[38;2;{r};{g};{b}m'

def bg(r, g, b):
    return f'\033[48;2;{r};{g};{b}m'

# Status colors
GREEN = fg(126, 211, 33)
YELLOW = fg(253, 203, 110)
RED = fg(255, 118, 117)
BLUE = fg(99, 179, 237)
GRAY = fg(120, 120, 120)
WHITE = fg(220, 220, 220)

BG_GREEN = fg(126, 211, 33)
BG_YELLOW = fg(253, 203, 110)
BG_GRAY = fg(70, 70, 70)

STATUS_CONFIG = {
    'completed':   {'icon': '✓', 'color': GREEN,  'bar_fg': BG_GREEN,  'pct': 100},
    'in_progress': {'icon': '◉', 'color': YELLOW, 'bar_fg': BG_YELLOW, 'pct': 50},
    'pending':     {'icon': '○', 'color': GRAY,   'bar_fg': BG_GRAY,   'pct': 0},
}


# ── Parsing ─────────────────────────────────────────────────

def parse_task(s):
    parts = s.strip().split(':')
    if len(parts) < 3:
        return None
    task = {
        'id': parts[0].strip(),
        'status': parts[1].strip(),
        'subject': parts[2].strip(),
        'pct': None,
        'blocked_by': [],
    }
    if len(parts) >= 4 and parts[3].strip():
        try:
            task['pct'] = int(parts[3].strip())
        except ValueError:
            pass
    if len(parts) >= 5 and parts[4].strip():
        task['blocked_by'] = [b.strip() for b in parts[4].split(',') if b.strip()]

    # Default percentage from status if not overridden
    if task['pct'] is None:
        task['pct'] = STATUS_CONFIG.get(task['status'], STATUS_CONFIG['pending'])['pct']

    return task


def parse_input():
    tasks = []
    if len(sys.argv) > 1:
        for arg in sys.argv[1:]:
            t = parse_task(arg)
            if t:
                tasks.append(t)
    else:
        for line in sys.stdin:
            if line.strip():
                t = parse_task(line)
                if t:
                    tasks.append(t)
    return tasks


# ── Rendering ───────────────────────────────────────────────

def render_bar(pct, width=30):
    filled = int((pct / 100) * width)
    empty = width - filled

    if pct >= 100:
        bar_color = BG_GREEN
    elif pct > 0:
        bar_color = BG_YELLOW
    else:
        bar_color = BG_GRAY

    bar = bar_color + '█' * filled + RST + GRAY + '░' * empty + RST
    return bar


def render_dashboard(tasks):
    if not tasks:
        print(f"  {DIM}No tasks.{RST}")
        return

    # Group into waves by dependency order
    waves = group_waves(tasks)

    # Overall stats
    total = len(tasks)
    completed = sum(1 for t in tasks if t['status'] == 'completed')
    in_progress = sum(1 for t in tasks if t['status'] == 'in_progress')
    pending = total - completed - in_progress
    overall_pct = sum(t['pct'] for t in tasks) / total if total else 0

    # Header
    print()
    print(f"  {BOLD}Task Dashboard{RST}  {DIM}({completed}/{total} complete){RST}")
    print(f"  {render_bar(overall_pct, 40)}  {WHITE}{overall_pct:.0f}%{RST}")
    print()

    # Render each wave
    for wave_num, wave_tasks in enumerate(waves, 1):
        wave_ids = ', '.join(f'#{t["id"]}' for t in wave_tasks)
        wave_pct = sum(t['pct'] for t in wave_tasks) / len(wave_tasks)
        wave_completed = all(t['status'] == 'completed' for t in wave_tasks)
        wave_pending = all(t['status'] == 'pending' for t in wave_tasks)

        if wave_completed:
            wave_status = f'{GREEN}✓ Complete{RST}'
        elif wave_pending:
            wave_status = f'{GRAY}⏸ Queued{RST}'
        else:
            wave_status = f'{YELLOW}◉ Running{RST}'

        print(f"  {BOLD}Wave {wave_num}{RST}  {DIM}{wave_ids}{RST}  {wave_status}")

        for t in wave_tasks:
            cfg = STATUS_CONFIG.get(t['status'], STATUS_CONFIG['pending'])
            icon = cfg['color'] + cfg['icon'] + RST
            pct_str = f"{t['pct']:>3}%"
            bar = render_bar(t['pct'], 20)
            subject = t['subject'][:35]
            print(f"    {icon} {WHITE}#{t['id']}{RST} {subject:<35} {bar} {pct_str}")

        print()


def group_waves(tasks):
    """Group tasks into dependency waves. Tasks with no blockers are wave 1, etc."""
    task_map = {t['id']: t for t in tasks}
    assigned = set()
    waves = []

    # Keep grouping until all tasks are assigned
    remaining = list(tasks)
    max_iterations = len(tasks) + 1
    iteration = 0

    while remaining and iteration < max_iterations:
        iteration += 1
        wave = []
        for t in remaining:
            blockers = [b for b in t['blocked_by'] if b not in assigned]
            if not blockers:
                wave.append(t)

        if not wave:
            # Remaining tasks have circular deps or unresolvable blockers
            wave = remaining[:]

        for t in wave:
            assigned.add(t['id'])
        remaining = [t for t in remaining if t['id'] not in assigned]
        waves.append(wave)

    return waves


# ── Main ────────────────────────────────────────────────────

if __name__ == '__main__':
    tasks = parse_input()
    render_dashboard(tasks)
