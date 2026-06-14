use comfy_table::{presets::UTF8_FULL, Cell, Table};

pub fn kv_table(title: &str, rows: &[(&str, String)]) -> String {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec![Cell::new(title), Cell::new("value")]);

    for (k, v) in rows {
        table.add_row(vec![Cell::new(*k), Cell::new(v)]);
    }

    table.to_string()
}
