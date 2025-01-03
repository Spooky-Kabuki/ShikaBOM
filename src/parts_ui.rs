use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    prelude::{Color, Line, Modifier, Span, Style, Stylize},
    style::palette::tailwind,
    widgets::{
        Block, Borders, Clear, Paragraph, Row, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Table, Wrap
    }
};
use crate::app::App;
use crate::parts_view;
//TODO: this should go into like a utils file or something
use crate::ui::centered_rect;

pub fn render_main_parts_panel(f: &mut Frame, app: &App, rect: Rect) {
    let rows = create_parts_table_rows(&app);
// Columns widths are constrained in the same way as Layout...
    let widths = [
        Constraint::Length(20),
        Constraint::Length(11),
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
            Row::new(vec!["Part Number", "Total Qty", "Manufacturer", "Package", "Label", "Value", "Tolerance"])
                .style(header_style)
                // To add space between the header and the rest of the rows, specify the margin
                .bottom_margin(1),
        )
        // It has an optional footer, which is simply a Row always visible at the bottom.
        .footer(Row::new(vec!["Refreshed last at: 2021-09-01 12:34:56"]))
        // As any other widget, a Table can be wrapped in a Block.
        .block(Block::default().title("Table"))
        // The selected row and its content can also be styled.
        .row_highlight_style(selected_style)
        // ...and potentially show a symbol in front of the selection.
        .highlight_symbol(">>");
    //.border_style(Style::new().fg(Color::Cyan))
    //.borders(Borders::ALL);
    f.render_stateful_widget(table, rect, &mut app.parts_view.part_table_state.clone());
}

//TODO: This panel is missing the label field
pub fn render_details_panel(f: &mut Frame, app: &App, panel: Rect) {
    let clear = Clear::default();
    let parent_block = Block::default()
        .title("Details")
        .borders(Borders::ALL);
    f.render_widget(clear, panel);
    f.render_widget(parent_block, panel);
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    let panel_layout = centered_rect(95, 95, panel);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(panel_layout);
    let header = layout[0];
    let content = layout[1];
    let table_size = app.parts_view.part_storage_data.len() + 2 + 2;
    let header_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(4),
            Constraint::Fill(1),
            Constraint::Min(table_size as u16)
        ])
        .split(header);
    //TODO: figure out how to render only a box of the content.

    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3)
        ])
        .split(content);
    //These are fixed bits of info
    let pn_b = Block::default().title("Part Number:").borders(Borders::TOP | Borders::BOTTOM);
    let pn_t = Paragraph::new(app.parts_view.part_text.part_number.clone()).block(pn_b);
    f.render_widget(pn_t, header_chunks[0]);
    let desc_b = Block::default().title("Description:").borders(Borders::TOP | Borders::BOTTOM);
    let desc_t = Paragraph::new(app.parts_view.part_text.description.clone()).block(desc_b).wrap(Wrap { trim: true });
    f.render_widget(desc_t, header_chunks[1]);
    let storage_b = Block::default().title("Storage").borders(Borders::TOP | Borders::BOTTOM);
    f.render_widget(create_part_storage_table(app, storage_b), header_chunks[2]);

    //Dynamically render these
    let total_qty_b = Block::default().title("Total Quantity:").borders(Borders::TOP | Borders::BOTTOM);
    let total_qty_t = Paragraph::new(app.parts_view.part_text.total_qty.clone()).block(total_qty_b);

    let mfg_b = Block::default().title("Manufacturer:").borders(Borders::TOP | Borders::BOTTOM);
    let mfg_t = Paragraph::new(app.parts_view.part_text.manufacturer.clone()).block(mfg_b);

    let pkg_b = Block::default().title("Package:").borders(Borders::TOP | Borders::BOTTOM);
    let pkg_t = Paragraph::new(app.parts_view.part_text.package.clone()).block(pkg_b);

    let val_b = Block::default().title("Value:").borders(Borders::TOP | Borders::BOTTOM);
    let val_t = Paragraph::new(app.parts_view.part_text.value.clone()).block(val_b);

    let tol_b = Block::default().title("Tolerance:").borders(Borders::TOP | Borders::BOTTOM);
    let tol_t = Paragraph::new(content_chunks.len().to_string()).block(tol_b);


    let width = content_chunks[0].width;
    let mut real_height = 0;
    //TODO: Use this to calculate the starting index for the panel components
    //let starting_idx = 0;
    let panel_component_vec: Vec<Paragraph> = vec![total_qty_t, mfg_t, pkg_t, val_t, tol_t];
    for item in &panel_component_vec {
        real_height += item.line_count(width);
    }
    for i in 0..content_chunks.len() {
        if i < panel_component_vec.len() {
            f.render_widget(panel_component_vec[i].clone(), content_chunks[i]);
        }
    }
    let mut scrollbar_state = ScrollbarState::new(real_height).position(app.parts_view.part_scroll_info.scroll_position);

    f.render_stateful_widget(
        scrollbar,
        panel.inner(Margin {
            // using an inner vertical margin of 1 unit makes the scrollbar inside the block
            vertical: 1,
            horizontal: 0,
        }),
        &mut scrollbar_state,
    );
}



pub fn render_new_part_popup(f: &mut Frame, app: &App) {
    let highlighted_style = Style::default().fg(Color::White).bg(Color::Blue);

    let disabled_style = Borders::NONE;

    let popup_block = Block::default()
        .title("Enter new part information:")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    let area = centered_rect(35, 60, f.area());
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

    if app.parts_view.parts_sub_state == parts_view::PartsSubState::EditPart {
        pn_b = pn_b.borders(disabled_style);
    }

    match app.parts_view.currently_editing_part {
        parts_view::CurrentlyEditingPart::PartNumber => {
            pn_b = pn_b.style(highlighted_style);
        }
        parts_view::CurrentlyEditingPart::Manufacturer => {
            mfg_b = mfg_b.style(highlighted_style);
        }
        parts_view::CurrentlyEditingPart::Package => {
            pkg_b = pkg_b.style(highlighted_style);
        }
        parts_view::CurrentlyEditingPart::Label => {
            lbl_b = lbl_b.style(highlighted_style);
        }
        parts_view::CurrentlyEditingPart::Value => {
            val_b = val_b.style(highlighted_style);
        }
        parts_view::CurrentlyEditingPart::Tolerance => {
            tol_b = tol_b.style(highlighted_style);
        }
    }
    let pn_t = Paragraph::new(app.parts_view.part_text.part_number.clone()).block(pn_b);
    let mfg_t = Paragraph::new(app.parts_view.part_text.manufacturer.clone()).block(mfg_b);
    let pkg_t = Paragraph::new(app.parts_view.part_text.package.clone()).block(pkg_b);
    let lbl_t = Paragraph::new(app.parts_view.part_text.label.clone()).block(lbl_b);
    let val_t = Paragraph::new(app.parts_view.part_text.value.clone()).block(val_b);
    let tol_t = Paragraph::new(app.parts_view.part_text.tolerance.clone()).block(tol_b);

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

pub(crate) fn side_panel_rect(f: &mut Frame) -> Rect {
    let layouts = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ])
        .split(f.area());
    return layouts[1]
}

fn create_part_storage_table<'a>(app: &App, block: Block<'a>) -> Table<'a> {
    //Storage table
    let rows = create_storage_table_rows(&app);
// Columns widths are constrained in the same way as Layout...
    let widths = [
        Constraint::Percentage(50),
        Constraint::Percentage(50)
    ];
    let header_style = Style::default()
        .fg(tailwind::SLATE.c200)
        .bg(tailwind::BLUE.c900);
    let storage_ta = Table::new(rows, widths)
        .column_spacing(1)
        .style(header_style)
        .header(
            Row::new(vec!["Location", "Qty"])
                .style(header_style)
                // To add space between the header and the rest of the rows, specify the margin
                .bottom_margin(1),
        )
        .block(block);
    return storage_ta;
}

fn create_storage_table_rows(app: &App) -> Vec<Row<'static>> {
    let part_data = &app.parts_view.part_storage_data;
    let mut rows: Vec<Row> = Vec::new();
    for part in part_data {
        let row = Row::new(vec![
            part.location.clone(),
            part.quantity.to_string(),
        ]);
        rows.push(row);
    }
    return rows;
}

pub fn create_parts_table_rows(app: &App) -> Vec<Row<'static>> {
    let part_data = &app.parts_view.part_data;
    let mut rows: Vec<Row> = Vec::new();
    for part in part_data {
        let row = Row::new(vec![
            part.part_number.clone(),
            part.total_qty.unwrap_or(0).to_string(),
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