use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::*,
    Frame,
};
use crate::{parts_ui, parts_view, stock_view, stock_ui};
use crate::app::{App, CurrentScreen};

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

    let header_chunk = chunks[0];
    let content_chunk = chunks[1];
    let footer_chunk = chunks[2];

    let menu_bar_b = Block::default()
        .title("ShikaBOM")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));
    let menu_bar_text = Span::styled("[P]rojects, P[A]rts, [S]torage", Style::default().fg(Color::White));
    let menu_bar_t = Paragraph::new(menu_bar_text)
        .block(menu_bar_b);
    f.render_widget(menu_bar_t, header_chunk);
    //Footer is rendered first, so the other widgets can be rendered on top of it if they choose
    let footer_b = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));
    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::PartScreen => Span::styled(
                "(q) to quit / (n) to make new part / (r) to refresh data / (e) to edit part / (d)etailed view",
                Style::default().fg(Color::Red),
            ),
            _ => Span::styled(
                "(q) to quit / (c) to create stock / (a) to add stock / (s) to spend stock / (e) to edit stock",
                Style::default().fg(Color::Red),
            ),
        }
    };
    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .block(footer_b);
    f.render_widget(key_notes_footer, footer_chunk);

    match app.current_screen {
        CurrentScreen::PartScreen => {
            parts_ui::render_main_parts_panel(f, app, content_chunk);
            match app.parts_view.parts_sub_state {
                parts_view::PartsSubState::Main => {
                    if app.parts_view.show_details {
                        let panel = parts_ui::side_panel_rect(f);
                        parts_ui::render_details_panel(f, app, panel);
                    }
                }
                parts_view::PartsSubState::NewPart => {
                    parts_ui::render_new_part_popup(f, app);
                }
                parts_view::PartsSubState::EditPart => {
                    parts_ui::render_new_part_popup(f, app);
                }
            }
        },
        CurrentScreen::StockScreen => {
            stock_ui::render_main_stock_panel(f, app, content_chunk);
            match app.stock_view.stock_sub_state {
                stock_view::StockSubState::StockMain => {
                    //TODO: Render a detail panel???
                },
                stock_view::StockSubState::CreateStock => {
                    stock_ui::render_create_stock_popup(f, app);
                }

                _ => {}
            }
        }
        _ => {}
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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