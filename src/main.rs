use comfy_table::{Table, CellAlignment, Cell, Attribute, Color};
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_SOLID_INNER_BORDERS;

mod util;

fn main() {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_width(40)
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS)
        .set_header(vec![
                Cell::new(format!("{}@", util::user()))
                    .add_attribute(Attribute::Bold),
                Cell::new(util::host())
        ])
        .add_row(vec![
                 Cell::new("██ os")
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Red),
                 Cell::new(util::os())
        ])
        .add_row(vec![
                 Cell::new("██ krnl")
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Green),
                 Cell::new(util::kernel())
        ])
        .add_row(vec![
                 Cell::new("██ dist")
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Yellow),
                 Cell::new(util::distro())
        ])
        .add_row(vec![
                 Cell::new("██ pkgs")
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Blue),
                 Cell::new(util::pkgs())
        ])
        .add_row(vec![
                 Cell::new("██ mem")
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Magenta),
                 Cell::new(util::mem())
        ])
        .add_row(vec![
                 Cell::new("██ time")
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Cyan),
                 Cell::new(util::uptime())
        ]);
    let column = table.column_mut(1).expect("Table has two columns");
    column.set_cell_alignment(CellAlignment::Right);

    println!("{table}");
}
