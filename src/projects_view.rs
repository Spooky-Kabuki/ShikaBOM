use crossterm::event::KeyCode;
use ratatui::widgets::{ListState, TableState};
use crate::{projects, utils};
use crate::projects::{fetch_project_list, Project};
use crate::projects_view::ProjectSubState::{BOMMode, CreateNewProject, ListMode, Main, AddToBOM};
use crate::utils::{ListMvmtDir, ScrollBarInfo};

pub enum ProjectSubState {
    Main,
    ListMode,
    CreateNewProject,
    BOMMode,
    AddToBOM,
}
impl PartialEq for ProjectSubState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Main, Main) => true,
            (ListMode, ListMode) => true,
            (BOMMode, BOMMode) => true,
            (CreateNewProject, CreateNewProject) => true,
            (AddToBOM, AddToBOM) => true,
            _ => false,
        }
    }
}

pub enum ATBFormField {
    PN,
    Designators,
    Qty
}

pub struct AddToBOMFormData {
    pub selected_pn: String,
    pub qty: String,
    pub designators: String,
    pub pns_not_in_project: Vec<String>,
    pub pnip_list_state: ListState,
    pub currently_editing: ATBFormField
}

impl AddToBOMFormData {
    pub fn new() -> AddToBOMFormData {
        Self {
            selected_pn: "".to_string(),
            qty: "".to_string(),
            designators: "".to_string(),
            pns_not_in_project: vec![],
            pnip_list_state: ListState::default(),
            currently_editing: ATBFormField::PN

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
    pub bom_table_state: TableState,
    pub atb_form_data: AddToBOMFormData
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
            bom_table_state: TableState::default(),
            atb_form_data: AddToBOMFormData::new()

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
                if self.selected_project_idx != self.project_list_state.selected().unwrap_or(0) {
                    self.project_list_state.select(Some(self.selected_project_idx))
                }
                self.sub_state = BOMMode;
            }
            KeyCode::Enter => {
                match self.project_list_state.selected() {
                    Some(selected) => {
                        let project = &mut self.project_data[selected];
                        projects::fetch_project_details(project);
                        self.selected_project_idx = selected;
                        self.sub_state = BOMMode;
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
            KeyCode::Char('c') => {
                self.atb_form_data.pns_not_in_project = projects::fetch_pns_not_in_project(
                    &self.project_data[self.selected_project_idx]);
                self.sub_state = AddToBOM;
            }
            KeyCode::Up => {
                let parts_list = &self.project_data[self.selected_project_idx].parts;
                match self.bom_table_state.selected() {
                    Some(selected) => {
                        if selected > 0 && parts_list.len() > 0 {
                            self.bom_table_state.select(Some(selected - 1));
                        }
                    }
                    None => {
                        if parts_list.len() > 0 {
                            self.bom_table_state.select(Some(0));
                        }
                    }
                }
            }
            KeyCode::Down => {
                let parts_list = &self.project_data[self.selected_project_idx].parts;
                match self.bom_table_state.selected() {
                    Some(selected) => {
                        if selected < parts_list.len() - 1 {
                            self.bom_table_state.select(Some(selected + 1));
                        }
                    }
                    None => {
                        if parts_list.len() > 0 {
                            self.bom_table_state.select(Some(0));
                        }
                    }
                }
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
                self.sub_state = BOMMode;
            }
            _ => {}
        }
    }
    pub fn handle_add_to_bom_keys(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.sub_state = BOMMode;
            }
            KeyCode::Tab => {
                match self.atb_form_data.currently_editing {
                    ATBFormField::PN => {
                        self.atb_form_data.currently_editing = ATBFormField::Designators;
                    }
                    ATBFormField::Designators => {
                        self.atb_form_data.currently_editing = ATBFormField::Qty;
                    }
                    ATBFormField::Qty => {
                        self.atb_form_data.currently_editing = ATBFormField::PN;
                    }
                }
            }
            KeyCode::Char(character) => {
                match self.atb_form_data.currently_editing {
                    ATBFormField::PN => {},
                    ATBFormField::Designators => {
                        self.atb_form_data.designators.push(character);
                    },
                    ATBFormField::Qty => {
                        self.atb_form_data.qty.push(character);
                    }
                }
            }
            KeyCode::Backspace => {
                match self.atb_form_data.currently_editing {
                    ATBFormField::PN => {},
                    ATBFormField::Designators => {
                        self.atb_form_data.designators.pop();
                    }
                    ATBFormField::Qty => {
                        self.atb_form_data.qty.pop();
                    }
                }
            }
            KeyCode::Up => {
                match self.atb_form_data.currently_editing {
                    ATBFormField::PN => {
                        let list_len = self.atb_form_data.pns_not_in_project.len();
                        utils::exec_list_mvmt(ListMvmtDir::Less, &mut self.atb_form_data.pnip_list_state, list_len);
                    }
                    //Up arrow can navigate to the field above because why not
                    ATBFormField::Qty => {
                        self.atb_form_data.currently_editing = ATBFormField::Designators;
                    }
                    _ => {}
                }
            }
            KeyCode::Down => {
                match self.atb_form_data.currently_editing {
                    ATBFormField::PN => {
                        let list_len = self.atb_form_data.pns_not_in_project.len();
                        utils::exec_list_mvmt(ListMvmtDir::Greater, &mut self.atb_form_data.pnip_list_state, list_len);
                    }
                    ATBFormField::Qty => {}
                    ATBFormField::Designators => {
                        self.atb_form_data.currently_editing = ATBFormField::Qty;
                    }
                }
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