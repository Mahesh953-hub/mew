use comfy_table::{presets::UTF8_FULL, Cell, Table};

use crate::layout::TerminalLayout;

pub fn kv_table(title: &str, rows: &[(&str, String)]) -> String {
    let layout = TerminalLayout::detect();

    if layout.is_tiny() {
        let mut out = String::new();
        out.push_str(title);
        out.push('\n');

        for (k, v) in rows {
            out.push_str(&format!("  {:<10} {}\n", k, v));
        }

        return out.trim_end().to_string();
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec![Cell::new(title), Cell::new("value")]);

    for (k, v) in rows {
        table.add_row(vec![Cell::new(*k), Cell::new(v)]);
    }

    table.to_string()
}
