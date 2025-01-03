use std::str::FromStr;
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
use ratatui::style::Styled;
use ratatui::text::Text;
use ratatui::widgets::{List, ListDirection, ListState};
use tracing::info;
use crate::app::App;
use crate::projects_view::ProjectSubState;
use crate::ui::centered_rect;

pub fn render_main_panel(f: &mut Frame, app: &App, rect: Rect) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(85),
        ])
        .split(rect);
    render_projects_list_panel(f, app, layout[0]);
    render_project_detail_panel(f, app, layout[1]);
    render_new_project_popup(f, app, rect);
}

fn render_projects_list_panel(f: &mut Frame, app: &App, rect: Rect) {
    let is_select = app.projects_view.sub_state == ProjectSubState::ListMode;
    let b = Block::default()
        .borders(Borders::ALL)
        .border_style(get_block_border_style(is_select))
        .title("Projects")
        .style(get_block_style());
    let items2 = create_project_list(app);
    let list = List::new(items2)
        .block(b)
        .highlight_style(Style::default().fg(Color::LightCyan))
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);
    f.render_stateful_widget(list, rect, &mut app.projects_view.project_list_state.clone());
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
    //TODO: Just use a regular scrollbar state that we reference?
    let mut scrollbar_state =
        ScrollbarState::new(app.projects_view.project_data.len())
            .position(app.projects_view.prj_lst_sbar_state.scroll_position);
    f.render_stateful_widget(
        scrollbar,
        rect.inner(Margin {
            // using an inner vertical margin of 1 unit makes the scrollbar inside the block
            vertical: 1,
            horizontal: 0,
        }),
        &mut scrollbar_state,
    );
}

fn render_project_detail_panel(f: &mut Frame, app: &App, rect: Rect) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(rect);
    let table_rect = layout[0];
    let refreshed_rect = layout[1];
    let b = Block::default()
        .borders(Borders::ALL)
        .border_style(get_block_border_style(app.projects_view.sub_state == ProjectSubState::BOMMode))
        .title("BOM")
        .style(Style::default().fg(Color::White));
    let rows = create_project_table_rows(app);
    let widths = [
        Constraint::Percentage(13),
        Constraint::Percentage(26),
        Constraint::Fill(7),
        Constraint::Percentage(7),
        Constraint::Percentage(7),
        Constraint::Percentage(7),
        Constraint::Percentage(13),
        Constraint::Percentage(13),

    ];
    let header_style = Style::default()
        .fg(tailwind::SLATE.c200)
        .bg(tailwind::BLUE.c900);
    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(tailwind::BLUE.c400);
    let table = Table::new(rows, widths)
        .block(b)
        .column_spacing(1)
        .style(Style::new().blue())
        .header(
            //TODO: Bring this in from parts.rs
            Row::new(vec!["Part Number", "Designator(s)", "Qty", "Value", "Tolerance", "Package", "Label", "MFG"])
                .style(header_style)
                // To add space between the header and the rest of the rows, specify the margin
                .bottom_margin(1),
        )
        // The selected row and its content can also be styled.
        .row_highlight_style(selected_style)
        // ...and potentially show a symbol in front of the selection.
        .highlight_symbol(">>");
    //.border_style(Style::new().fg(Color::Cyan))
    //.borders(Borders::ALL);
    let refreshed_text = Text::from("Refreshed last at 2025-01-01 12:46")
        .style(Style::default().fg(Color::White));
    f.render_stateful_widget(table, table_rect, &mut app.projects_view.bom_table_state.clone());
    f.render_widget(refreshed_text, refreshed_rect);


}

fn create_project_list(app: &App) -> Vec<String> {
    let projects = &app.projects_view.project_data;
    let mut rows: Vec<String> = Vec::new();
    for project in projects {
        rows.push(project.name.clone());
    }
    rows
}

fn get_block_border_style(is_selected: bool) -> Style {
    let mut style = Style::default().fg(Color::from_str("#12B3D6").unwrap());
    if is_selected {
        style = style.add_modifier(Modifier::REVERSED);
    }
    style
}

fn get_block_style() -> Style {
    let retval = Style::default()
        .fg(Color::White);
    retval
}

fn create_project_table_rows(app: &App) -> Vec<Row> {
    let project = &app.projects_view.project_data.get(app.projects_view.selected_project_idx).unwrap();
    let mut rows: Vec<Row> = Vec::new();
    for part in &project.parts {
        let row = Row::new(vec![
            part.partnumber.clone(),
            part.designators.clone(),
            part.qty.to_string(),
            part.part_info.value.clone().unwrap().to_string(),
            part.part_info.tolerance.clone().unwrap().to_string(),
            part.part_info.package.clone().unwrap().to_string(),
            part.part_info.label.clone().unwrap().to_string(),
            part.part_info.manufacturer.clone().unwrap().to_string(),
        ]);
        rows.push(row);
    }
    rows
}

fn render_new_project_popup(f: &mut Frame, app: &App, rect: Rect) {
    if app.projects_view.sub_state != ProjectSubState::CreateNewProject {return};

    let popup_block = Block::default()
        .title("Create new project:")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black).fg(tailwind::EMERALD.c400));

    let area = centered_rect(30, 15, rect);
    let clear = Clear::default();
    f.render_widget(clear, area);
    f.render_widget(popup_block, area);
    let txt_b = Block::default().title("Enter Project Name: ")
        .borders(Borders::ALL)
        .border_style(get_block_border_style(true));
    let txt_t = Paragraph::new(app.projects_view.new_project_name_text.clone()).block(txt_b);
    f.render_widget(txt_t, area);
}

