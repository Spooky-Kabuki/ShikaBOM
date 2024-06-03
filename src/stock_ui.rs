use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    prelude::{Color, Line, Modifier, Span, Style, Stylize},
    style::palette::tailwind,
    widgets::{
        Block, Borders, Clear, Paragraph, Row, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Table, Wrap,
        List, ListDirection
    }
};
use crate::app::App;
//TODO: this should go into like a utils file or something
use crate::ui::centered_rect;

pub fn render_main_stock_panel(f: &mut Frame, app: &App, rect: Rect) {
    let rows = create_stock_table_rows(&app);
// Columns widths are constrained in the same way as Layout...
    let widths = [
        Constraint::Length(20),
        Constraint::Length(15),
        Constraint::Length(10),
        Constraint::Length(15),
        Constraint::Length(15),
        Constraint::Length(15),
        Constraint::Length(25),
        Constraint::Length(15)

    ];
    let header_style = Style::default()
        .fg(tailwind::SLATE.c200)
        .bg(tailwind::EMERALD.c900);
    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(tailwind::EMERALD.c400);
    let table = Table::new(rows, widths)
        .column_spacing(1)
        .style(Style::new().bg(Color::Black).fg(tailwind::EMERALD.c400))
        .header(
            //TODO: Bring this in from parts.rs
            Row::new(vec!["Part Number", "Total Stock", "On Hand", "Available", "In Production", "Balance", "Low Stock Threshold", "On Order"])
                .style(header_style)
                // To add space between the header and the rest of the rows, specify the margin
                .bottom_margin(1),
        )
        // It has an optional footer, which is simply a Row always visible at the bottom.
        .footer(Row::new(vec!["Only stocked parts are shown."]))
        // As any other widget, a Table can be wrapped in a Block.
        .block(Block::default().title("Stock Table"))
        // The selected row and its content can also be styled.
        .highlight_style(selected_style)
        // ...and potentially show a symbol in front of the selection.
        .highlight_symbol(">>");
    f.render_stateful_widget(table, rect, & mut app.stock_view.stock_table_state.clone());
}

pub fn create_stock_table_rows(app: &App) -> Vec<Row> {
    let mut rows = Vec::new();
    for stock in &app.stock_view.stock_data {
        rows.push(Row::new(vec![
            stock.partnumber.to_string(),
            stock.total_stock.to_string(),
            stock.on_hand.to_string(),
            stock.available.to_string(),
            stock.in_prod.to_string(),
            stock.balance.to_string(),
            stock.low_stock_threshold.to_string(),
            stock.on_order.to_string(),
        ]));
    }
    rows

}
pub fn render_create_stock_popup(f: &mut Frame, app: &App) {
    let highlighted_style = Style::default()
        .fg(tailwind::SLATE.c200)
        .bg(tailwind::EMERALD.c900);
    let selected_style = Style::default()
        //.add_modifier(Modifier::REVERSED)
        .fg(tailwind::EMERALD.c400)
        .bg(tailwind::SLATE.c200);

    let popup_block = Block::default()
        .title("Create stock for an existing part:")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black).fg(tailwind::EMERALD.c400));

    let area = centered_rect(60, 35, f.size());
    let clear = Clear::default();
    f.render_widget(clear, area);
    f.render_widget(popup_block, area);

    let hori_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ])
        .split(area);
    let list_chunk = hori_chunks[0];
    let form_chunk = hori_chunks[1];

    let items = app.stock_view.nonstocked_pns.clone();
    let list = List::new(items)
        .block(Block::bordered().title("Part Number").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(highlighted_style.add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    f.render_stateful_widget(list, list_chunk, & mut app.stock_view.nonstocked_pn_list_state.clone());
}