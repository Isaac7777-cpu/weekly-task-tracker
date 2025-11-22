use terminal_size::{Width, terminal_size};

fn compute_bar_width(message_len: usize) -> usize {
    let default_bar = 20;

    if let Some((Width(w), _)) = terminal_size() {
        let cols = w as usize;
        let available = cols.saturating_sub(message_len);
        available.clamp(10, 250)
    } else {
        default_bar
    }
}

pub fn render_progress_bar(current: f64, target: f64, message_len: usize) -> String {
    let width = compute_bar_width(message_len); // + 2 for the bracket

    if target <= 0.0 || width == 0 {
        return format!("[{}]", "-".repeat(width));
    }

    let ratio = (current / target).max(0.0).min(1.0);

    let filled = (ratio * width as f64).round() as usize;
    let empty = width - filled;

    format!("[{}{}]", "#".repeat(filled), "-".repeat(empty))
}


const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const GREEN: &str = "\x1b[32m";

pub fn color_for_pct(pct: f64) -> &'static str {
    if pct < 25.0 {
        RED
    } else if pct < 75.0 {
        YELLOW
    } else {
        GREEN
    }
}
