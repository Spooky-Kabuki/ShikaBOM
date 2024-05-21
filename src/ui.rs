// ANCHOR: all
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::*,
    Frame,
};

use ratatui::style::{Modifier, Stylize};

use ratatui::style::palette::tailwind;
use unicode_width::UnicodeWidthStr;

use crate::parts::Part;


use crate::app::{App, CurrentScreen};

// ANCHOR: method_sig
pub fn ui(f: &mut Frame, app: &App) {
    let mut table_state = TableState::default();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(f.size());

    let a = Block::default()
        .title("ShikaBOM")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));
    f.render_widget(a, chunks[0]);
    // let b = Block::default()
    //     .borders(Borders::ALL);
    // f.render_widget(b, chunks[1]);
    //let rows = [Row::new(vec!["Cell1", "Cell2", "Cell3"])];
    //TODO: We don't want this running so often.
    let rows = create_table_rows();
// Columns widths are constrained in the same way as Layout...
    let widths = [
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(20),
    ];
    let header_style = Style::default()
        .fg(tailwind::SLATE.c200)
        .bg(tailwind::BLUE.c900);
    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(tailwind::BLUE.c400);
    let table = Table::new(rows, widths)
        .column_spacing(1)
        .style(Style::new().blue())
        .header(
            //TODO: Bring this in from parts.rs
            Row::new(vec!["Part Number", "Manufacturer", "Package", "Label", "Value", "Tolerance"])
                .style(header_style)
                // To add space between the header and the rest of the rows, specify the margin
                .bottom_margin(1),
        )
        // It has an optional footer, which is simply a Row always visible at the bottom.
        .footer(Row::new(vec!["Refreshed last at: 2021-09-01 12:34:56"]))
        // As any other widget, a Table can be wrapped in a Block.
        .block(Block::default().title("Table"))
        // The selected row and its content can also be styled.
        .highlight_style(selected_style)
        // ...and potentially show a symbol in front of the selection.
        .highlight_symbol(">>");
        //.border_style(Style::new().fg(Color::Cyan))
        //.borders(Borders::ALL);
    f.render_stateful_widget(table, chunks[1], &mut table_state);
    let c = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));
    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Parts => Span::styled(
                "(q) to quit / (n) to make new part",
                Style::default().fg(Color::Red),
            ),
        }
    };
    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .block(c);
    f.render_widget(key_notes_footer, chunks[2]);

    let popup_block = Block::default()
        .title("Enter new part information:")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    let area = centered_rect(35, 60, f.size());
    let clear = Clear::default();
    f.render_widget(clear, area);
    f.render_widget(popup_block, area);
    // ANCHOR_END: editing_popup

    // ANCHOR: popup_layout
    let popup_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(100/6),
            Constraint::Percentage(100/6),
            Constraint::Percentage(100/6),
            Constraint::Percentage(100/6),
            Constraint::Percentage(100/6),
            Constraint::Percentage(100/6)
        ])
        .split(area);
    let mut part_number_b = Block::default().title("Part Number").borders(Borders::ALL);
    let mut mfg_b = Block::default().title("Manufacturer").borders(Borders::ALL);
    let mut package_b = Block::default().title("Package").borders(Borders::ALL);
    let mut label_b = Block::default().title("Label").borders(Borders::ALL);
    let mut value_b = Block::default().title("Value").borders(Borders::ALL);
    let mut tolerance_b = Block::default().title("Tolerance").borders(Borders::ALL);
    f.render_widget(part_number_b, popup_chunks[0]);
    f.render_widget(mfg_b, popup_chunks[1]);
    f.render_widget(package_b, popup_chunks[2]);
    f.render_widget(label_b, popup_chunks[3]);
    f.render_widget(value_b, popup_chunks[4]);
    f.render_widget(tolerance_b, popup_chunks[5]);
}

fn create_table_rows() -> Vec<Row<'static>> {
    let parts = crate::parts::fetch_part_data();
    let mut rows: Vec<Row> = Vec::new();
    for part in parts {
        let row = Row::new(vec![
            part.part_number,
            part.manufacturer.unwrap_or("".to_string()),
            part.package.unwrap_or("".to_string()),
            part.label.unwrap_or("".to_string()),
            part.value.unwrap_or("".to_string()),
            part.tolerance.unwrap_or("".to_string()),
        ]);
        rows.push(row);
    }
    return rows;
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}