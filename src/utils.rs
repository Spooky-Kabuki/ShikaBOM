use ratatui::Frame;
use ratatui::layout::{Rect};
use ratatui::widgets::{Block, Clear, ListState};

pub struct ScrollBarInfo {
    pub scroll_position: usize,
    pub scroll_length: u16
}

impl ScrollBarInfo {
    pub fn new() -> Self {
        Self {
            scroll_position: 0,
            scroll_length: 0
        }
    }
    pub fn clear(&mut self) {
        self.scroll_position = 0;
        self.scroll_length = 0;
    }
}

pub fn render_popup_block(f: &mut Frame, area: Rect, popup_block: Block) {
    let clear = Clear::default();
    f.render_widget(clear, area);
    f.render_widget(popup_block, area);
}

pub enum ListMvmtDir {
    Greater,
    Less
}
pub fn exec_list_mvmt(direction: ListMvmtDir, state: &mut ListState, list_len: usize) {
    match direction {
        ListMvmtDir::Greater => {
            match state.selected() {
                Some(selected) => {
                    if selected < list_len - 1 {
                        state.select(Some(selected + 1));
                    }
                }
                None => {
                    if list_len > 0 {
                        state.select(Some(0));
                    }
                }
            }
        }
        ListMvmtDir::Less => {
            match state.selected() {
                Some(selected) => {
                    if selected > 0 && list_len > 0 {
                        state.select(Some(selected - 1));
                    }
                }
                None => {
                    if list_len > 0 {
                        state.select(Some(0));
                    }
                }
            }
        }
    }
}