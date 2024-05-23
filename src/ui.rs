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
use crate::app;

use crate::parts::Part;


use crate::app::{App, CurrentScreen};

// ANCHOR: method_sig
pub fn ui(f: &mut Frame, app: &App) {
    // TODO: this is just the parts view
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
    let rows = create_table_rows(&app);
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
    f.render_stateful_widget(table, chunks[1], &mut app.part_table_state.clone());
    let c = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));
    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::PartScreen => Span::styled(
                "(q) to quit / (n) to make new part / (r) to refresh data / (e) to edit part",
                Style::default().fg(Color::Red),
            ),
        }
    };
    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .block(c);
    f.render_widget(key_notes_footer, chunks[2]);

    match app.current_screen {
        CurrentScreen::PartScreen => {
            match app.parts_sub_state {
                app::PartsSubState::NewPart => {
                    render_new_part_popup(f, app);
                }
                app::PartsSubState::EditPart => {
                    render_new_part_popup(f, app);
                }
                _ => {}
            }
        },
        _ => {}
    }


}


fn render_new_part_popup(f: &mut Frame, app: &App) {
    let highlighted_style = Style::default().fg(Color::White).bg(Color::Blue);

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
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(4),
        ])
        .split(area);
    let mut pn_b = Block::default().title("Part Number").borders(Borders::ALL);
    let mut mfg_b = Block::default().title("Manufacturer").borders(Borders::ALL);
    let mut pkg_b = Block::default().title("Package").borders(Borders::ALL);
    let mut lbl_b = Block::default().title("Label").borders(Borders::ALL);
    let mut val_b = Block::default().title("Value").borders(Borders::ALL);
    let mut tol_b = Block::default().title("Tolerance").borders(Borders::ALL);

    match app.currently_editing_part {
        crate::app::CurrentlyEditingPart::PartNumber => {
            pn_b = pn_b.style(highlighted_style);
        }
        crate::app::CurrentlyEditingPart::Manufacturer => {
            mfg_b = mfg_b.style(highlighted_style);
        }
        crate::app::CurrentlyEditingPart::Package => {
            pkg_b = pkg_b.style(highlighted_style);
        }
        crate::app::CurrentlyEditingPart::Label => {
            lbl_b = lbl_b.style(highlighted_style);
        }
        crate::app::CurrentlyEditingPart::Value => {
            val_b = val_b.style(highlighted_style);
        }
        crate::app::CurrentlyEditingPart::Tolerance => {
            tol_b = tol_b.style(highlighted_style);
        }
    }
    let pn_t = Paragraph::new(app.part_text.part_number.clone()).block(pn_b);
    let mfg_t = Paragraph::new(app.part_text.manufacturer.clone()).block(mfg_b);
    let pkg_t = Paragraph::new(app.part_text.package.clone()).block(pkg_b);
    let lbl_t = Paragraph::new(app.part_text.label.clone()).block(lbl_b);
    let val_t = Paragraph::new(app.part_text.value.clone()).block(val_b);
    let tol_t = Paragraph::new(app.part_text.tolerance.clone()).block(tol_b);

    f.render_widget(pn_t, popup_chunks[0]);
    f.render_widget(mfg_t, popup_chunks[1]);
    f.render_widget(pkg_t, popup_chunks[2]);
    f.render_widget(lbl_t, popup_chunks[3]);
    f.render_widget(val_t, popup_chunks[4]);
    f.render_widget(tol_t, popup_chunks[5]);
    let footer_text =  Span::styled("<ESC> to exit, <ENTER> to save", Style::default().fg(Color::Red));
    let foot = Paragraph::new(Line::from(footer_text))
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(foot, popup_chunks[6]);
}

fn create_table_rows(app: &App) -> Vec<Row<'static>> {
    let parts = &app.part_data;
    let mut rows: Vec<Row> = Vec::new();
    for part in parts {
        let row = Row::new(vec![
            part.part_number.clone(),
            part.manufacturer.clone().unwrap_or("".to_string()),
            part.package.clone().unwrap_or("".to_string()),
            part.label.clone().unwrap_or("".to_string()),
            part.value.clone().unwrap_or("".to_string()),
            part.tolerance.clone().unwrap_or("".to_string()),
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