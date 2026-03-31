//! Shared ANSI table formatter for terminal output.
//!
//! Usage:
//!   let mut t = Table::new(&["Way", "Score", "Description"]);
//!   t.align(1, Align::Right);
//!   t.add(vec!["softwaredev/code/testing", "8.96", "unit testing..."]);
//!   t.print();

/// Column alignment.
#[derive(Clone, Copy)]
pub enum Align {
    Left,
    Right,
}

pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    aligns: Vec<Align>,
    max_widths: Vec<usize>,
    cap_widths: Vec<Option<usize>>,
    indent: usize,
}

impl Table {
    pub fn new(headers: &[&str]) -> Self {
        let n = headers.len();
        let headers: Vec<String> = headers.iter().map(|s| s.to_string()).collect();
        let max_widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
        Table {
            headers,
            rows: Vec::new(),
            aligns: vec![Align::Left; n],
            max_widths,
            cap_widths: vec![None; n],
            indent: 2,
        }
    }

    /// Set alignment for a column.
    pub fn align(&mut self, col: usize, a: Align) {
        if col < self.aligns.len() {
            self.aligns[col] = a;
        }
    }

    /// Set left indent (spaces before each row).
    pub fn indent(&mut self, n: usize) {
        self.indent = n;
    }

    /// Set a maximum width cap for a column (truncates with ellipsis).
    pub fn max_width(&mut self, col: usize, width: usize) {
        if col < self.cap_widths.len() {
            self.cap_widths[col] = Some(width);
        }
    }

    /// Add a row. Accepts anything that can be stringified.
    pub fn add(&mut self, cells: Vec<&str>) {
        let row: Vec<String> = cells.iter().map(|s| s.to_string()).collect();
        // Update max widths (measuring visible length, not ANSI codes)
        for (i, cell) in row.iter().enumerate() {
            if i < self.max_widths.len() {
                let visible = visible_len(cell);
                if visible > self.max_widths[i] {
                    self.max_widths[i] = visible;
                }
            }
        }
        self.rows.push(row);
    }

    /// Add a row from owned strings.
    pub fn add_owned(&mut self, cells: Vec<String>) {
        for (i, cell) in cells.iter().enumerate() {
            if i < self.max_widths.len() {
                let visible = visible_len(cell);
                if visible > self.max_widths[i] {
                    self.max_widths[i] = visible;
                }
            }
        }
        self.rows.push(cells);
    }

    /// Print the table to stdout.
    pub fn print(&self) {
        let pad = " ".repeat(self.indent);
        let ncols = self.headers.len();

        // Apply caps
        let widths: Vec<usize> = self.max_widths.iter().enumerate().map(|(i, w)| {
            match self.cap_widths.get(i).and_then(|c| *c) {
                Some(cap) => (*w).min(cap),
                None => *w,
            }
        }).collect();

        // Header
        let mut header = String::new();
        for (i, h) in self.headers.iter().enumerate() {
            if i > 0 {
                header.push(' ');
            }
            let w = widths.get(i).copied().unwrap_or(10);
            header.push_str(&pad_cell(h, w, self.aligns.get(i).copied().unwrap_or(Align::Left)));
        }
        println!("{pad}\x1b[1m{header}\x1b[0m");

        // Separator
        let total_width: usize = widths.iter().sum::<usize>() + ncols.saturating_sub(1);
        println!("{pad}\x1b[2m{}\x1b[0m", "─".repeat(total_width));

        // Rows
        for row in &self.rows {
            let mut line = String::new();
            for (i, cell) in row.iter().enumerate() {
                if i > 0 {
                    line.push(' ');
                }
                let w = widths.get(i).copied().unwrap_or(10);
                let align = self.aligns.get(i).copied().unwrap_or(Align::Left);
                let truncated = truncate_visible(cell, w);
                line.push_str(&pad_cell(&truncated, w, align));
            }
            println!("{pad}{line}");
        }
    }

    /// Return the number of rows.
    pub fn len(&self) -> usize {
        self.rows.len()
    }
}

/// Measure the visible length of a string (ignoring ANSI escape codes).
fn visible_len(s: &str) -> usize {
    let mut len = 0;
    let mut in_escape = false;
    for c in s.chars() {
        if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else if c == '\x1b' {
            in_escape = true;
        } else {
            len += 1;
        }
    }
    len
}

/// Truncate a string to a visible width, adding ellipsis if needed.
/// Preserves ANSI codes (doesn't count them toward width).
fn truncate_visible(s: &str, max: usize) -> String {
    let vlen = visible_len(s);
    if vlen <= max {
        return s.to_string();
    }
    if max <= 1 {
        return "…".to_string();
    }

    let target = max - 1; // room for ellipsis
    let mut result = String::new();
    let mut visible = 0;
    let mut in_escape = false;

    for c in s.chars() {
        if in_escape {
            result.push(c);
            if c == 'm' {
                in_escape = false;
            }
        } else if c == '\x1b' {
            in_escape = true;
            result.push(c);
        } else {
            if visible >= target {
                break;
            }
            result.push(c);
            visible += 1;
        }
    }
    result.push('…');
    // Close any open ANSI sequence
    result.push_str("\x1b[0m");
    result
}

/// Pad a cell to a width with the given alignment.
fn pad_cell(s: &str, width: usize, align: Align) -> String {
    let vlen = visible_len(s);
    if vlen >= width {
        return s.to_string();
    }
    let padding = width - vlen;
    match align {
        Align::Left => format!("{s}{}", " ".repeat(padding)),
        Align::Right => format!("{}{s}", " ".repeat(padding)),
    }
}
