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
    let rows = [Row::new(vec!["Cell1", "Cell2", "Cell3"])];
// Columns widths are constrained in the same way as Layout...
    let widths = [
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
            Row::new(vec!["Name", "Address", "Email"])
                .style(header_style)
                // To add space between the header and the rest of the rows, specify the margin
                .bottom_margin(1),
        )
        // It has an optional footer, which is simply a Row always visible at the bottom.
        .footer(Row::new(vec!["Updated on Dec 28"]))
        // As any other widget, a Table can be wrapped in a Block.
        .block(Block::default().title("Table"))
        // The selected row and its content can also be styled.
        .highlight_style(Style::new().reversed())
        // ...and potentially show a symbol in front of the selection.
        .highlight_symbol(">>");
        //.border_style(Style::new().fg(Color::Cyan))
        //.borders(Borders::ALL);
    f.render_stateful_widget(table, chunks[1], &mut table_state);
    let c = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));
    f.render_widget(c, chunks[2]);
}