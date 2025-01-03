use crossterm::event::KeyCode;
use ratatui::widgets::{ListState};
use crate::projects;
use crate::projects::{fetch_project_list, Project};
use crate::projects_view::ProjectSubState::{BOMMode, CreateNewProject, ListMode, Main};
use crate::utils::ScrollBarInfo;

pub enum ProjectSubState {
    Main,
    ListMode,
    CreateNewProject,
    BOMMode,
}
impl PartialEq for ProjectSubState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Main, Main) => true,
            (ListMode, ListMode) => true,
            (BOMMode, BOMMode) => true,
            (CreateNewProject, CreateNewProject) => true,
            _ => false,
        }
    }
}

pub struct ProjectsView {
    pub sub_state: ProjectSubState,
    pub project_data: Vec<Project>,
    pub project_list_state: ListState,
    pub selected_project_idx: usize,
    pub new_project_name_text: String,
    pub prj_lst_sbar_state: ScrollBarInfo,

}

impl ProjectsView {
    pub fn new() -> Self {
        Self {
            sub_state: Main,
            project_data: fetch_project_list(),
            project_list_state: ListState::default(),
            selected_project_idx: 0,
            new_project_name_text: String::from(""),
            prj_lst_sbar_state: ScrollBarInfo::new(),
        }
    }

    pub fn refresh_list(&mut self) {
        self.project_data = fetch_project_list();
    }

    pub fn handle_main_keys(&mut self, key: KeyCode) {
        match key {
            KeyCode::Tab => {
                self.sub_state = ListMode;
            }
            _ => {}
        }
    }

    pub fn handle_list_mode_keys(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.sub_state = Main;
            }
            KeyCode::Tab => {
                self.sub_state = BOMMode;
            }
            KeyCode::Enter => {
                match self.project_list_state.selected() {
                    Some(selected) => {
                        let project = &mut self.project_data[selected];
                        projects::fetch_project_details(project);
                        self.selected_project_idx = selected;
                    }
                    None => {}
                }
            }
            KeyCode::Down => {
                match self.project_list_state.selected() {
                    Some(selected) => {
                        if selected < self.project_data.len() - 1 {
                            self.project_list_state.select(Some(selected + 1));
                        }
                    }
                    None => {
                        self.project_list_state.select(Some(0));
                    }
                }
                self.prj_lst_sbar_state.scroll_position += 1
            }
            KeyCode::Up => {
                match self.project_list_state.selected() {
                    Some(selected) => {
                        if selected > 0 {
                            self.project_list_state.select(Some(selected - 1));
                        }
                        else { self.project_list_state.select(Some(0)); }
                    }
                    None => {
                        self.project_list_state.select(Some(0));
                    }
                }
                if self.prj_lst_sbar_state.scroll_position > 0 {
                    self.prj_lst_sbar_state.scroll_position -= 1;
                }
            }
            KeyCode::Char('r') => {
                self.refresh_list()
            }
            KeyCode::Char('c') => {
                //Create new project
                self.sub_state = CreateNewProject;
            }
            KeyCode::Char('d') => {
                //Delete project??
            }
            _ => {}
        }
    }
    pub fn handle_bom_mode_keys(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.sub_state = Main;
            }
            KeyCode::Tab => {
                self.sub_state = ListMode;
            }
            _ => {}
        }
    }
    pub fn handle_create_project_keys(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.sub_state = ListMode;
            }
            KeyCode::Char(character) => {
                self.new_project_name_text.push(character);
            }
            KeyCode::Backspace => {
                self.new_project_name_text.pop();
            }
            KeyCode::Enter => {
                projects::create_new_project_name(self.new_project_name_text.clone());
                self.new_project_name_text.clear();
                self.refresh_list();
                self.select_last_idx();
                self.sub_state = ListMode;
            }
            _ => {}
        }
    }

    fn select_last_idx(&mut self) {
        if self.project_data.len() == 0 {return};

        let idx = self.project_data.len() - 1;
        self.project_list_state.select(Some(idx));
        let project = &mut self.project_data[idx];
        projects::fetch_project_details(project);
        self.selected_project_idx = idx;
    }
}